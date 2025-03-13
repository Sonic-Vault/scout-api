#![allow(dead_code)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::redundant_field_names)]

use crate::constants::DB_PATH;
use crate::profiles::ProfileDatabase;
use rusqlite::OptionalExtension;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};

use bs58;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    message::Message,
    pubkey::Pubkey,
    signature::Keypair, // Removed unused Signature import
    signer::Signer,
    system_instruction,
    transaction::Transaction,
};
use std::{env, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Wallet {
    pub id: Option<i64>,
    pub address: String,
    pub private: String, // Base58 encoded private key
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

// Change from `fn decode_keypair` to `pub fn decode_keypair` to make it accessible
pub fn decode_keypair(private_key: &str) -> Result<Keypair> {
    let bytes = bs58::decode(private_key)
        .into_vec()
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    
    let keypair = Keypair::from_bytes(&bytes)
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    
    Ok(keypair)
}

pub async fn get_balance(user_id: &str) -> Result<String> {
    let db = ProfileDatabase::new().unwrap();
    let profile = db.get(&user_id).unwrap();

    if let Some(p) = profile {
        let rpc_client = RpcClient::new(env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()));
        
        let pubkey = Pubkey::from_str(&p.wallet)
            .map_err(|_| rusqlite::Error::InvalidQuery)?;
        
        match rpc_client.get_balance(&pubkey) {
            Ok(balance) => {
                // Convert lamports to SOL (1 SOL = 10^9 lamports)
                let sol_balance = balance as f64 / 1_000_000_000.0;
                return Ok(sol_balance.to_string());
            },
            Err(_) => return Ok("0".to_string()),
        }
    }

    Ok("0".to_string())
}

pub async fn transfer(user_id: &str, recipient: &str, amount: &str) -> Result<String> {
    let db = ProfileDatabase::new().unwrap();
    let profile = db.get(&user_id).unwrap();

    if let Some(p) = profile {
        let db = WalletDatabase::new().unwrap();
        let wallet = db.get(&p.wallet).unwrap();
        
        if let Some(w) = wallet {
            // Parse amount in SOL to lamports
            let amount = amount.parse::<f64>()
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            let lamports = (amount * 1_000_000_000.0) as u64;
            
            // Create a Solana client
            let rpc_client = RpcClient::new_with_commitment(
                env::var("RPC_URL").unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
                CommitmentConfig::confirmed(),
            );
            
            // Decode the keypair from the stored private key
            let keypair = decode_keypair(&w.private)?;
            
            // Parse recipient address
            let recipient = Pubkey::from_str(recipient)
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            
            // Create the transfer instruction
            let instruction = system_instruction::transfer(
                &keypair.pubkey(),
                &recipient,
                lamports,
            );
            
            // Get recent blockhash
            let recent_blockhash = rpc_client
                .get_latest_blockhash()
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            
            // Create a transaction
            let message = Message::new_with_blockhash(
                &[instruction],
                Some(&keypair.pubkey()),
                &recent_blockhash,
            );
            
            let transaction = Transaction::new(
                &[&keypair],
                message,
                recent_blockhash,
            );
            
            // Send the transaction
            match rpc_client.send_and_confirm_transaction(&transaction) {
                Ok(signature) => {
                    let explorer_url = env::var("CHAIN_EXPLORER_URL")
                        .unwrap_or_else(|_| "https://explorer.solana.com".to_string());
                    return Ok(format!("{}/tx/{}", explorer_url, signature));
                },
                Err(_) => return Ok("Transaction failed".to_string()),
            }
        }
    }

    Ok("0x".to_string())
}
