mod providers;

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
    sync::atomic::{AtomicUsize, Ordering},
};

struct Handler<'a> {
    providers: [&'a dyn Provider; 1],
    guild_count: AtomicUsize,
}

impl Default for Handler<'_> {
    fn default() -> Self {
        Self {
            providers: [&TiktokProvider {}],
            guild_count: AtomicUsize::new(0),
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
        if msg.author.bot {
            return;
        }

        for provider in self.providers {
            if let Some(url) = provider.match_url(&msg.content) {
                let create_message = provider.new_message(url);
                let map = json::hashmap_to_json_map(create_message.0);
                if let Err(why) = ctx
                    .http
                    .send_message(msg.channel_id.0, &Value::from(map))
                    .await
                {
                    println!("Failed to send message {why}")
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("discord token");

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler::default())
        .await
        .expect("Create client");

    if let Err(why) = client.start().await {
        println!("Client error: {why}");
    }
}
