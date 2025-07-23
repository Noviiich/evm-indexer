use ethers::{
    types::U64
};

pub fn convert_block_number(block_number: U64) -> u64 {
    block_number.as_u64() as u64
}