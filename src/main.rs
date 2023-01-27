mod commands;
mod db;
mod providers;

use std::env;

use dotenv::dotenv;
use providers::{tiktok::TiktokProvider, *};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    json::{self, Value},
    model::{
        application::{
            command::{Command, CommandOptionType},
            interaction::{Interaction, InteractionResponseType},
        },
        channel::Message,
        gateway::Ready,
        Permissions,
    },
    prelude::{GatewayIntents, TypeMapKey},
    Client,
};
use sqlx::postgres::PgPoolOptions;

struct Handler;

impl Default for Handler {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".into(),
                "list" => commands::list(&ctx, &command).await,
                "enable" => commands::enable(&ctx, &command).await,
                "disable" => commands::disable(&ctx, &command).await,
                _ => "not implemented :(".into(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Failed to respond to slash command: {why}");
            }
        }
    }

    async fn ready(&self, ctx: Context, about: Ready) {
        println!("Ready as {}", about.user.name);

        let data = ctx.data.read().await;
        let state = data.get::<State>().unwrap();

        let commands = Command::set_global_application_commands(&ctx, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("enable")
                        .description("Enable a provider")
                        .default_member_permissions(Permissions::ADMINISTRATOR)
                        .create_option(|option| {
                            option
                                .name("provider")
                                .description("provider")
                                .required(true)
                                .kind(CommandOptionType::String);
                            for provider in &state.providers {
                                option.add_string_choice(provider.name(), provider.name());
                            }
                            option
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("disable")
                        .description("Disable a provider")
                        .default_member_permissions(Permissions::ADMINISTRATOR)
                        .create_option(|option| {
                            option
                                .name("provider")
                                .description("provider")
                                .required(true)
                                .kind(CommandOptionType::String);
                            for provider in &state.providers {
                                option.add_string_choice(provider.name(), provider.name());
                            }
                            option
                        })
                })
                .create_application_command(|command| {
                    command
                        .name("list")
                        .description("List providers")
                        .default_member_permissions(Permissions::ADMINISTRATOR)
                })
        })
        .await;

        if let Err(err) = commands {
            println!("Error adding commands {err}")
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.guild_id.is_none() {
            return;
        }

        let data = ctx.data.read().await;

        let state = data.get::<State>().unwrap();

        for provider in &state.providers {
            if let Some(url) = provider.match_url(&msg.content) {
                let guild = match state.db.create_or_get_guild(msg.guild_id.unwrap().0).await {
                    Ok(guild) => guild,
                    Err(err) => {
                        println!("Error getting guild: {}", err);
                        return;
                    }
                };

                if guild.disabled_providers.contains(&provider.name()) {
                    continue;
                }

                println!(
                    "[{}] [{}#{}] {}",
                    msg.guild_id.unwrap_or_default(),
                    msg.author.name,
                    msg.author.discriminator,
                    url
                );
                let create_message = provider.new_message(
                    format!("https://www.buymeacoffee.com/saucesteals - {url}").as_str(),
                );
                let map = json::hashmap_to_json_map(create_message.0);
                ctx.http
                    .send_message(msg.channel_id.0, &Value::from(map))
                    .await
                    .ok();

                return;
            }
        }
    }
}

struct State<'a> {
    db: db::Database,
    providers: Vec<&'a dyn Provider>,
}

impl TypeMapKey for State<'static> {
    type Value = Self;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Discord token");

    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("Database URL"))
        .await
        .expect("Connect to database");

    sqlx::migrate!().run(&pool).await.expect("Migrate database");

    let db = db::Database::new(pool);

    let state = State {
        db,
        providers: vec![&TiktokProvider {}],
    };

    let mut client = Client::builder(token, intents)
        .event_handler(Handler::default())
        .await
        .expect("Create client");

    {
        let mut data = client.data.write().await;
        data.insert::<State>(state);
    }

    if let Err(err) = client.start_autosharded().await {
        println!("Client error: {err}");
    }
}
