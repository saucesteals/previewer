mod providers;

use dotenv::dotenv;
use providers::{tiktok::TiktokProvider, *};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    json::{self, Value},
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
    Client,
};
use std::env;

struct Handler<'a> {
    providers: [&'a dyn Provider; 1],
}

impl Default for Handler<'_> {
    fn default() -> Self {
        Self {
            providers: [&TiktokProvider {}],
        }
    }
}

#[async_trait]
impl EventHandler for Handler<'_> {
    async fn ready(&self, _ctx: Context, about: Ready) {
        println!("Ready as {}", about.user.name);
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
