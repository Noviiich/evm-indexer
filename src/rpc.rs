use ethers::{
    providers::{Http, Middleware, Provider}, 
    core::types::Block,
};
use crate::utils::convert_block_number;

pub struct RpcClient {
    provider: Provider<Http>
}

impl RpcClient {
    pub fn new(rpc_url: &str) -> eyre::Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .expect("Failed to create provider from RPC URL");
        Ok(Self { provider })
    }

    pub async fn get_last_block(&self) -> u64 {
        let block_number = self.provider.get_block_number().await;
        match block_number {
            Ok(block_number) => convert_block_number(block_number),
            Err(e) => {
                eprintln!("Error fetching block number: {}", e);
                0
            }
        }
    }

    pub async fn get_block(&self) -> eyre::Result<()> {
        let block_number = self.get_last_block().await;
        let raw_block = self.provider.get_block(block_number).await;
        match raw_block {
            Ok(block) => {

            }
            Err(e) => {

            }
        }
        Ok(())
    }
}