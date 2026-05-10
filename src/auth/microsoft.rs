use crate::auth::types::{Account, AccountType, MicrosoftSession, Session};
use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, EmptyExtraTokenFields, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, StandardTokenResponse, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

type BasicTokenResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;
type AuthResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct MicrosoftAuth {
    pub client_id: String,
    pub redirect_uri: String,
}

pub struct MicrosoftLoginRequest {
    pub auth_url: String,
    pub state: String,
    pub pkce_verifier: String,
}

#[derive(Debug, Deserialize)]
struct XboxTokenResponse {
    #[serde(rename = "Token")]
    token: String,
    #[serde(rename = "DisplayClaims")]
    display_claims: XboxDisplayClaims,
}

#[derive(Debug, Deserialize)]
struct XboxDisplayClaims {
    xui: Vec<XuiEntry>,
}

#[derive(Debug, Deserialize)]
struct XuiEntry {
    uhs: String,
}

#[derive(Debug, Deserialize)]
struct MinecraftLoginResponse {
    access_token: String,
    expires_in: i64,
    token_type: String,
}

#[derive(Debug, Deserialize)]
struct MinecraftProfile {
    id: String,
    name: String,
}

struct XstsResult {
    token: String,
    uhs: String,
}

impl MicrosoftAuth {
    pub fn new() -> Self {
        Self {
            client_id: "7dfd8c26-1e48-4695-9081-5f7dbbebe0cb".to_string(),
            redirect_uri: "http://127.0.0.1:6767/callback".to_string(),
        }
    }

    pub fn authorization_request(&self) -> MicrosoftLoginRequest {
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

    pub async fn finish_login(&self, code: String, pkce_verifier: String) -> AuthResult<Account> {
        let microsoft_token = self.exchange_code(code, pkce_verifier).await?;
        let xbl = self
            .authenticate_xbox(microsoft_token.access_token().secret())
            .await?;
        let xsts = self.authenticate_xsts(&xbl.token).await?;
        let mc = self.authenticate_minecraft(&xsts.uhs, &xsts.token).await?;
        let profile = self.fetch_profile(&mc.access_token).await?;

        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs() as i64
            + mc.expires_in;

        let refresh_token = microsoft_token
            .refresh_token()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "missing refresh token"))?
            .secret()
            .to_string();

        Ok(Account {
            id: profile.id.clone(),
            username: profile.name,
            account_type: AccountType::Microsoft,
            session: Session::Microsoft(MicrosoftSession {
                access_token: mc.access_token,
                refresh_token,
                uuid: profile.id,
                expires_at,
            }),
        })
    }

    async fn exchange_code(&self, code: String, pkce_verifier: String) -> AuthResult<BasicTokenResponse> {
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

        let http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let token = client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
            .request_async(&http)
            .await?;

        Ok(token)
    }

    async fn authenticate_xbox(&self, microsoft_access_token: &str) -> AuthResult<XboxTokenResponse> {
        let body = serde_json::json!({
              "Properties": {
                  "AuthMethod": "RPS",
                  "SiteName": "user.auth.xboxlive.com",
                  "RpsTicket": format!("d={microsoft_access_token}"),
              },
              "RelyingParty": "http://auth.xboxlive.com",
              "TokenType": "JWT",
          });

        let response = reqwest::Client::new()
            .post("https://user.auth.xboxlive.com/user/authenticate")
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<XboxTokenResponse>()
            .await?;

        Ok(response)
    }

    async fn authenticate_xsts(&self, xbl_token: &str) -> AuthResult<XstsResult> {
        let body = serde_json::json!({
              "Properties": {
                  "SandboxId": "RETAIL",
                  "UserTokens": [xbl_token],
              },
              "RelyingParty": "rp://api.minecraftservices.com/",
              "TokenType": "JWT",
          });

        let response = reqwest::Client::new()
            .post("https://xsts.auth.xboxlive.com/xsts/authorize")
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<XboxTokenResponse>()
            .await?;

        let uhs = response
            .display_claims
            .xui
            .get(0)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "missing uhs"))?
            .uhs
            .clone();

        Ok(XstsResult {
            token: response.token,
            uhs,
        })
    }

    async fn authenticate_minecraft(
        &self,
        uhs: &str,
        xsts_token: &str,
    ) -> AuthResult<MinecraftLoginResponse> {
        let body = serde_json::json!({
              "identityToken": format!("XBL3.0 x={uhs};{xsts_token}")
          });

        let response = reqwest::Client::new()
            .post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let response_body = response.text().await?;

        if status.is_success() {
            let parsed = serde_json::from_str::<MinecraftLoginResponse>(&response_body)?;
            return Ok(parsed);
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "minecraft login_with_xbox failed: status={status}, body={response_body}"
            ),
        )
        .into())
    }

    async fn fetch_profile(&self, minecraft_access_token: &str) -> AuthResult<MinecraftProfile> {
        let response = reqwest::Client::new()
            .get("https://api.minecraftservices.com/minecraft/profile")
            .bearer_auth(minecraft_access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<MinecraftProfile>()
            .await?;

        Ok(response)
    }
}
