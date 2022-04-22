use crate::dissect::ParsedArgs;
use crate::models::Guild;
use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

use mongodb::bson::doc;

pub async fn set_prefix(
    bot: &Bot,
    ctx: &Context,
    msg: &Message,
    prefix: &str,
) -> Result<(), Error> {
    let guilds = bot.mongodb_client.database("main").collection("guilds");
    let mut guild_data = match guilds
        .find_one_and_delete(doc! { "discord_id": msg.guild_id.unwrap().0 as i64 }, None)
        .await?
    {
        Some(u) => u,
        None => Guild {
            id: None,
            discord_id: msg.guild_id.unwrap().0,
            prefix: bot.config.prefix.clone(),
        },
    };

    let content = format!("Set prefix to server: `{prefix}`");
    msg.channel_id
        .send_message(&ctx, |m| m.content(&content))
        .await?;

    guild_data.prefix = prefix.to_owned();
    guilds.insert_one(&guild_data, None).await?;

    Ok(())
}

pub async fn run(
    bot: &Bot,
    ctx: &Context,
    msg: &Message,
    args: ParsedArgs<'_>,
) -> Result<(), Error> {
    if let Some(prefix) = args.flags.get("set") {
        set_prefix(bot, ctx, msg, prefix).await?;
        return Ok(());
    }

    let guilds = bot.mongodb_client.database("main").collection("guilds");
    let guild_id = args
        .positional
        .get(0)
        .cloned()
        .unwrap_or(&msg.guild_id.unwrap().0.to_string())
        .parse::<u64>();

    if guild_id.is_err() {
        msg.channel_id
            .send_message(&ctx, |m| m.content("Expected a guild_id."))
            .await?;
        return Ok(());
    }

    let guild_id = guild_id.ok().unwrap();

    let guild_data = match guilds
        .find_one(doc! { "discord_id": guild_id as i64 }, None)
        .await?
    {
        Some(u) => u,
        None => Guild {
            id: None,
            discord_id: guild_id,
            prefix: bot.config.prefix.clone(),
        },
    };

    let content = format!("Prefix for guild `{guild_id}`: `{}`", guild_data.prefix);
    msg.channel_id
        .send_message(&ctx, |m| m.content(&content))
        .await?;

    Ok(())
}
