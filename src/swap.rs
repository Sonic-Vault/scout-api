use crate::defi::models::*;
use crate::models::AppState;
use crate::profiles::ProfileDatabase;
use crate::wallets::{WalletDatabase, decode_keypair};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use solana_sdk::signature::Keypair;

pub async fn get_quote(
    State(state): State<AppState>,
    Json(req): Json<GetQuoteRequest>,
) -> Result<Json<QuoteResponse>, (StatusCode, String)> {
    let params = QuoteParams {
        from_token_address: req.from_token,
        to_token_address: req.to_token,
        amount: req.amount,
        slippage: req.slippage,
        from_address: req.from_address,
        to_address: req.to_address,
        gasless: req.gasless,
        affiliate_address: req.affiliate_address,
        affiliate_fee: req.affiliate_fee,
    };

    let response = state.magpie.get_quote(&params).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get quote: {}", e),
        )
    })?;

    Ok(Json(response))
}

pub async fn execute_swap(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<ExecuteSwapRequest>,
) -> Result<Json<SwapResponse>, (StatusCode, String)> {
    // Get user profile and wallet
    let profile_db = ProfileDatabase::new().unwrap();
    let wallet_db = WalletDatabase::new().unwrap();
    
    let profile = profile_db.get(&user_id).unwrap().ok_or_else(|| {
        (StatusCode::NOT_FOUND, "Profile not found".to_string())
    })?;
    
    let wallet = wallet_db.get(&profile.wallet).unwrap().ok_or_else(|| {
        (StatusCode::NOT_FOUND, "Wallet not found for user".to_string())
    })?;

    // Decode private key into Solana keypair
    let keypair = decode_keypair(&wallet.private).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid keypair: {}", e),
        )
    })?;

    // Execute the swap using SPL Token Swap
    let response = state.magpie.execute_swap(&user_id, &req.quote_id, &keypair)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute swap: {}", e),
            )
        })?;

    Ok(Json(response))
}

pub async fn get_swap_status(
    State(state): State<AppState>,
    Query(req): Query<SwapStatusRequest>,
) -> Result<Json<SwapStatusResponse>, (StatusCode, String)> {
    let response = state
        .magpie
        .get_swap_status(&req.wallet_address)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get swap status: {}", e),
            )
        })?;

    Ok(Json(response))
}

pub async fn get_swap_details(
    State(state): State<AppState>,
    Query(req): Query<SwapDetailsRequest>,
) -> Result<Json<SwapDetailsResponse>, (StatusCode, String)> {
    let response = state
        .magpie
        .get_swap_details(&req.swap_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get swap details: {}", e),
            )
        })?;

    Ok(Json(response))
}

pub async fn get_distributions(
    State(state): State<AppState>,
    Query(req): Query<GetDistributionsRequest>,
) -> Result<Json<DistributionsResponse>, (StatusCode, String)> {
    let response = state
        .magpie
        .get_distributions(&req.quote_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get distributions: {}", e),
            )
        })?;

    Ok(Json(response))
}
