use crate::auth::account::{Account, AccountType};
use crate::auth::session::{MicrosoftSession, Session};
use minecraft_msa_auth::MinecraftAuthorizationFlow;
use oauth2::basic::BasicClient;
use oauth2::reqwest::{Client, Url};
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

pub(crate) struct MicrosoftAuth {
    pub client_id: String,
    pub redirect_uri: String,
}

impl MicrosoftAuth {
    pub(crate) fn new() -> Self {
        Self {
            client_id: "7dfd8c26-1e48-4695-9081-5f7dbbebe0cb".to_string(),
            redirect_uri: "http://127.0.0.1:6767/callback".to_string(),
        }
    }

    pub(crate) async fn login_interactive(&self) -> Option<Account> {
        let token_uri = TokenUrl::new(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/token".to_string(),
        )
        .ok()?;
        let auth_uri = AuthUrl::new(
            "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize".to_string(),
        )
        .ok()?;
        let redirect_uri = RedirectUrl::new(self.redirect_uri.clone()).ok()?;

        let client = BasicClient::new(ClientId::new(self.client_id.clone()))
            .set_token_uri(token_uri)
            .set_auth_uri(auth_uri)
            .set_auth_type(AuthType::RequestBody)
            .set_redirect_uri(redirect_uri);

        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorize_url, csrf_state) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("XboxLive.signin offline_access".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        let listener = TcpListener::bind("127.0.0.1:6767").await.ok()?;
        open::that(authorize_url.to_string()).ok()?;

        loop {
            let (stream, _) = listener.accept().await.ok()?;
            stream.readable().await.ok()?;
            let mut stream = BufReader::new(stream);

            let code;
            let state;
            {
                let mut request_line = String::new();
                stream.read_line(&mut request_line).await.ok()?;

                let redirect_url = request_line.split_whitespace().nth(1)?;
                let url = Url::parse(&("http://localhost:6767".to_string() + redirect_url)).ok()?;

                let (_, value) = url.query_pairs().find(|(key, _)| key == "code")?;
                code = AuthorizationCode::new(value.into_owned());

                let (_, value) = url.query_pairs().find(|(key, _)| key == "state")?;
                state = CsrfToken::new(value.into_owned());
            }

            let message = r#"<!doctype html>
                <html lang="en">
                <head>
                  <meta charset="utf-8" />
                  <meta name="viewport" content="width=device-width, initial-scale=1" />
                  <title>Authentication complete</title>
                  <style>
                    body { font-family: -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif; margin: 2rem; }
                  </style>
                </head>
                <body>
                  <h2>Authentication complete</h2>
                  <p>You can close this tab and return to the launcher.</p>
                </body>
                </html>"#;
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=utf-8\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).await.ok()?;
            stream.flush().await.ok()?;
            stream.get_mut().shutdown().await.ok()?;

            if state.secret() != csrf_state.secret() {
                return None;
            }

            let token = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_code_verifier)
                .request_async(&Client::new())
                .await
                .ok()?;

            let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
            let mc_token = mc_flow
                .exchange_microsoft_token(token.access_token().secret())
                .await
                .ok()?;

            let profile = reqwest::Client::new()
                .get("https://api.minecraftservices.com/minecraft/profile")
                .bearer_auth(mc_token.access_token().as_ref())
                .send()
                .await
                .ok()?
                .error_for_status()
                .ok()?
                .json::<Value>()
                .await
                .ok()?;

            let profile_id = profile.get("id")?.as_str()?.to_string();
            let refresh_token = token.refresh_token()?.secret().to_string();
            let expires_at = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs() as i64
                + i64::from(mc_token.expires_in());

            return Some(Account {
                id: profile_id.clone(),
                username: mc_token.username().to_string(),
                account_type: AccountType::Microsoft,
                session: Session::Microsoft(MicrosoftSession {
                    access_token: mc_token.access_token().as_ref().to_string(),
                    refresh_token,
                    uuid: profile_id,
                    expires_at,
                }),
            });
        }
    }
}
