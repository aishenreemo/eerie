mod commands;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use serde::{Deserialize, Serialize};

use mongodb::bson::{doc, oid::ObjectId};
use mongodb::options::ClientOptions as MClientOptions;
use mongodb::options::ResolverConfig as MResolverConfig;
use mongodb::Client as MClient;

type Error = Box<dyn ::std::error::Error>;

pub struct Bot {
    _mongodb_client: MClient,
    config: Configuration,
}

pub struct Configuration {
    prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    discord_id: u64,
    todos: Vec<String>,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(e) = command_handler(self, &ctx, &msg).await {
            eprintln!("Error: {}", e);
        }
        // let users = self.mongodb_client.database("main").collection("users");
        // let user: User = match users
        //     .find_one_and_delete(doc! { "discord_id": msg.author.id.0 as i64 }, None)
        //     .await
        //     .ok()
        //     .unwrap()
        // {
        //     Some(u) => u,
        //     None => User {
        //         id: None,
        //         discord_id: msg.author.id.0,
        //         todos: vec![],
        //     },
        // };

        // let insert_result = users.insert_one(&user, None).await.ok().unwrap();

        // println!("Result: {insert_result:?}\n{user:#?}")
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.tag());
    }
}

fn initialize_config() -> Result<Configuration, Error> {
    Ok(Configuration {
        prefix: std::env::var("EERIE_PREFIX")?,
    })
}

async fn command_handler(bot: &Bot, ctx: &Context, msg: &Message) -> Result<(), Error> {
    if msg.author.bot || msg.guild_id.is_none() {
        return Ok(());
    }

    let client_user_id = ctx.http.get_current_user().await?.id.0;
    let prefixes: [String; 3] = [
        bot.config.prefix.clone(),
        format!("<@{client_user_id}>"),
        format!("<@!{client_user_id}>"),
    ];

    let prefix = prefixes.into_iter().find(|p| msg.content.starts_with(p));

    if prefix.is_none() {
        return Ok(());
    }

    let content = msg.content.strip_prefix(&prefix.unwrap()).unwrap();
    let args: Vec<&str> = content.split_whitespace().collect();

    if args.is_empty() {
        return Ok(());
    }

    commands::run(bot, ctx, msg, args[0]).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv()?;

    let token = std::env::var("EERIE_DISCORD_TOKEN")?;
    let mongodb_uri = std::env::var("EERIE_MONGODB_URI")?;

    let mongodb_resolver_cfg = MResolverConfig::cloudflare();
    let mongodb_client_options =
        MClientOptions::parse_with_resolver_config(mongodb_uri, mongodb_resolver_cfg).await?;

    let bot = Bot {
        _mongodb_client: MClient::with_options(mongodb_client_options)?,
        config: initialize_config()?,
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(bot).await?;

    client.start().await?;
    Ok(())
}
