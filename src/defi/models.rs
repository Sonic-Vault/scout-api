#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::redundant_field_names)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuoteRequest {
    pub from_token: String,
    pub to_token: String,
    pub amount: String,
    pub slippage: Option<f64>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub gasless: Option<bool>,
    pub affiliate_address: Option<String>,
    pub affiliate_fee: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteParams {
    #[serde(rename = "fromTokenAddress")]
    pub from_token_address: String,
    #[serde(rename = "toTokenAddress")]
    pub to_token_address: String,
    pub amount: String,
    pub slippage: Option<f64>,
    #[serde(rename = "fromAddress")]
    pub from_address: Option<String>,
    #[serde(rename = "toAddress")]
    pub to_address: Option<String>,
    pub gasless: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "affiliateAddress")]
    pub affiliate_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "affiliateFee")]
    pub affiliate_fee: Option<f64>,
}

// Merged QuoteResponse struct with all fields from both versions
#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub quote_id: String,
    pub from_token: TokenInfo,
    pub to_token: TokenInfo,
    pub from_amount: String,
    pub to_amount: String,
    pub to_token_amount: Option<String>,
    pub estimated_gas: Option<String>,
    pub route: Option<Vec<RouteStep>>,
    pub valid_until: String,
    pub fees: Option<Fees>,
    pub message: Option<EIP712Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fees {
    pub network: String,
    pub estimated_gas: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EIP712Message {
    pub domain: EIP712Domain,
    pub types: EIP712Types,
    pub message: EIP712MessageParams,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EIP712Domain {
    pub name: String,
    pub version: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "verifyingContract")]
    pub verifying_contract: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EIP712Types {
    #[serde(rename = "Swap")]
    pub swap: Vec<EIP712Field>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EIP712Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EIP712MessageParams {
    #[serde(rename = "fromToken")]
    pub from_token: String,
    #[serde(rename = "toToken")]
    pub to_token: String,
    pub amount: String,
    pub recipient: String,
    pub deadline: String,
}

// Merged TokenInfo struct with all fields from both versions
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: Option<String>,
    pub decimals: u8,
    pub logo_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteStep {
    pub protocol: String,
    pub percent: f64,
    pub from_token_address: String,
    pub to_token_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
    pub to: String,
    pub data: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteSwapRequest {
    pub quote_id: String,
    pub network_name: String,
    pub permit_deadline: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapTransactionData {
    pub quote_id: String,
    pub transaction_data: String,
    pub gas_estimate: Option<String>,
}

// Merged GaslessSwapParams with all fields
#[derive(Debug, Serialize, Deserialize)]
pub struct GaslessSwapParams {
    #[serde(rename = "networkName")]
    pub network_name: String,
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    #[serde(rename = "swapSignature")]
    pub swap_signature: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "permitSignature")]
    pub permit_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "permitDeadline")]
    pub permit_deadline: Option<String>,
}

// Merged SwapResponse with all fields
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResponse {
    pub swap_id: String,
    pub status: String,
    pub tx_hash: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapStatusRequest {
    pub wallet_address: String,
}

// Merged SwapStatusResponse with all fields
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapStatusResponse {
    pub swaps: Vec<SwapStatusItem>,
    pub pending: Option<i32>,
    pub error: Option<i32>,
    pub completed: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapStatusItem {
    pub swap_id: String,
    pub status: String,
    pub created_at: String,
    pub from_token: TokenInfo,
    pub to_token: TokenInfo,
    pub from_amount: String,
    pub to_amount: String,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapDetailsRequest {
    pub swap_id: String,
}

// Merged SwapDetailsResponse with all fields
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapDetailsResponse {
    pub id: Option<String>,
    pub swap_id: Option<String>,
    pub status: String,
    pub tx_hash: Option<String>,
    pub from_token: TokenInfo,
    pub to_token: TokenInfo,
    pub from_amount: String,
    pub to_amount: String,
    pub timestamp: Option<u64>,
    pub created_at: Option<String>,
    pub completed_at: Option<String>,
    pub block_number: Option<u64>,
    pub gas_used: Option<String>,
    pub gas_price: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDistributionsRequest {
    pub quote_id: String,
}

// Merge Distribution types to resolve conflict
#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionItem {
    pub protocol: String,
    pub amount: String,
    pub percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    pub dex: String,
    pub percentage: f64,
}

// Merged DistributionsResponse with all fields
#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionsResponse {
    pub quote_id: Option<String>,
    pub distributions: Vec<DistributionItem>,
}
