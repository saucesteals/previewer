mod providers;
mod db;

use sqlx::{postgres::PgPoolOptions};
use dotenv::dotenv;
use providers::{tiktok::TiktokProvider, *};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    json::{self, Value},
    model::{
        channel::Message,
        gateway::{Activity, Ready},
        guild::Guild,
    },
    prelude::GatewayIntents,
    Client,
};
use std::{
    env,
    sync::{atomic::{AtomicUsize, Ordering}}
};


struct Handler<'a> {
    providers: Vec<&'a dyn Provider>,
    guild_count: AtomicUsize,
    db: db::Database,
}

impl Handler<'_> {
    fn new(database:db::Database) -> Self {
        Self {
            providers: vec![&TiktokProvider {}],
            guild_count: AtomicUsize::default(),
            db: database,
        }
    }
}

#[async_trait]
impl EventHandler for Handler<'_> {
    async fn guild_create(&self, ctx: Context, _guild: Guild) {
        let guild_count = self.guild_count.fetch_add(1, Ordering::SeqCst);

        if guild_count % 10 == 0 {
            ctx.set_activity(Activity::watching(format!("{} guilds", guild_count)))
                .await;
        }
    }

    async fn ready(&self, ctx: Context, about: Ready) {
        println!("Ready as {}", about.user.name);

        let guild_count = about.guilds.len();
        self.guild_count.store(guild_count, Ordering::SeqCst);
        ctx.set_activity(Activity::watching(format!("{} guilds", guild_count)))
            .await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.guild_id.is_none() {
            return;
        }   

        let guild = tokio::sync::OnceCell::new();

        let guild_id = msg.guild_id.unwrap();

        let get_guild = || guild.get_or_init(|| self.db.create_or_get_guild(guild_id.0));

        let is_valid_provider = |provider: &str| self.providers.iter().any(|p| p.name() == provider);
        let invalid_provider = || {
            msg.channel_id.say(&ctx, 
                format!("Invalid provider. Please use one of: {}", 
                    self.providers.iter().map(|p| p.name()).collect::<Vec<String>>().join(", ")
                )
            )
        };
  
        if msg.content.starts_with("!enable") {
            let provider = match msg.content.split_whitespace().nth(1) {
                Some(provider) => provider,
                None => {
                    msg.channel_id.say(&ctx, "Please specify a provider to enable.").await.ok();
                    return
                }
            };

            if !is_valid_provider(provider) {
                invalid_provider().await.ok();
                return
            }

            let guild = match get_guild().await {
                Ok(guild) => guild,
                Err(err) => {
                    println!("Error getting guild: {}", err);
                    return
                }
            };

            if !guild.disabled_providers.contains(&provider.to_string()) {
                msg.channel_id.say(&ctx, "Provider is already enabled.").await.ok();
                return
            }

            let mut disabled_providers = guild.disabled_providers.clone();
            disabled_providers.retain(|p| p != provider);

            match self.db.update_guild_by_id(guild_id.0, &disabled_providers).await {
                Ok(_) => {
                    msg.channel_id.say(&ctx, "Provider enabled.").await.ok();
                },
                Err(err) => {
                    println!("Error updating guild: {}", err);
                    return
                }
            };
            return
        } else if msg.content.starts_with("!disable") {
            let provider = match msg.content.split_whitespace().nth(1) {
                Some(provider) => provider,
                None => {
                    msg.channel_id.say(&ctx, "Please specify a provider to disable.").await.ok();
                    return
                }
            };

            if !is_valid_provider(provider) {
                invalid_provider().await.ok();
                return
            }

            let guild = match get_guild().await {
                Ok(guild) => guild,
                Err(err) => {
                    println!("Error getting guild: {}", err);
                    return
                }
            };

            if guild.disabled_providers.contains(&provider.to_string()) {
                msg.channel_id.say(&ctx, "Provider is already disabled.").await.ok();
                return
            }

            let mut disabled_providers = guild.disabled_providers.clone();
            disabled_providers.push(provider.to_string());
            
            match self.db.update_guild_by_id(guild_id.0, &disabled_providers).await {
                Ok(_) => {
                    msg.channel_id.say(&ctx, "Provider disabled.").await.ok();
                },
                Err(err) => {
                    println!("Error updating guild: {}", err);
                    msg.channel_id.say(&ctx, "Error disabling provider.").await.ok();
                }
            };
            return
        } else if msg.content.starts_with("!providers") {
            let guild = match get_guild().await {
                Ok(guild) => guild,
                Err(err) => {
                    println!("Error getting guild: {}", err);
                    return
                }
            };

            msg.channel_id.say(&ctx, format!("Disabled providers: {}", guild.disabled_providers.join(", "))).await.ok();
            return
        }

    
        for provider in &self.providers {    
            if let Some(url) = provider.match_url(&msg.content) {
                let guild = match get_guild().await {
                    Ok(guild) => guild,
                    Err(err) => {
                        println!("Error getting guild: {}", err);
                        return
                    }
                };
    
                if guild.disabled_providers.contains(&provider.name()) {
                    continue;
                }

                let create_message = provider.new_message(url);
                let map = json::hashmap_to_json_map(create_message.0);
                ctx.http
                    .send_message(msg.channel_id.0, &Value::from(map))
                    .await.ok();
            }
            }
        }
    }


#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Discord token");

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&env::var("DATABASE_URL").expect("Database URL")).await.expect("Connect to database");

    sqlx::migrate!().run(&pool).await.expect("Migrate database");

    let mut client = Client::builder(token, intents)
        .event_handler(Handler::new(db::Database::new(pool)))
        .await
        .expect("Create client");

    if let Err(err) = client.start().await {
        println!("Client error: {err}");
    }

    
}
