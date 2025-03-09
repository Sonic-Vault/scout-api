mod auth;
mod constants;
mod defi;
mod models;
mod profiles;
mod swap;
mod wallets;

use auth::{callback, login};
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use models::{
    AppState, BalanceResponse, Project, ProjectSummary, TransactionResponse, TransferForm,
};
use profiles::{Profile, ProfileDatabase};
use std::sync::Arc;
use std::{env, fs};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = AppState {
        oauth: Arc::new(tokio::sync::Mutex::new(None)),
        magpie: defi::magpiefi::MagpieClient::new(&env::var("MAGPIEFI_API_URL").unwrap()),
    };

    let app = Router::new()
        .route("/profile/:id", get(get_profile))
        .route("/login/:id", get(login))
        .route("/callback", get(callback))
        .route("/projects", get(get_projects))
        .route("/projects/:chain/:pid", get(get_project))
        .route("/balance/:id", get(get_balance))
        .route("/transfer", post(execute_transfer))
        .route("/swap/quote", post(swap::get_quote))
        .route("/swap/execute", post(swap::execute_swap))
        .route("/swap/status", get(swap::get_swap_status))
        .route("/swap/details", get(swap::get_swap_details))
        .route("/swap/distributions", get(swap::get_distributions))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = env::var("SERVER_HOST").unwrap().parse().unwrap();
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

async fn get_profile(Path(user_id): Path<String>) -> Json<Profile> {
    let db = ProfileDatabase::new().unwrap();
    let profile = db.get(&user_id).unwrap();
    if let Some(p) = profile {
        return Json(p);
    }

    Json(Profile::default())
}

async fn get_balance(Path(user_id): Path<String>) -> impl IntoResponse {
    let balance = wallets::get_balance(&user_id).await;
    let my_balance = BalanceResponse {
        balance: balance.unwrap(),
    };

    (StatusCode::OK, Json(my_balance)).into_response()
}

async fn execute_transfer(Json(payload): Json<TransferForm>) -> impl IntoResponse {
    let trx = wallets::transfer(&payload.user_id, &payload.recipient, &payload.amount).await;
    let my_transaction = TransactionResponse { trx: trx.unwrap() };

    (StatusCode::OK, Json(my_transaction)).into_response()
}
