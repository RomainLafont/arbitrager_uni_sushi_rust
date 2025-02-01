# Arbitrager Uni Sushi Rust

A small arbitrager program in Rust for a technical test. This program checks for arbitrage opportunities between Uniswap and SushiSwap.

## Features

- Fetches the latest block number from an Ethereum node.
- Checks for arbitrage opportunities between Uniswap and SushiSwap.
- Prints arbitrage opportunities to the console.

## Prerequisites

- Rust and Cargo installed. You can install them from [rustup.rs](https://rustup.rs/).
- An Ethereum node running locally or remotely. You can use [Infura](https://infura.io/) or [Alchemy](https://www.alchemy.com/) for a remote node.

## Installation

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/arbitrager_uni_sushi_rust.git
    cd arbitrager_uni_sushi_rust
    ```

2. Install dependencies:

    ```sh
    cargo build
    ```

3. Create a `.env` file in the root directory and add your Ethereum node URL:

    ```env
    RPC_URL=
    ```

## Usage

To run the arbitrager program:

```sh
cargo run
```

The program will start sniffing for new blocks and checking for arbitrage opportunities between Uniswap and SushiSwap.

## Testing

To run the tests:
```sh
cargo test
```

## Project Structure
src/arbitrager.rs: Contains the Arbitrager struct and its implementation.
src/block_sniffer.rs: Contains the block sniffer logic that fetches the latest block number and checks for arbitrage opportunities.
src/router.rs: Contains the router for getAmount calls.