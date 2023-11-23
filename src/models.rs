use bigdecimal::BigDecimal;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NativeType {
    pub address: Vec<u8>,
    pub u256: BigDecimal,
    pub block_number: i64,
    pub tx_hash: Vec<u8>,
    pub optional_address: Option<Vec<u8>>,
    pub optional_u256: Option<BigDecimal>,
}

// #[derive(Queryable, Selectable)]
// #[diesel(table_name = crate::schema::types)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct EthType {
//     pub address: Vec<u8>,
//     pub u256: BigDecimal,
//     pub block_number: i64,
//     pub tx_hash: Vec<u8>,
//     pub optional_address: Option<Vec<u8>>,
//     pub optional_u256: Option<BigDecimal>,
// }
