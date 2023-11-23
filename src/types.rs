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
