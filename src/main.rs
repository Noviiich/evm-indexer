
use evm_indexer::rpc::RpcClient;

const RPC_URL: &str = "https://ethereum-sepolia-rpc.publicnode.com";
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let rpc_client = RpcClient::new(RPC_URL)?;
    let block_number = rpc_client.get_last_block().await;
    println!("Current block number: {}", block_number);

    Ok(())
}