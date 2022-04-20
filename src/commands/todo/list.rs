use crate::models::User;
use crate::{Bot, Error};

use serenity::model::channel::Message;
use serenity::prelude::*;

use mongodb::bson::doc;

pub async fn run(bot: &Bot, ctx: &Context, msg: &Message, _args: &[&str]) -> Result<(), Error> {
    let users = bot.mongodb_client.database("main").collection("users");

    let user = match users
        .find_one(doc! { "discord_id": msg.author.id.0 as i64 }, None)
        .await?
    {
        Some(u) => u,
        None => User {
            id: None,
            discord_id: msg.author.id.0,
            todos: vec![],
        },
    };

    if user.todos.is_empty() {
        msg.channel_id
            .send_message(&ctx, |m| m.content("You don't have any todos!."))
            .await?;

        Ok(())
    } else {
        let mut out = "".to_owned();
        for (i, todo) in user.todos.iter().enumerate() {
            out.push_str(&format!("{}. {todo}\n", i + 1));
        }

        let msg_content = format!("TODO LIST:\n{out}");
        msg.channel_id
            .send_message(&ctx, |m| m.content(&msg_content))
            .await?;

        Ok(())
    }
}
