use crate::auth::account::{Account, AccountType};
use crate::auth::session::{MicrosoftSession, Session};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde_json::Value;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

type AuthResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub(crate) struct MicrosoftAuth {
    pub client_id: String,
    pub redirect_uri: String,
}

pub(crate) struct MicrosoftLoginRequest {
    pub auth_url: String,
    pub state: String,
    pub pkce_verifier: String,
}

impl MicrosoftAuth {
    pub(crate) fn new() -> Self {
        Self {
            client_id: "7dfd8c26-1e48-4695-9081-5f7dbbebe0cb".to_string(),
            redirect_uri: "http://127.0.0.1:6767/callback".to_string(),
        }
    }

    pub(crate) fn authorization_request(&self) -> MicrosoftLoginRequest {
        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_auth_uri(AuthUrl::new(
                "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize".to_string(),
            )
                .unwrap())
            .set_token_uri(TokenUrl::new(
                "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string(),
            )
                .unwrap())
            .set_redirect_uri(RedirectUrl::new(self.redirect_uri.clone()).unwrap());

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("xboxlive.signin".to_string()))
            .add_scope(Scope::new("xboxlive.offline_access".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        MicrosoftLoginRequest {
            auth_url: auth_url.to_string(),
            state: state.secret().to_string(),
            pkce_verifier: pkce_verifier.secret().to_string(),
        }
    }

    pub(crate) async fn finish_login(&self, code: String, pkce_verifier: String) -> AuthResult<Account> {
        fn missing(field: &str) -> std::io::Error {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("missing field in auth response: {field}"),
            )
        }

        // 1) Exchange Microsoft OAuth code for tokens.
        let oauth_client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_auth_uri(AuthUrl::new(
                "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize".to_string(),
            )
                .unwrap())
            .set_token_uri(TokenUrl::new(
                "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string(),
            )
                .unwrap())
            .set_redirect_uri(RedirectUrl::new(self.redirect_uri.clone()).unwrap());

        let oauth_http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let microsoft_token = oauth_client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
            .request_async(&oauth_http)
            .await?;

        // 2) Authenticate to Xbox Live.
        let xbl_body = serde_json::json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={}", microsoft_token.access_token().secret()),
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT",
        });

        let xbl = reqwest::Client::new()
            .post("https://user.auth.xboxlive.com/user/authenticate")
            .json(&xbl_body)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;
        let xbl_token = xbl["Token"]
            .as_str()
            .ok_or_else(|| missing("Token"))?
            .to_string();

        // 3) Exchange Xbox token for XSTS token.
        let xsts_body = serde_json::json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [xbl_token],
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT",
        });

        let xsts_response = reqwest::Client::new()
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&xsts_body)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

        let uhs = xsts_response["DisplayClaims"]["xui"][0]["uhs"]
            .as_str()
            .ok_or_else(|| missing("DisplayClaims.xui[0].uhs"))?
            .to_string();
        let xsts_token = xsts_response["Token"]
            .as_str()
            .ok_or_else(|| missing("Token"))?
            .to_string();

        // 4) Authenticate to Minecraft services.
        let mc_body = serde_json::json!({
            "identityToken": format!("XBL3.0 x={uhs};{xsts_token}")
        });

        let mc_response = reqwest::Client::new()
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&mc_body)
            .send()
            .await?;

        let mc_status = mc_response.status();
        let mc_body_text = mc_response.text().await?;
        if !mc_status.is_success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "minecraft login_with_xbox failed: status={mc_status}, body={mc_body_text}"
                ),
            )
                .into());
        }

        let mc: Value = serde_json::from_str(&mc_body_text)?;
        let mc_access_token = mc["access_token"]
            .as_str()
            .ok_or_else(|| missing("access_token"))?
            .to_string();
        let mc_expires_in = mc["expires_in"]
            .as_i64()
            .ok_or_else(|| missing("expires_in"))?;

        // 5) Fetch Minecraft profile.
        let profile = reqwest::Client::new()
            .get("https://api.minecraftservices.com/minecraft/profile")
            .bearer_auth(&mc_access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

        let profile_id = profile["id"]
            .as_str()
            .ok_or_else(|| missing("id"))?
            .to_string();
        let profile_name = profile["name"]
            .as_str()
            .ok_or_else(|| missing("name"))?
            .to_string();

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64
            + mc_expires_in;

        let refresh_token = microsoft_token
            .refresh_token()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "missing refresh token"))?
            .secret()
            .to_string();

        Ok(Account {
            id: profile_id.clone(),
            username: profile_name,
            account_type: AccountType::Microsoft,
            session: Session::Microsoft(MicrosoftSession {
                access_token: mc_access_token,
                refresh_token,
                uuid: profile_id,
                expires_at,
            }),
        })
    }
}
