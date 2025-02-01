use ethers::prelude::*;
use ethers::contract::abigen;
use std::sync::Arc;

abigen!(
    UniswapV2Pair,
    r#"[
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
    ]"#,
);

pub struct UniswapPair {
    contract: UniswapV2Pair<Provider<Http>>,
    decimals0: u8,
    decimals1: u8,
}

impl UniswapPair {
    pub fn new(address: Address, client: Arc<Provider<Http>>, decimals0: u8, decimals1: u8) -> Self {
        let contract = UniswapV2Pair::new(address, client);
        Self { contract, decimals0, decimals1 }
    }

    pub async fn get_price(&self) -> Result<f64, Box<dyn std::error::Error>> {
        let (reserve0, reserve1, _) = self.contract.get_reserves().call().await?;
        println!("uniswap: reserve0={} reserve1={}", reserve0, reserve1);
        let price = (reserve0 as f64 / 10f64.powi(self.decimals0 as i32)) / (reserve1 as f64 / 10f64.powi(self.decimals1 as i32));
        Ok(price)
    }
}

/*#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::MockProvider;
    use ethers::types::U256;

    #[tokio::test]
    async fn test_get_price() {
        let provider = MockProvider::new();
        let client = Arc::new(provider.clone());

        provider.push(U256::from(1000), U256::from(2000), U256::from(0));

        let uniswap_pair = UniswapPair::new("0x0000000000000000000000000000000000000000".parse().unwrap(), client, 18, 6);

        let price = uniswap_pair.get_price().await.unwrap();
        assert_eq!(price, 0.5);
    }
}*/