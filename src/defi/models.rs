use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteParams {
    #[serde(rename = "fromTokenAddress")]
    pub from_token_address: String,
    #[serde(rename = "toTokenAddress")]
    pub to_token_address: String,
    pub amount: String,
    pub slippage: String,
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    #[serde(rename = "toAddress")]
    pub to_address: String,
    pub gasless: bool,
    #[serde(skip_serializing_if = "Option::is_none", rename = "affiliateAddress")]
    pub affiliate_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "affiliateFee")]
    pub affiliate_fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    pub quote_id: String,
    #[serde(rename = "toTokenAmount")]
    pub to_token_amount: String,
    pub fees: Fees,
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
    // This would contain the actual EIP-712 types structure
    // Simplified for this example
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
    pub to: String,
    pub data: String,
    pub value: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResponse {
    pub swap_id: String,
    pub status: String,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapStatusResponse {
    pub pending: i32,
    pub error: i32,
    pub completed: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapDetailsResponse {
    pub id: String,
    pub status: String,
    pub tx_hash: Option<String>,
    pub from_token: TokenInfo,
    pub to_token: TokenInfo,
    pub from_amount: String,
    pub to_amount: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionsResponse {
    pub distributions: Vec<Distribution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    pub dex: String,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuoteRequest {
    pub from_token: String,
    pub to_token: String,
    pub amount: String,
    pub slippage: String,
    pub from_address: String,
    pub to_address: String,
    pub gasless: bool,
    pub affiliate_address: Option<String>,
    pub affiliate_fee: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteSwapRequest {
    pub quote_id: String,
    pub network_name: String,
    pub wallet_key: Option<String>,
    pub permit_deadline: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapStatusRequest {
    pub wallet_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapDetailsRequest {
    pub swap_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDistributionsRequest {
    pub quote_id: String,
}
