mod eval;
mod parseargs;
mod ping;
mod prefix;
mod todo;

use crate::dissect::ParsedArgs;
use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(
    bot: &Bot,
    ctx: &Context,
    msg: &Message,
    args: ParsedArgs<'_>,
) -> Result<(), Error> {
    match args.command.to_lowercase().as_str() {
        "ping" => ping::run(bot, ctx, msg, args).await,
        "todo" => todo::run(bot, ctx, msg, args).await,
        "parseargs" => parseargs::run(bot, ctx, msg, args).await,
        "prefix" => prefix::run(bot, ctx, msg, args).await,
        "eval" => eval::run(bot, ctx, msg, args).await,
        _ => Ok(()),
    }
}
