use ethers::types::{Bytes, H160, H256, U64, U256, H64};

pub fn convert_block_number(block_number: U64) -> u64 {
    block_number.as_u64() as u64
}

pub fn format_hash(h: H256) -> String {
    format!("{:?}", h)
}

pub fn format_bytes(b: &Bytes) -> String {
    serde_json::to_string(b).unwrap_or_default()
}

pub fn format_address(h: H160) -> String {
    format!("{:?}", h)
}

pub fn format_bytes_slice(b: &[u8]) -> String {
    return format!("0x{}", hex::encode(b));
}

pub fn format_number(n: U256) -> String {
    return format!("{}", n.to_string());
}

pub fn format_nonce(h: H64) -> String {
    return format!("{:?}", h);
}