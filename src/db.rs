use crate::models::EthType;

use {
    crate::models::NativeType,
    crate::schema::*,
    diesel::{pg::PgConnection, prelude::*},
    dotenvy::dotenv,
    std::env,
};

pub struct Database {
    pub client: PgConnection,
}

impl Database {
    pub fn new(db_url: &str) -> Self {
        Self {
            client: PgConnection::establish(db_url)
                .unwrap_or_else(|_| panic!("Error connecting to Diesel Client")),
        }
    }

    pub fn new_from_env() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Self::new(&database_url)
    }

    pub fn add_row(&mut self, record: EthType) {
        let result = diesel::insert_into(crate::schema::types::dsl::types)
            .values(record.clone())
            .on_conflict((types::address, types::u256))
            .do_update()
            .set(record)
            .execute(&mut self.client)
            .unwrap();
        println!("Added {} record", result);
    }

    // pub fn save_nft(&mut self, nft: &Nft) {
    //     let result = diesel::insert_into(nfts::dsl::nfts)
    //         .values(nft)
    //         .on_conflict((nfts::contract_address, nfts::token_id))
    //         .do_update()
    //         .set(nft)
    //         .execute(&mut self.client);
    //     handle_insert_result(result, 1, format!("save_nft {:?}", nft))
    // }

    pub fn get_native_data(&mut self) -> Vec<NativeType> {
        types::dsl::types.load(&mut self.client).unwrap()
    }

    pub fn get_eth_data(&mut self) -> Vec<EthType> {
        types::dsl::types.load(&mut self.client).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Address;
    use serde_json::json;

    use super::*;

    #[test]
    fn test_get_native() {
        let mut db = Database::new_from_env();
        let data = db.get_native_data();
        println!("Data {:?}", data);
    }

    #[test]
    fn test_get_eth() {
        let mut db = Database::new_from_env();
        let data = db.get_eth_data();
        println!("Data Debug: {:?}", data);
        let json_value = serde_json::to_value(&data).unwrap();
        assert_eq!(
            json_value,
            json!(
                [{
                    "address":"0x92be2f02c94d214f8d38ece700385471d9a66c0a",
                    "u256":"9999999999999999999999999999999999999999999999",
                    "block_number":1,
                    "tx_hash":"0xb44c4e99de65f6a5f4a2162a76241cf858c09ff218f3023a3ac03acc17fea885",
                    "optional_address":null,
                    "optional_u256":null
                },
                {
                    "address":"0x0000000000000000000000000000000000000000",
                    "u256":"9999999999999999999999999999999999999999999999",
                    "block_number":1,
                    "tx_hash":"0xb44c4e99de65f6a5f4a2162a76241cf858c09ff218f3023a3ac03acc17fea885",
                    "optional_address":null,
                    "optional_u256":null
                }
                ]
            )
        );

        println!("As JSON String: {}", serde_json::to_string(&data).unwrap());
    }

    #[test]
    fn test_add_record() {
        let mut db = Database::new_from_env();
        let mut data = db.get_eth_data()[0].clone();
        data.address = Address(alloy_primitives::Address::ZERO);
        db.add_row(data);

        assert!(db.get_eth_data().len() > 1);
    }
}
