use std::str::FromStr;

use alloy_primitives::{hex, Address as AlloyAddress, FixedBytes as AlloyBytes, U256 as AlloyU256};
use bigdecimal::BigDecimal;
use diesel::{
    self,
    data_types::PgNumeric,
    deserialize::{self, FromSql},
    pg::{Pg, PgValue},
    serialize::ToSql,
    sql_types::{Binary, Numeric, SqlType},
    Queryable,
};
use serde::Serialize;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SqlType)]
#[diesel(postgres_type(name = "address"))]
#[diesel(not_sized)]
pub struct Address(pub AlloyAddress);

/// ! WARNING! This function is meant to be used by Diesel
/// for Ethereum address fields encoded in postgres
/// as BYTEA type (since there is no fixed length type)
impl From<Vec<u8>> for Address {
    fn from(value: Vec<u8>) -> Self {
        Self(AlloyAddress::from_slice(value.as_slice()))
    }
}

impl From<Address> for Vec<u8> {
    fn from(value: Address) -> Self {
        value.0.as_slice().to_vec()
    }
}

impl Queryable<Binary, Pg> for Address {
    type Row = Vec<u8>;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row.into())
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = [0u8; 2 + 20 * 2];
        bytes[..2].copy_from_slice(b"0x");
        // Can only fail if the buffer size does not match but we know it is correct.
        hex::encode_to_slice(self.0 .0, &mut bytes[2..]).unwrap();
        // Hex encoding is always valid utf8.
        let s = std::str::from_utf8(&bytes).unwrap();
        serializer.serialize_str(s)
    }
}

impl<DB> ToSql<Binary, DB> for Address
where
    DB: diesel::backend::Backend,
    [u8]: ToSql<Binary, DB>,
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, DB>,
    ) -> diesel::serialize::Result {
        self.0 .0.as_slice().to_sql(out)
    }
}

// impl ToSql<Binary, Pg> for Address {
//     fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
//         self.0 .0.as_slice().to_sql(out)
//     }
// }

// impl FromSql<diesel::sql_types::Nullable<diesel::sql_types::Binary>, Pg> for Address {
//     fn from_sql(
//         bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
//     ) -> deserialize::Result<Self> {
//         Ok(Address::from(bytes.as_bytes().to_vec()))
//     }
// }

// impl FromSql<Address, Pg> for Address {
//     fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
//         Ok(Address::from(bytes.as_bytes().to_vec()))
//     }
// }

impl FromSql<Address, Pg> for Address {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        Ok(Address::from(bytes.as_bytes().to_vec()))
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, SqlType)]
#[diesel(not_sized)]
pub struct Bytes32(pub AlloyBytes<32>);

impl From<Vec<u8>> for Bytes32 {
    fn from(value: Vec<u8>) -> Self {
        Self(AlloyBytes::from_slice(value.as_slice()))
    }
}

impl Queryable<Binary, Pg> for Bytes32 {
    type Row = Vec<u8>;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row.into())
    }
}

impl Serialize for Bytes32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut bytes = [0u8; 2 + 32 * 2];
        bytes[..2].copy_from_slice(b"0x");
        // Can only fail if the buffer size does not match but we know it is correct.
        hex::encode_to_slice(self.0 .0, &mut bytes[2..]).unwrap();
        // Hex encoding is always valid utf8.
        let s = std::str::from_utf8(&bytes).unwrap();
        serializer.serialize_str(s)
    }
}

impl From<Bytes32> for Vec<u8> {
    fn from(value: Bytes32) -> Self {
        value.0.as_slice().to_vec()
    }
}
/// Define Custom U256 type (although this is not really the problem)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, SqlType, Hash)]
#[diesel(postgres_type(name = "U256"))]
pub struct U256(pub AlloyU256);

impl FromSql<Numeric, Pg> for U256 {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let big_decimal: BigDecimal = PgNumeric::from_sql(bytes)?.try_into()?;
        Ok(U256::from(big_decimal))
    }
}

impl From<BigDecimal> for U256 {
    fn from(val: BigDecimal) -> Self {
        U256(AlloyU256::from_str(&val.to_string()).expect("Invalid value"))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use alloy_primitives::{address, fixed_bytes, Address, FixedBytes, I256, U256};
    #[test]
    fn test_primitives() {
        // FixedBytes
        let n: FixedBytes<6> = fixed_bytes!("1234567890ab");
        assert_eq!(n, "0x1234567890ab".parse::<FixedBytes<6>>().unwrap());
        assert_eq!(n.to_string(), "0x1234567890ab");

        // Uint
        let mut n: U256 = "42".parse().unwrap();
        n += U256::from(10);
        assert_eq!(n.to_string(), "52");

        // Signed
        let mut n: I256 = "-42".parse().unwrap();
        n = -n;
        assert_eq!(n.to_string(), "42");

        // Address
        let addr_str = "0x66f9664f97F2b50F62D13eA064982f936dE76657";
        let addr: Address = Address::parse_checksummed(addr_str, None).unwrap();
        assert_eq!(addr, address!("66f9664f97F2b50F62D13eA064982f936dE76657"));
        assert_eq!(addr.to_checksum(None), addr_str);

        // Address checksummed with a custom chain id
        let addr_str = "0x66F9664f97f2B50F62d13EA064982F936de76657";
        let addr: Address = Address::parse_checksummed(addr_str, Some(30)).unwrap();
        assert_eq!(addr, address!("66F9664f97f2B50F62d13EA064982F936de76657"));
        assert_eq!(addr.to_checksum(Some(30)), addr_str);
    }
}
