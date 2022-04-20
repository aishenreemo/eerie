mod add;
mod remove;
mod list;

use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(bot: &Bot, ctx: &Context, msg: &Message, args: &[&str]) -> Result<(), Error> {
    if args.get(1).is_none() {
        return Ok(())
    }

    match args[1] {
        "add" => add::run(bot, ctx, msg, args).await,
        "list" => list::run(bot, ctx, msg, args).await,
        "remove" => remove::run(bot, ctx, msg, args).await,
        _ => Ok(()),
    }
}
