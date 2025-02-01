mod arbitrager;
mod sushiswap_pair;
mod uniswap_pair;
mod factory;
mod block_sniffer;

#[tokio::main]
async fn main() {
    block_sniffer::start_sniffing().await;
}