use ethers::prelude::*;
use ethers::contract::abigen;
use std::sync::Arc;

abigen!(
    UniswapV2Factory,
    r#"[
        function getPair(address tokenA, address tokenB) external view returns (address pair)
    ]"#,
);

abigen!(
    SushiSwapFactory,
    r#"[
        function getPair(address tokenA, address tokenB) external view returns (address pair)
    ]"#,
);

pub struct Factory {
    uniswap_factory: UniswapV2Factory<Provider<Http>>,
    sushiswap_factory: SushiSwapFactory<Provider<Http>>,
}

impl Factory {
    pub fn new(uniswap_factory_address: Address, sushiswap_factory_address: Address, client: Arc<Provider<Http>>) -> Self {
        let uniswap_factory = UniswapV2Factory::new(uniswap_factory_address, client.clone());
        let sushiswap_factory = SushiSwapFactory::new(sushiswap_factory_address, client);
        Self { uniswap_factory, sushiswap_factory }
    }

    pub async fn get_pair(&self, token_a: Address, token_b: Address) -> Result<(Address, Address), Box<dyn std::error::Error>> {
        let uniswap_pair = self.uniswap_factory.get_pair(token_a, token_b).call().await?;
        let sushiswap_pair = self.sushiswap_factory.get_pair(token_a, token_b).call().await?;
        Ok((uniswap_pair, sushiswap_pair))
    }
}