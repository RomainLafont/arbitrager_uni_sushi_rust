use crate::router::Router;
use ethers::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;

pub struct Arbitrager<P> {
    router: Router<P>,
    tokens: Vec<(Address, &'static str, u8)>,
}

impl<P: Middleware + 'static> Arbitrager<P> {
    pub fn new(client: Arc<P>) -> Self {
        dotenv().ok();

        let uniswap_router_address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse().unwrap();
        let sushiswap_router_address = "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F".parse().unwrap();
        
        let router = Router::new(uniswap_router_address, sushiswap_router_address, client.clone());

        let tokens = vec![
            ("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap(), "WETH", 18),
            ("0xA0b86991c6218b36c1d19d4a2e9eb0ce3606eb48".parse().unwrap(), "USDC", 6),
            ("0xdAC17F958D2ee523a2206206994597C13D831ec7".parse().unwrap(), "USDT", 6),
            ("0x6B175474E89094C44Da98b954EedeAC495271d0F".parse().unwrap(), "DAI", 18),
            ("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599".parse().unwrap(), "WBTC", 8),
        ];

        Self { router, tokens }
    }

    pub async fn check_arbitrage(&self) -> Result<(), Box<dyn std::error::Error>> {
        let amount_in = U256::exp10(18); // 1 WETH

        for (token, token_name, decimals) in &self.tokens {
            if *token == self.tokens[0].0 { continue; } // Skip WETH/WETH pair

            let path = vec![self.tokens[0].0, *token];
            let (uniswap_amount_out, sushiswap_amount_out) = self.router.get_amount_out(amount_in, path).await?;

            let uniswap_price = uniswap_amount_out.as_u128() as f64 / 10f64.powi(*decimals as i32);
            let sushiswap_price = sushiswap_amount_out.as_u128() as f64 / 10f64.powi(*decimals as i32);

            println!("{}: Uniswap price {}, SushiSwap price {}", token_name, uniswap_price, sushiswap_price);

            println!("{}", self.find_arbitrage(token_name, uniswap_price, sushiswap_price));
        }

        Ok(())
    }

    fn find_arbitrage(&self, token_name: &str, uniswap_price: f64, sushiswap_price: f64) -> String{
        if sushiswap_price > uniswap_price {
            let weth_out = sushiswap_price / uniswap_price;
            format!(
                "Arbitrage opportunity: Buy {} {} for 1 WETH on SushiSwap, sell for {} WETH on Uniswap",
                sushiswap_price, token_name, weth_out
            )
        } else if uniswap_price > sushiswap_price {
            let weth_out = uniswap_price / sushiswap_price;
            format!(
                "Arbitrage opportunity: Buy {} {} for 1 WETH on Uniswap, sell for {} WETH on SushiSwap",
                uniswap_price, token_name, weth_out
            )
        } else {
            format!(
                "No arbitrage opportunity for {}",
                token_name
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::providers::{Provider, Http};
    use ethers::types::U256;
    use std::sync::Arc;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_find_arbitrage() {
        let mut uniswap_price = 1.0;
        let mut sushiswap_price = 1.2;
        let pair = "USDC";
        
        let provider = Arc::new(Provider::<Http>::try_from("http://localhost:8545").unwrap());
        let arbitrager = Arbitrager::new(provider);
        
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