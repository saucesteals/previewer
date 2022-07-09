

#[derive(sqlx::FromRow)]
pub struct Guild { 
    pub id: i64,
    pub guild_id: String,
    pub disabled_providers: Vec<String>,
}