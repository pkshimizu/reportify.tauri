pub mod entities;
pub mod migration;

use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection as SeaOrmDatabaseConnection};

pub struct DatabaseConnection {
    connection: SeaOrmDatabaseConnection,
}

impl DatabaseConnection {
    pub async fn new(database_url: &str) -> Result<Self> {
        let connection = Database::connect(database_url).await?;

        // Run migrations
        Migrator::up(&connection, None).await?;

        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> &SeaOrmDatabaseConnection {
        &self.connection
    }
}
