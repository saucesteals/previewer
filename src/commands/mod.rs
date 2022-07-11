use crate::providers::Provider;
use crate::State;

use serenity::model::application::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::prelude::Context;

fn str_providers<'a>(providers: &Vec<&'a dyn Provider>) -> String {
    providers
        .iter()
        .fold(String::new(), |acc, &provider| acc + &provider.name())
}

fn is_valid_provider<'a>(providers: &Vec<&'a dyn Provider>, provider: &String) -> bool {
    providers.iter().any(|p| p.name().eq(provider))
}

pub async fn providers(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let data = ctx.data.read().await;
    let state = data.get::<State>().unwrap();

    let guild_id = command.guild_id.unwrap();

    match state.db.create_or_get_guild(guild_id.0).await {
        Ok(guild) => format!(
            "Disabled providers: {}",
            guild.disabled_providers.join(", ")
        ),
        Err(_) => "Failed to get guild".into(),
    }
}

pub async fn enable(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected provider option")
        .resolved
        .as_ref()
        .expect("Expected provider string");

    let provider = match options {
        CommandDataOptionValue::String(provider) => provider,
        _ => return "Invalid provider".into(),
    };

    let data = ctx.data.read().await;
    let state = data.get::<State>().unwrap();

    if !is_valid_provider(&state.providers, &provider) {
        return format!(
            "Invalid provider. Use one of: {}",
            str_providers(&state.providers)
        );
    }

    let guild_id = command.guild_id.unwrap();

    let guild = match state.db.create_or_get_guild(guild_id.0).await {
        Ok(guild) => guild,
        Err(_) => return "Failed to get guild".into(),
    };

    if !guild.disabled_providers.contains(&provider.to_string()) {
        return "Provider is already enabled.".into();
    }

    let mut disabled_providers = guild.disabled_providers.clone();
    disabled_providers.retain(|p| p != provider);

    match state
        .db
        .update_guild_by_id(guild_id.0, &disabled_providers)
        .await
    {
        Ok(_) => "Provider enabled.".into(),
        Err(err) => {
            println!("Error updating guild: {err}");
            "Unexpected error".into()
        }
    }
}

pub async fn disable(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let options = command
        .data
        .options
        .get(0)
        .expect("Expected provider option")
        .resolved
        .as_ref()
        .expect("Expected provider string");

    let provider = match options {
        CommandDataOptionValue::String(provider) => provider,
        _ => return "Invalid provider".into(),
    };

    let data = ctx.data.read().await;
    let state = data.get::<State>().unwrap();

    if !is_valid_provider(&state.providers, &provider) {
        return format!(
            "Invalid provider. Use one of: {}",
            str_providers(&state.providers)
        );
    }

    let guild_id = command.guild_id.unwrap();

    let guild = match state.db.create_or_get_guild(guild_id.0).await {
        Ok(guild) => guild,
        Err(_) => return "Failed to get guild".into(),
    };

    if guild.disabled_providers.contains(&provider.to_string()) {
        return "Provider is already disabled.".into();
    }

    let mut disabled_providers = guild.disabled_providers.clone();
    disabled_providers.push(provider.into());

    match state
        .db
        .update_guild_by_id(guild_id.0, &disabled_providers)
        .await
    {
        Err(_) => "Failed to update guild".into(),
        Ok(_) => "Provider disabled.".into(),
    }
}
