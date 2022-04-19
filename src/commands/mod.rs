mod ping;

use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(bot: &Bot, ctx: &Context, msg: &Message, cmd: &str) -> Result<(), Error> {
    match cmd.to_lowercase().as_str() {
        "ping" => ping::run(bot, ctx, msg).await,
        _ => Ok(()),
    }
}
