use defillama_rs::models::{Chain, Token};
use defillama_rs::{DefiLlamaClient, DefillamaError};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), DefillamaError> {
    // Create the DefiLlama client
    let client = DefiLlamaClient::new();

    // Define tokens to fetch
    // 1. Native tokens
    let sol = Token::native(Chain::Solana);
    let eth = Token::native(Chain::Ethereum);

    // 2. Specific tokens on various chains
    // USDC on Solana
    let usdc_sol = Token::new(
        Chain::Solana,
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
    );

    // USDC on Ethereum
    let usdc_eth = Token::new(
        Chain::Ethereum,
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
    );

    // Raydium on Solana
    let ray_sol = Token::new(
        Chain::Solana,
        "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R".to_string(),
    );

    // Create a list of all tokens to fetch prices for
    let tokens = vec![sol, eth, usdc_sol, usdc_eth, ray_sol];

    // Create a mapping from token identifier to a friendly name
    // (This is just for display purposes in this example)
    let mut token_names = HashMap::new();
    token_names.insert(
        "solana:0x0000000000000000000000000000000000000000",
        "SOL (Native)",
    );
    token_names.insert(
        "ethereum:0x0000000000000000000000000000000000000000",
        "ETH (Native)",
    );
    token_names.insert(
        "solana:EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "USDC on Solana",
    );
    token_names.insert(
        "ethereum:0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
        "USDC on Ethereum",
    );
    token_names.insert(
        "solana:4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R",
        "Raydium (RAY)",
    );

    // Fetch all prices in a single request
    println!("Fetching prices for multiple tokens...");
    let result = client.get_prices(&tokens).await?;

    // Process and display the results
    println!("\nCurrent Prices:");
    println!("{:-<50}", "");

    for (token_id, coin_data) in result.coins {
        let token_id = token_id.as_str();
        let name = token_names.get(token_id).unwrap_or(&token_id);

        println!("Token: {}", name);
        println!("  ID: {}", token_id);
        println!("  Price: ${:.6}", coin_data.price);

        if !coin_data.symbol.is_empty() {
            println!("  Symbol: {}", coin_data.symbol);
        }

        if let Some(decimals) = coin_data.decimals {
            println!("  Decimals: {}", decimals);
        }

        println!("{:-<50}", "");
    }

    Ok(())
}
