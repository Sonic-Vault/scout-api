#![allow(dead_code)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::redundant_field_names)]

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::defi::spl_swap_client::SplSwapClient; // Fixed import path

#[derive(Clone)]
pub struct AppState {
    pub oauth: Arc<tokio::sync::Mutex<Option<crate::auth::OAuthState>>>,
    pub magpie: SplSwapClient, // Use the directly imported type
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub website: String,
    pub logo_uri: String,
    pub symbol: String,
    pub decimals: u64,
    pub address: String,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub name: String,
    pub tvl: f64,
    pub sentiment: String,
    pub whitepaper_summary: String,
    pub github_activity: GithubActivity,
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubActivity {
    pub commits_last_30_days: u64,
    pub contributors: u64,
    pub repo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub mainnet: String,
    pub testnet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceResponse {
    pub balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub trx: String,
}

#[derive(Deserialize, Debug)]
pub struct TransferForm {
    pub user_id: String,
    pub recipient: String,
    pub amount: String,
}
