mod model;
pub use model::Guild;

use serenity::prelude::TypeMapKey;
use sqlx::{Error, Pool, Postgres};

pub struct Database {
    pool: Pool<Postgres>,
}

impl TypeMapKey for Database {
    type Value = Self;
}

impl Database {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_or_get_guild(&self, guild_id: u64) -> Result<Guild, Error> {
        match self.get_guild_by_id(guild_id).await {
            Ok(guild) => Ok(guild),
            Err(err) => {
                if let sqlx::Error::RowNotFound = err {
                    return self.create_guild(guild_id).await;
                }

                Err(err)
            }
        }
    }

    pub async fn create_guild(&self, guild_id: u64) -> Result<Guild, Error> {
        sqlx::query_as!(
            Guild,
            "INSERT INTO guilds (guild_id, disabled_providers) VALUES ($1, '{}') RETURNING *",
            guild_id.to_string()
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_guild_by_id(&self, id: u64) -> Result<Guild, Error> {
        sqlx::query_as!(
            Guild,
            r#"SELECT * FROM guilds WHERE guild_id = $1"#,
            id.to_string()
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_guild_by_id(
        &self,
        id: u64,
        disabled_providers: &Vec<String>,
    ) -> Result<Guild, Error> {
        sqlx::query_as!(
            Guild,
            r#"UPDATE guilds SET disabled_providers = $2 WHERE guild_id = $1 RETURNING *"#,
            id.to_string(),
            disabled_providers
        )
        .fetch_one(&self.pool)
        .await
    }
}
