use std::borrow::Cow;

use crate::dissect::ParsedArgs;
use crate::{Bot, Error};

use serenity::model::channel::{AttachmentType, Message};
use serenity::prelude::*;

pub async fn run(
    _bot: &Bot,
    ctx: &Context,
    msg: &Message,
    args: ParsedArgs<'_>,
) -> Result<(), Error> {
    let ctn = format!("{args:#?}");
    let msg_content = format!(
        "parsed args:```xl\n{}```",
        ctn.replace('`', "\\x60").replace("\\\"", "\\x22")
    );
    let attachment = AttachmentType::Bytes {
        data: Cow::from(ctn.as_bytes()),
        filename: "parseargs.xl".to_owned(),
    };

    if ctn.len() > 1000 {
        msg.channel_id
            .send_message(&ctx, |m| m.add_file(attachment))
            .await?;
    } else {
        msg.channel_id
            .send_message(&ctx, |m| m.content(&msg_content))
            .await?;
    }
    Ok(())
}
