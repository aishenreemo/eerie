use crate::models::User;
use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

use mongodb::bson::doc;

pub async fn run(bot: &Bot, ctx: &Context, msg: &Message, args: &[&str]) -> Result<(), Error> {
    let users = bot.mongodb_client.database("main").collection("users");

    if args.get(2).is_none() {
        msg.channel_id
            .send_message(&ctx, |m| m.content("Not enough arguments."))
            .await?;
        return Ok(());
    };

    // temporarily remove it from the database or make a new entry
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

    let todo = args[2..].join(" ").to_string();
    let msg_content = format!("Added to todo list: `{todo}`");
    msg.channel_id
        .send_message(&ctx, |m| m.content(&msg_content))
        .await?;

    user.todos.push(todo);

    users.insert_one(&user, None).await?;
    Ok(())
}
