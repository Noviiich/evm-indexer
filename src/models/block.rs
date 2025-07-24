use ethers::types::{H256, H160, Block};
use serde::{Serialize, Deserialize};
use crate::utils::format::{
    format_hash, 
    format_address,
    format_bytes_slice,
    format_bytes,
    format_number,
    format_nonce,
};
use diesel::prelude::*;
use crate::schema::blocks;

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = blocks)]
pub struct DatabaseBlock {
    pub base_fee_per_gas: String,
    pub chain: String,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub block_hash: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: i64,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: i64,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: String,
    pub transactions: i64,
    // pub uncles: Vec<Option<String>>
}

impl DatabaseBlock {
    pub fn new(block: Block<H256>, chain: &'static str) -> Self {
        let base_fee_per_gas: String = match block.base_fee_per_gas {
            None => String::from("0"),
            Some(base_fee_per_gas) => format_number(base_fee_per_gas),
        };

        let nonce: String = match block.nonce {
            None => String::from("0"),
            Some(nonce) => format_nonce(nonce),
        };

        // let uncles = block
        //     .uncles
        //     .clone()
        //     .into_iter()
        //     .map(|uncle| Some(format_hash(uncle)))
        //     .collect();

        let mix_hash: String = match block.mix_hash {
            None => String::from("0x"),
            Some(mix_hash) => format_hash(mix_hash),
        };

        let block_hash: String = match block.hash {
            None => String::from("0x"),
            Some(hash) => format_hash(hash),
        };

        let number: i64 = match block.number {
            None => 0,
            Some(number) => number.as_u64() as i64,
        };

        let size: i64 = match block.size {
            None => 0,
            Some(size) => size.as_u64() as i64,
        };

        let total_difficulty: String = match block.total_difficulty {
            None => String::from("0x"),
            Some(total_difficulty) => format_number(total_difficulty),
        };

        let miner: String = match block.author {
            None => format_address(H160::zero()),
            Some(author) => format_address(author),
        };

       Self {
            base_fee_per_gas,
            chain: chain.to_owned(),
            difficulty: format_number(block.difficulty),
            extra_data: format_bytes(&block.extra_data),
            gas_limit: format_number(block.gas_limit),
            gas_used: format_number(block.gas_used),
            block_hash,
            logs_bloom: format_bytes_slice(block.logs_bloom.unwrap().as_bytes()),
            miner,
            mix_hash,
            nonce,
            number,
            parent_hash: format_hash(block.parent_hash),
            receipts_root: format_hash(block.receipts_root),
            sha3_uncles: format_hash(block.uncles_hash),
            size,
            state_root: format_hash(block.state_root),
            timestamp: format_number(block.timestamp),
            total_difficulty,
            transactions: block.transactions.len() as i64,
            // uncles,
        }
    }
}