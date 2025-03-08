#![allow(dead_code)]

use crate::constants::DB_PATH;
use crate::profiles::ProfileDatabase;
use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

use ethers::{
    prelude::*,
    providers::{Http, Provider},
    types::Address,
    utils::format_units,
};
use std::{convert::TryFrom, env, str::FromStr, sync::Arc};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Wallet {
    pub id: Option<i64>,
    pub address: String,
    pub private: String,
}

pub struct WalletDatabase {
    pub conn: Connection,
}

impl WalletDatabase {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(DB_PATH)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS wallets (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                address TEXT NOT NULL UNIQUE,
                private TEXT NOT NULL
            )",
            [],
        )?;

        Ok(WalletDatabase { conn })
    }

    pub fn create(&self, wallet: &Wallet) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO wallets (address, private) VALUES (?1, ?2)",
            params![wallet.address, wallet.private],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get(&self, address: &str) -> Result<Option<Wallet>> {
        let wallet = self
            .conn
            .query_row(
                "SELECT id, address, private FROM wallets WHERE address = ?1",
                params![address],
                |row| {
                    Ok(Wallet {
                        id: row.get(0)?,
                        address: row.get(1)?,
                        private: row.get(2)?,
                    })
                },
            )
            .optional()?;

        Ok(wallet)
    }

    pub fn delete(&self, address: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM wallets WHERE address = ?1", params![address])?;
        Ok(())
    }
}

pub async fn get_balance(user_id: &str) -> Result<String> {
    let db = ProfileDatabase::new().unwrap();
    let profile = db.get(&user_id).unwrap();

    if let Some(p) = profile {
        let provider = Provider::<Http>::try_from(env::var("RPC_URL").unwrap()).unwrap();

        let provider = Arc::new(provider);
        let address = Address::from_str(&p.wallet).unwrap();
        let balance = provider.get_balance(address, None).await.unwrap();
        let formatted = format_units(balance, 18).unwrap();
        return Ok(formatted.to_string());
    }

    return Ok("0".to_string());
}

pub async fn transfer(user_id: &str, recipient: &str, amount: &str) -> Result<String> {
    let db = ProfileDatabase::new().unwrap();
    let profile = db.get(&user_id).unwrap();

    if let Some(p) = profile {
        let db = WalletDatabase::new().unwrap();
        let wallet = db.get(&p.wallet).unwrap();
        if let Some(w) = wallet {
            let provider = Provider::<Http>::try_from(env::var("RPC_URL").unwrap()).unwrap();
            let provider = Arc::new(provider);
            let chain_id = env::var("CHAIN_ID")
                .unwrap()
                .parse::<u64>()
                .expect("Invalid chain ID");

            let wallet = w
                .private
                .parse::<LocalWallet>()
                .unwrap()
                .with_chain_id(chain_id);
            println!("addr {}", wallet.address());
            let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));

            let to_address = recipient.parse::<Address>().unwrap();
            let value = ethers::utils::parse_ether(amount).unwrap();

            let tx = TransactionRequest::new()
                .from(wallet.address())
                .to(to_address)
                .value(value)
                .gas_price(provider.get_gas_price().await.unwrap())
                .gas(21000); // Standard gas limit for ETH transfers

            match client.send_transaction(tx, None).await {
                Ok(pending_tx) => {
                    let tx_hash = pending_tx.tx_hash();
                    println!("Transaction sent! Tx Hash: {:?}", tx_hash);

                    let receipt = pending_tx.await.unwrap();
                    println!(
                        "Transaction confirmed in block: {:?}",
                        receipt.unwrap().block_number
                    );

                    return Ok(format!(
                        "{}/tx/{:?}",
                        env::var("CHAIN_EXPLORER_URL").unwrap(),
                        tx_hash
                    ));
                }
                Err(_) => {}
            };
        }
    }

    return Ok("0x".to_string());
}
