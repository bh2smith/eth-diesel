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
        println!("Data {:?}", data);
    }
}
