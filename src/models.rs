use crate::types::{Address, Bytes32};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug)]
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

#[derive(Queryable, Selectable, Debug, Serialize, Insertable, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EthType {
    #[diesel(serialize_as = Vec<u8>)]
    pub address: Address,
    pub u256: BigDecimal,
    pub block_number: i64,
    #[diesel(serialize_as = Vec<u8>)]
    pub tx_hash: Bytes32,
    pub optional_address: Option<Vec<u8>>,
    pub optional_u256: Option<BigDecimal>,
}
