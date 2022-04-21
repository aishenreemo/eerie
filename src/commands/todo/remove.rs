use crate::dissect::ParsedArgs;
use crate::models::User;
use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

use mongodb::bson::doc;

pub async fn run(
    bot: &Bot,
    ctx: &Context,
    msg: &Message,
    args: ParsedArgs<'_>,
) -> Result<(), Error> {
    let users = bot.mongodb_client.database("main").collection("users");

    if args.positional.get(1).is_none() {
        msg.channel_id
            .send_message(&ctx, |m| m.content("Not enough arguments."))
            .await?;
        return Ok(());
    };

    let mut user = match users
        .find_one_and_delete(doc! { "discord_id": msg.author.id.0 as i64 }, None)
        .await?
    {
        Some(u) => u,
        None => User {
            id: None,
            discord_id: msg.author.id.0,
            todos: vec![],
        },
    };

    let index = args.positional[1].parse::<usize>();

    if index.is_err() {
        let err_msg = format!("Expected an integer got `{}`", args.positional[1]);
        msg.channel_id
            .send_message(&ctx, |m| m.content(&err_msg))
            .await?;
        return Ok(());
    };

    let index = index.unwrap();

    if index == 0 {
        msg.channel_id
            .send_message(&ctx, |m| m.content("0 is invalid."))
            .await?;
        return Ok(());
    }

    let index = index - 1;
    let todo = user.todos.get(index as usize);

    if todo.is_none() {
        let err_msg = format!("Nothing to remove at index `{index}`");
        msg.channel_id
            .send_message(&ctx, |m| m.content(&err_msg))
            .await?;
        return Ok(());
    }

    let todo = todo.unwrap();
    let msg_content = format!("Removed from todo list: `{todo}`");

    msg.channel_id
        .send_message(&ctx, |m| m.content(&msg_content))
        .await?;

    user.todos.remove(index);

    users.insert_one(&user, None).await?;
    Ok(())
}
