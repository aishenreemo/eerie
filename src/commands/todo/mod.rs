mod add;
mod list;
mod remove;

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
    if args.positional.get(0).is_none() {
        return Ok(());
    }

    match args.positional[0] {
        "add" => add::run(bot, ctx, msg, args).await,
        "list" => list::run(bot, ctx, msg, args).await,
        "remove" => remove::run(bot, ctx, msg, args).await,
        _ => Ok(()),
    }
}
