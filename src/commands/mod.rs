mod ping;
mod todo;

use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(bot: &Bot, ctx: &Context, msg: &Message, args: &[&str]) -> Result<(), Error> {
    if args.get(0).is_none() {
        return Ok(());
    };
    match args[0].to_lowercase().as_str() {
        "ping" => ping::run(bot, ctx, msg, args).await,
        "todo" => todo::run(bot, ctx, msg, args).await,
        _ => Ok(()),
    }
}
