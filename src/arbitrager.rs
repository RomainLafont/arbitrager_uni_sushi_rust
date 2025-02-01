use crate::sushiswap_pair::SushiSwapPairWrapper;
use crate::uniswap_pair::UniswapPair;
use crate::factory::Factory;
use ethers::prelude::*;
use ethers::providers::{Provider, Http, MockProvider};
use std::convert::TryFrom;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use ethers::types::Address;

pub struct Arbitrager {
    client: Arc<Provider<Http>>,
    factory: Factory,
    tokens: Vec<(Address, &'static str, u8)>,
}

impl Arbitrager {
    pub fn new() -> Self {
        dotenv().ok();
        let provider = Provider::<Http>::try_from(env::var("RPC_URL").expect("RPC_URL should be set in .env")).expect("invalid provider URL");
        let client = Arc::new(provider.clone());

        let uniswap_factory_address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse().unwrap(); 
        let sushiswap_factory_address = "0xC0AEe478e3658e2610c5F7A4A2E1777Ce9e4f2Ac".parse().unwrap();

        let factory = Factory::new(uniswap_factory_address, sushiswap_factory_address, client.clone());

        let tokens = vec![
            ("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap(), "WETH", 18), 
            ("0xA0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse().unwrap(), "USDC", 6),
            ("0xdAC17F958D2ee523a2206206994597C13D831ec7".parse().unwrap(), "USDT", 6),
            ("0x6B175474E89094C44Da98b954EedeAC495271d0F".parse().unwrap(), "DAI", 18),
            ("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599".parse().unwrap(), "WBTC", 8),
        ];

        Self { client, factory, tokens }
    }

    pub async fn check_arbitrage(&self) -> Result<(), Box<dyn std::error::Error>> {
        for (token, token_name, decimals) in &self.tokens {
            if *token == self.tokens[0].0 { continue; } // skip WETH/WETH pair

            let (uniswap_address, sushiswap_address) = self.factory.get_pair(self.tokens[0].0, *token).await?;

            let uniswap_pair = UniswapPair::new(uniswap_address, self.client.clone(), 18, *decimals);
            let sushiswap_pair = SushiSwapPairWrapper::new(sushiswap_address, self.client.clone(), 18, *decimals);

            let uniswap_price = uniswap_pair.get_price().await?;
            let sushiswap_price = sushiswap_pair.get_price().await?;

            println!("{}", self.find_arbitrage(token_name, uniswap_price, sushiswap_price));
        }

        Ok(())
    }

    fn find_arbitrage(&self, pair: &str, uniswap_price: f64, sushiswap_price: f64) -> String {
        if sushiswap_price > uniswap_price {
            let weth_out = sushiswap_price / uniswap_price;
            format!(
                "Arbitrage opportunity: Buy {} {} for 1 WETH on SushiSwap, sell for {} WETH on Uniswap",
                sushiswap_price, pair, weth_out
            )
        } else if uniswap_price > sushiswap_price {
            let weth_out = uniswap_price / sushiswap_price;
            format!(
                "Arbitrage opportunity: Buy {} {} for 1 WETH on Uniswap, sell for {} WETH on SushiSwap",
                uniswap_price, pair, weth_out
            )
        } else {
            format!(
                "No arbitrage opportunity for {}",
                pair
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_find_arbitrage() {
        let mut uniswap_price = 1.0;
        let mut sushiswap_price = 1.2;
        let pair = "USDC";
        let arbitrager = Arbitrager::new();

        let output = arbitrager.find_arbitrage(pair, uniswap_price, sushiswap_price);
        assert!(output.contains(&format!(
            "Arbitrage opportunity: Buy {} {} for 1 WETH on SushiSwap, sell for {} WETH on Uniswap",
            sushiswap_price, "USDC", sushiswap_price / uniswap_price
        )));

        uniswap_price = 1.2;
        sushiswap_price = 1.0;

        let output = arbitrager.find_arbitrage(pair, uniswap_price, sushiswap_price);
        assert!(output.contains(&format!(
            "Arbitrage opportunity: Buy {} {} for 1 WETH on Uniswap, sell for {} WETH on SushiSwap",
            uniswap_price, "USDC", uniswap_price / sushiswap_price
        )));

        uniswap_price = 1.0;
        sushiswap_price = 1.0;

        let output = arbitrager.find_arbitrage(pair, uniswap_price, sushiswap_price);
        assert!(output.contains(&format!(
            "No arbitrage opportunity for {}",
            "USDC"
        )));
    }
}
