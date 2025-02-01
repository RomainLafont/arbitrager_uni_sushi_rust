use ethers::prelude::*;
use ethers::contract::abigen;
use std::sync::Arc;

abigen!(
    UniswapV2Router02,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts)
    ]"#,
);

abigen!(
    SushiSwapRouter,
    r#"[
        function getAmountsOut(uint amountIn, address[] memory path) external view returns (uint[] memory amounts)
    ]"#,
);

pub struct Router<P> {
    uniswap_router: UniswapV2Router02<P>,
    sushiswap_router: SushiSwapRouter<P>,
}

impl<P: Middleware + 'static> Router<P> {
    pub fn new(uniswap_router_address: Address, sushiswap_router_address: Address, client: Arc<P>) -> Self {
        let uniswap_router = UniswapV2Router02::new(uniswap_router_address, client.clone());
        let sushiswap_router = SushiSwapRouter::new(sushiswap_router_address, client);
        Self { uniswap_router, sushiswap_router }
    }

    pub async fn get_amount_out(&self, amount_in: U256, path: Vec<Address>) -> Result<(U256, U256), Box<dyn std::error::Error>> {
        let uniswap_amounts = self.uniswap_router.get_amounts_out(amount_in, path.clone()).call().await?;
        let sushiswap_amounts = self.sushiswap_router.get_amounts_out(amount_in, path).call().await?;
        Ok((uniswap_amounts[1], sushiswap_amounts[1]))
    }
}