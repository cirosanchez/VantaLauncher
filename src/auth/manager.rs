use crate::auth::session::microsoft::{MicrosoftAuth, MicrosoftLoginRequest};
use crate::auth::session::offline::create_offline_account;
use crate::auth::storage::{AuthData, AuthStorage};
use crate::auth::Account;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http::StatusCode, Router};
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;
use slint::ComponentHandle;
use tokio::net::TcpListener;
use tokio::sync::{oneshot, Mutex};

type AuthResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

pub struct AccountManager {
    microsoft: MicrosoftAuth,
    data: Mutex<AuthData>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            microsoft: MicrosoftAuth::new(),
            data: Mutex::new(AuthStorage::load()),
        }
    }

    pub async fn add_account(&self, account: Account) {
        let mut data = self.data.lock().await;

        // If account already exists, update it, otherwise add it
        if let Some(existing) = data.accounts.iter_mut().find(|a| a.id == account.id) {
            *existing = account.clone();
        } else {
            data.accounts.push(account.clone());
        }

        data.active_account_id = Some(account.id);
        AuthStorage::save(&data);
    }

    pub async fn get_active_account(&self) -> Option<Account> {
        let data = self.data.lock().await;
        let id = data.active_account_id.as_ref()?;
        data.accounts.iter().find(|a| &a.id == id).cloned()
    }

    pub async fn login_offline(&self, username: &str) -> Account {
        let account = create_offline_account(username);
        self.add_account(account.clone()).await;
        account
    }

    pub fn begin_microsoft_login(&self) -> MicrosoftLoginRequest {
        self.microsoft.authorization_request()
    }

    pub async fn login_microsoft_interactive(&self) -> AuthResult<Account> {
        let request = self.begin_microsoft_login();
        let callback = self.wait_for_microsoft_callback(&request).await?;
        let account = self
            .microsoft
            .finish_login(callback.code, request.pkce_verifier)
            .await?;
        self.add_account(account.clone()).await;
        Ok(account)
    }

    pub async fn ensure_authenticated_with_login_window(
        self: Arc<Self>,
        login_ui: crate::LoginWindow,
    ) -> Result<bool, slint::PlatformError> {
        if self.get_active_account().await.is_some() {
            return Ok(true);
        }

        let am_clone = self.clone();
        let handle = login_ui.as_weak();
        login_ui.on_login_microsoft(move || {
            let am = am_clone.clone();
            let h = handle.clone();
            tokio::spawn(async move {
                match am.login_microsoft_interactive().await {
                    Ok(_) => {
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(window) = h.upgrade() {
                                let _ = window.hide();
                            }
                        });
                    }
                    Err(err) => {
                        eprintln!("microsoft login failed: {err}");
                    }
                }
            });
        });

        let am_clone = self.clone();
        let handle = login_ui.as_weak();
        login_ui.on_login_offline(move |username| {
            let am = am_clone.clone();
            let h = handle.clone();
            let username = username.to_string();
            tokio::spawn(async move {
                am.login_offline(&username).await;
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(window) = h.upgrade() {
                        let _ = window.hide();
                    }
                });
            });
        });

        login_ui.run()?;
        Ok(self.get_active_account().await.is_some())
    }

    async fn wait_for_microsoft_callback(
        &self,
        request: &MicrosoftLoginRequest,
    ) -> AuthResult<MicrosoftCallbackPayload> {
        let callback_state = Arc::new(CallbackServerState::new(request.state.clone()));
        let (tx, rx) = oneshot::channel();
        callback_state.set_sender(tx).await;

        let app = Router::new()
            .route("/callback", get(handle_microsoft_callback))
            .with_state(callback_state);

        let listener = TcpListener::bind("127.0.0.1:6767").await?;
        let server = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        if let Err(err) = open::that(&request.auth_url) {
            server.abort();
            return Err(err.into());
        }

        let callback_result = rx.await.map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                "Microsoft callback channel closed",
            )
        })?;
        server.abort();

        let callback = callback_result?;
        if callback.state != request.state {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Microsoft callback state did not match",
            )
            .into());
        }

        Ok(callback)
    }
}

#[derive(Debug, Deserialize)]
struct MicrosoftCallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug)]
struct MicrosoftCallbackPayload {
    code: String,
    state: String,
}

struct CallbackServerState {
    expected_state: String,
    sender: Mutex<Option<oneshot::Sender<AuthResult<MicrosoftCallbackPayload>>>>,
}

impl CallbackServerState {
    fn new(expected_state: String) -> Self {
        Self {
            expected_state,
            sender: Mutex::new(None),
        }
    }

    async fn set_sender(&self, sender: oneshot::Sender<AuthResult<MicrosoftCallbackPayload>>) {
        *self.sender.lock().await = Some(sender);
    }

    async fn send_result(&self, result: AuthResult<MicrosoftCallbackPayload>) {
        if let Some(sender) = self.sender.lock().await.take() {
            let _ = sender.send(result);
        }
    }
}

async fn handle_microsoft_callback(
    State(state): State<Arc<CallbackServerState>>,
    Query(query): Query<MicrosoftCallbackQuery>,
) -> impl IntoResponse {
    let result = match (query.code, query.state, query.error, query.error_description) {
        (_, _, Some(error), description) => {
            let message = match description {
                Some(description) if !description.is_empty() => format!("{error}: {description}"),
                _ => error,
            };

            Err(std::io::Error::new(std::io::ErrorKind::Other, message).into())
        }
        (Some(code), Some(returned_state), None, None) => {
            if returned_state != state.expected_state {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "callback state mismatch",
                )
                .into())
            } else {
                Ok(MicrosoftCallbackPayload {
                    code,
                    state: returned_state,
                })
            }
        }
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "missing code or state in Microsoft callback",
        )
        .into()),
    };

    state.send_result(result).await;

    let message = "Microsoft login received. You can close this window.";
    (StatusCode::OK, message)
}
