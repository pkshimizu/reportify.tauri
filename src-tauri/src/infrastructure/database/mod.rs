pub mod models;
pub mod schema;

use std::sync::{Arc, Mutex};

use anyhow::Result;
use diesel::{prelude::*, sqlite::SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct DatabaseConnection {
    connection: Arc<Mutex<SqliteConnection>>,
}

impl DatabaseConnection {
    pub fn new(database_url: &str) -> Result<Self> {
        let connection = SqliteConnection::establish(database_url)?;
        let connection = Arc::new(Mutex::new(connection));

        // Run migrations
        {
            let mut conn = connection.lock().unwrap();
            conn.run_pending_migrations(MIGRATIONS)
                .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;
        }

        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> Arc<Mutex<SqliteConnection>> {
        Arc::clone(&self.connection)
    }
}
