use crate::models::EthType;

use {
    crate::models::NativeType,
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

    pub fn get_native_data(&mut self) -> Vec<NativeType> {
        crate::schema::types::dsl::types
            .load(&mut self.client)
            .unwrap()
    }

    pub fn get_eth_data(&mut self) -> Vec<EthType> {
        crate::schema::types::dsl::types
            .load(&mut self.client)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    // use serde_json::json;

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
        let json_string = serde_json::to_string(&data).unwrap();
        println!("Data JSON {:?}", json_string);
        // This doesn't work yet
        // assert_eq!(
        //     json_string,
        //     json!(
        //         [{
        //             "address":"0x92be2f02c94d214f8d38ece700385471d9a66c0a",
        //             "u256":"9999999999999999999999999999999999999999999999",
        //             "block_number":1,
        //             "tx_hash":"0xb44c4e99de65f6a5f4a2162a76241cf858c09ff218f3023a3ac03acc17fea885",
        //             "optional_address":null,
        //             "optional_u256":null
        //         }
        //         ]
        //     )
        // )
    }
}
