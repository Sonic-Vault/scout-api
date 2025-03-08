mod auth;
mod constants;
mod models;
mod profiles;
mod wallets;

use auth::{callback, login};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use models::{BalanceResponse, Project, ProjectSummary, TransactionResponse, TransferForm};
use profiles::{Profile, ProfileDatabase};
use std::fs;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_state = auth::AppState {
        oauth_state: Arc::new(tokio::sync::Mutex::new(None)),
    };

    let app = Router::new()
        .route("/profile/:id", get(get_profile))
        .route("/login/:id", get(login))
        .route("/callback", get(callback))
        .route("/projects", get(get_projects))
        .route("/projects/:chain/:pid", get(get_project))
        .route("/balance/:id", get(get_balance))
        .route("/transfer", post(do_transfer))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let addr = "0.0.0.0:7000".parse().unwrap();
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_projects() -> Json<Vec<Project>> {
    let file_content = fs::read_to_string("projects.json").unwrap_or("[]".to_string());
    let items: Vec<Project> = serde_json::from_str(&file_content).unwrap_or_else(|_| vec![]);
    Json(items)
}

async fn get_project() -> Json<ProjectSummary> {
    let file_content = fs::read_to_string("project.json").unwrap_or("{}".to_string());
    let summ: ProjectSummary = serde_json::from_str(&file_content).unwrap();
    Json(summ)
}

async fn get_profile(
    State(_state): State<auth::AppState>,
    Path(user_id): Path<String>,
) -> Json<Profile> {
    let db = ProfileDatabase::new().unwrap();
    let profile = db.get(&user_id).unwrap();
    if let Some(p) = profile {
        return Json(p);
    }

    Json(Profile::default())
}

async fn get_balance(
    State(_state): State<auth::AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    let balance = wallets::get_balance(&user_id).await;
    let my_balance = BalanceResponse {
        balance: balance.unwrap(),
    };

    (StatusCode::OK, Json(my_balance)).into_response()
}

async fn do_transfer(
    State(_state): State<auth::AppState>,
    Json(payload): Json<TransferForm>,
) -> impl IntoResponse {
    let trx = wallets::transfer(&payload.user_id, &payload.recipient, &payload.amount).await;
    let my_transaction = TransactionResponse { trx: trx.unwrap() };

    (StatusCode::OK, Json(my_transaction)).into_response()
}
