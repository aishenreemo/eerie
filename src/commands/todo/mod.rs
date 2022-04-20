use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(_bot: &Bot, ctx: &Context, msg: &Message, _args: &[&str]) -> Result<(), Error> {
    msg.channel_id
        .send_message(&ctx, |m| m.content("Not implemented yet!"))
        .await?;
    Ok(())
}
