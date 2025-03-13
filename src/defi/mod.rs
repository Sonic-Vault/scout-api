pub mod models;
pub mod spl_swap_client;

// Use SPL Token Swap client instead of solana_magpie
pub use spl_swap_client::SplSwapClient as MagpieClient;
