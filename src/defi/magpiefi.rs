use super::models::*;
use reqwest::Client;

#[derive(Clone)]
pub struct MagpieClient {
    pub client: Client,
    pub base_url: String,
}

impl MagpieClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_quote(&self, params: &QuoteParams) -> Result<QuoteResponse, reqwest::Error> {
        let url = format!("{}/aggregator/quote", self.base_url);
        self.client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_transaction(&self, quote_id: &str) -> Result<TransactionData, reqwest::Error> {
        let url = format!("{}/aggregator/transaction", self.base_url);
        self.client
            .get(&url)
            .query(&[("quoteId", quote_id)])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn execute_gasless_swap(
        &self,
        params: &GaslessSwapParams,
    ) -> Result<SwapResponse, reqwest::Error> {
        let url = format!("{}/user-manager/execute-swap", self.base_url);
        self.client
            .post(&url)
            .json(&params)
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_swap_status(
        &self,
        wallet_address: &str,
    ) -> Result<SwapStatusResponse, reqwest::Error> {
        let url = format!("{}/user-manager/status-counts", self.base_url);
        self.client
            .get(&url)
            .query(&[("walletAddress", wallet_address)])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_swap_details(
        &self,
        swap_id: &str,
    ) -> Result<SwapDetailsResponse, reqwest::Error> {
        let url = format!("{}/user-manager/swap", self.base_url);
        self.client
            .get(&url)
            .query(&[("swapId", swap_id)])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn get_distributions(
        &self,
        quote_id: &str,
    ) -> Result<DistributionsResponse, reqwest::Error> {
        let url = format!("{}/aggregator/distributions", self.base_url);
        self.client
            .get(&url)
            .query(&[("quote-id", quote_id)])
            .send()
            .await?
            .json()
            .await
    }
}
