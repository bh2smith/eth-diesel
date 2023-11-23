// @generated automatically by Diesel CLI.

diesel::table! {
    types (address, u256) {
        address -> Bytea,
        u256 -> Numeric,
        block_number -> Int8,
        tx_hash -> Bytea,
        optional_address -> Nullable<Bytea>,
        optional_u256 -> Nullable<Numeric>,
    }
}
