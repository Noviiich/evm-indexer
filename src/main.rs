
use evm_indexer::{
    rpc::RpcClient,
    db::Database,
    models::block::DatabaseBlock,
};

const RPC_URL: &str = "https://ethereum-sepolia-rpc.publicnode.com";
const CHAIN: &str = "sepolia";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Конфигурация
    let cfg = evm_indexer::config::Config::from_env()?;

    // Подключение к блокчейну
    let rpc_client = RpcClient::new(RPC_URL)?;

    // Подключение к БД
    let mut db = Database::new(&cfg.db_url).await?;

    // получение номера блока
    let block_number = rpc_client.get_last_block().await;
    // получение блока
    let block = rpc_client.get_block(block_number).await?;

    let db_block = DatabaseBlock::new(block, CHAIN);
    println!("Database block details: {:?}", db_block);

    db.save_block(&db_block)
        .expect("Failed to save block to database");

    Ok(())
}