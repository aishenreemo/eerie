use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::model::timestamp::Timestamp;
use serenity::prelude::*;

pub async fn run(_bot: &Bot, ctx: &Context, msg: &Message) -> Result<(), Error> {
    let latency = Timestamp::now().unix_timestamp() - msg.timestamp.unix_timestamp();
    let message_content = format!("🏓Pong! Latency: `{latency}ms`");
    msg.channel_id
        .send_message(&ctx, |m| m.content(&message_content))
        .await?;
    Ok(())
}
