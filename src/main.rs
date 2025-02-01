mod arbitrager;
mod router;
mod block_sniffer;

#[tokio::main]
async fn main() {
    block_sniffer::start_sniffing().await;
}