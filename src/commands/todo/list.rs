use crate::{Bot, Error};
// use crate::models::User;

use serenity::model::channel::Message;
use serenity::prelude::*;

// use mongodb::doc;

pub async fn run(bot: &Bot, ctx: &Context, msg: &Message, args: &[&str]) -> Result<(), Error> {
    msg.channel_id.send_message(&ctx |m| m.content("Not implemented yet!")).await?
    Ok(())
}
