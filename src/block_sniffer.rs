use ethers::prelude::*;
use ethers::providers::{MockProvider, Provider, Http};
use std::convert::TryFrom;
use std::sync::Arc;
use std::env;
use tokio::time::{sleep, Duration};
use ethers::types::U64;
use tokio::runtime::Runtime;

pub async fn start_sniffing() {
    dotenv::dotenv().ok();
    let provider = Provider::<Http>::try_from(env::var("RPC_URL").expect("RPC_URL should be set in .env")).expect("invalid provider URL");
    let client = Arc::new(provider);

    let mut last_block_number = U64::zero();

    loop {
        match client.get_block_number().await {
            Ok(block_number) => {
                if block_number > last_block_number {
                    println!("new block: {}", block_number);
                    last_block_number = block_number;
                    //TO DO Code to check arbitrage
                }
            }
            Err(e) => eprintln!("error fetching block number: {}", e),
        }
        sleep(Duration::from_millis(100)).await;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_block_number() {
        let mut provider = MockProvider::new();
        provider.push(U64::from(12345)).unwrap();
        let client = Arc::new(Provider::new(provider));

        let block_number = client.get_block_number().await.unwrap();
        assert_eq!(block_number, U64::from(12345));
    }
}
