use diesel::RunQueryDsl;
use diesel::prelude::*;

use crate::{
    models::block::DatabaseBlock
};

pub struct Database {
    conn: SqliteConnection,
}

impl Database {
    pub async fn new(
        db_url: &str,
    ) -> eyre::Result<Self> {
        let conn = SqliteConnection::establish(db_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
        Ok(Self { conn })
    }

    pub fn save_block(
        &mut self, 
        block: &DatabaseBlock
    ) -> eyre::Result<()> {
        use crate::schema::blocks;

        diesel::insert_into(blocks::table)
            .values(block)
            .execute(&mut self.conn)
            .expect("Error saving block");
        Ok(())
    }
}