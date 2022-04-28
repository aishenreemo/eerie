use crate::dissect::ParsedArgs;
use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

pub async fn run(
    _bot: &Bot,
    ctx: &Context,
    msg: &Message,
    args: ParsedArgs<'_>,
) -> Result<(), Error> {
    if args.positional.get(0).is_none() && args.flags.get("code").is_none() {
        msg.channel_id
            .send_message(&ctx, |m| m.content("No script provided"))
            .await?;
        return Ok(());
    }

    let code = args.flags.get("code").unwrap_or(&args.positional[0]);
    let output = format!("```rs\n{code}```");

    msg.channel_id
        .send_message(&ctx, |m| m.content(&output))
        .await?;
    Ok(())
}
