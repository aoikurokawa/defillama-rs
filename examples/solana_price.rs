use defillama_rs::models::{Chain, Token};
use defillama_rs::{DefiLlamaClient, DefillamaError};

#[tokio::main]
async fn main() -> Result<(), DefillamaError> {
    // Create a client
    let client = DefiLlamaClient::new();

    // Get the Solana token
    // For native tokens, we can use the Token::native helper
    let solana_token = Token::native(Chain::Solana);

    // Fetch the price
    let response = client.get_price(&solana_token).await?;

    // Print the result
    println!("Solana price data:");
    for (token_id, data) in response.coins {
        println!("Token ID: {}", token_id);
        println!("Price: ${:.4}", data.price);

        if !data.symbol.is_empty() {
            println!("Symbol: {}", data.symbol);
        }
    }

    // Alternative way: if you need a specific token on Solana
    // For example, a specific SPL token
    let spl_token_address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"; // USDC on Solana
    let usdc_token = Token::new(Chain::Solana, spl_token_address.to_string());

    // Fetch USDC price
    let response = client.get_price(&usdc_token).await?;

    // Print USDC price data
    println!("\nUSDC on Solana price data:");
    for (token_id, data) in response.coins {
        println!("Token ID: {}", token_id);
        println!("Price: ${:.4}", data.price);

        if !data.symbol.is_empty() {
            println!("Symbol: {}", data.symbol);
        }
    }

    Ok(())
}
