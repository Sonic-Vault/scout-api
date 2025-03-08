use crate::constants::{TWITTER_OAUTH_AUTHORIZE_URL, TWITTER_OAUTH_TOKEN_URL};
use crate::profiles::{Profile, ProfileDatabase};
use crate::wallets::{Wallet, WalletDatabase};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use hex;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rand::thread_rng;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct OAuthState {
    client: BasicClient,
    csrf_token: CsrfToken,
    pkce_verifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterUserResponse {
    pub data: TwitterUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterUser {
    pub id: String,
    pub name: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Clone)]
pub struct AppState {
    pub oauth_state: Arc<tokio::sync::Mutex<Option<OAuthState>>>,
}

pub fn create_twitter_oauth_client() -> Result<BasicClient, Box<dyn std::error::Error>> {
    let client_id = ClientId::new(
        env::var("TWITTER_CLIENT_ID")
            .map_err(|_| "Missing TWITTER_CLIENT_ID environment variable")?,
    );
    let client_secret = ClientSecret::new(
        env::var("TWITTER_CLIENT_SECRET")
            .map_err(|_| "Missing TWITTER_CLIENT_SECRET environment variable")?,
    );
    let redirect_url = RedirectUrl::new(
        env::var("TWITTER_REDIRECT_URL")
            .map_err(|_| "Missing TWITTER_REDIRECT_URL environment variable")?,
    )?;

    let auth_url = AuthUrl::new(TWITTER_OAUTH_AUTHORIZE_URL.to_string())?;
    let token_url = TokenUrl::new(TWITTER_OAUTH_TOKEN_URL.to_string())?;

    Ok(
        BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
            .set_redirect_uri(redirect_url),
    )
}

pub async fn login(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let client = match create_twitter_oauth_client() {
        Ok(client) => client,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(format!("OAuth setup failed: {}", e)),
            )
                .into_response();
        }
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let scopes = vec![
        Scope::new("tweet.read".to_string()),
        Scope::new("users.read".to_string()),
    ];

    let (auth_url, csrf_token) = client
        .authorize_url(|| CsrfToken::new(user_id))
        .add_scopes(scopes)
        .set_pkce_challenge(pkce_challenge)
        .url();

    let mut oauth_state = state.oauth_state.lock().await;
    *oauth_state = Some(OAuthState {
        client,
        csrf_token: csrf_token.clone(),
        pkce_verifier: pkce_verifier.secret().to_string(), // Store as string
    });

    (StatusCode::OK, Html(format!("{}", auth_url.as_str()))).into_response()
}

pub async fn callback(
    State(state): State<AppState>,
    Query(params): Query<CallbackQuery>,
) -> impl IntoResponse {
    let mut oauth_state_guard = state.oauth_state.lock().await;

    let oauth_state = match oauth_state_guard.take() {
        Some(state) => state,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Html("OAuth state not found".to_string()),
            )
                .into_response()
        }
    };

    if params.state != *oauth_state.csrf_token.secret() {
        return (
            StatusCode::UNAUTHORIZED,
            Html("CSRF token mismatch".to_string()),
        )
            .into_response();
    }

    let pkce_verifier = PkceCodeVerifier::new(oauth_state.pkce_verifier);

    match oauth_state
        .client
        .exchange_code(AuthorizationCode::new(params.code))
        .set_pkce_verifier(pkce_verifier)
        .request_async(oauth2::reqwest::async_http_client)
        .await
    {
        Ok(token_response) => {
            let access_token = token_response.access_token().secret().to_string();

            match fetch_user_info(&access_token).await {
                Ok(user) => {
                    let profile_db = ProfileDatabase::new().unwrap();

                    match profile_db.get(&params.state).unwrap() {
                        Some(_profile) => {}
                        None => {
                            let mut rng = thread_rng();
                            let signing_key = SigningKey::random(&mut rng);
                            let private_key_bytes = signing_key.to_bytes();
                            let private_key_hex = hex::encode(private_key_bytes);
                            let wallet = LocalWallet::from_bytes(&private_key_bytes)
                                .expect("Invalid private key");
                            let address = wallet.address();

                            let wallet_db = WalletDatabase::new().unwrap();

                            let wallet = Wallet {
                                id: None,
                                address: format!("{:#x}", address),
                                private: private_key_hex,
                            };

                            let _ = wallet_db.create(&wallet).unwrap();

                            let profile = Profile {
                                id: None,
                                user_id: params.state,
                                username: user.data.username.to_string(),
                                name: user.data.name.to_string(),
                                wallet: format!("{:#x}", address),
                            };

                            let _ = profile_db.upsert(&profile).unwrap();
                        }
                    };

                    (
                        StatusCode::OK,
                        Html(format!(
                            "Logged in successfully! <br/> User: {} ({}) <br/> You can close this page",
                            user.data.name, user.data.username
                        )),
                    )
                        .into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(format!("Failed to fetch user info: {}", e)),
                )
                    .into_response(),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(format!("Token exchange failed: {}", e)),
        )
            .into_response(),
    }
}

pub async fn fetch_user_info(access_token: &str) -> Result<TwitterUserResponse, reqwest::Error> {
    let client = HttpClient::new();
    client
        .get("https://api.twitter.com/2/users/me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?
        .json::<TwitterUserResponse>()
        .await
}
