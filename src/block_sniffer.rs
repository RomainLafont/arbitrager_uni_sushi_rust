use crate::arbitrager::Arbitrager;
use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::convert::TryFrom;
use std::sync::Arc;
use std::env;
use tokio::time::{sleep, Duration};

pub async fn start_sniffing() {
    dotenv::dotenv().ok();
    let provider = Provider::<Http>::try_from(env::var("RPC_URL").expect("RPC_URL should be set in .env")).expect("Invalid provider URL");
    let client = Arc::new(provider);

    let arbitrager = Arbitrager::new(client.clone());
    let mut last_block_number = U64::zero();

    loop {
        match client.get_block_number().await {
            Ok(block_number) => {
                if block_number > last_block_number {
                    println!("New block: {}", block_number);
                    last_block_number = block_number;
                    match arbitrager.check_arbitrage().await {
                        Ok(_) => {}
                        Err(e) => eprintln!("Error checking arbitrage: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Error fetching block number: {}", e),
        }

        // Sleep for a few hundred milliseconds before checking again
        sleep(Duration::from_millis(1000)).await;
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
