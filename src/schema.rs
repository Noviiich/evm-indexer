// @generated automatically by Diesel CLI.

diesel::table! {
    blocks (block_hash) {
        base_fee_per_gas -> Text,
        block_hash -> Text,
        chain -> Text,
        difficulty -> Text,
        extra_data -> Text,
        gas_limit -> Text,
        gas_used -> Text,
        logs_bloom -> Text,
        miner -> Text,
        mix_hash -> Text,
        nonce -> Text,
        number -> BigInt,
        parent_hash -> Text,
        receipts_root -> Text,
        sha3_uncles -> Text,
        size -> BigInt,
        state_root -> Text,
        timestamp -> Text,
        total_difficulty -> Text,
        transactions -> BigInt,
    }
}
