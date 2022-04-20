mod commands;
mod config;

pub mod models;

use config::Settings;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use mongodb::options::ClientOptions as MClientOptions;
use mongodb::options::ResolverConfig as MResolverConfig;
use mongodb::Client as MClient;

type Error = Box<dyn ::std::error::Error>;

pub struct Bot {
    pub mongodb_client: MClient,
    config: Settings,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Err(e) = command_handler(self, &ctx, &msg).await {
            eprintln!("Error: {}", e);
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.tag());
    }
}

async fn command_handler(bot: &Bot, ctx: &Context, msg: &Message) -> Result<(), Error> {
    // if the author is bot or is it a direct message
    if msg.author.bot || msg.guild_id.is_none() {
        return Ok(());
    }

    // the bot's user id
    let client_user_id = ctx.http.get_current_user().await?.id.0;

    // prefixes users can use
    let prefixes: [String; 3] = [
        bot.config.prefix.clone(),
        format!("<@{client_user_id}> "),
        format!("<@!{client_user_id}> "),
    ];

    let prefix = prefixes.into_iter().find(|p| msg.content.starts_with(p));

    // if the message doesnt start with the prefixes
    if prefix.is_none() {
        return Ok(());
    }

    // strip the prefix then split by whitespace
    let content = msg.content.strip_prefix(&prefix.unwrap()).unwrap();
    let args: Vec<&str> = content.trim().split_whitespace().collect();

    // run the command
    commands::run(bot, ctx, msg, &args).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize env vars
    dotenv::dotenv().ok();

    let token = std::env::var("EERIE_DISCORD_TOKEN")?;
    let mongodb_uri = std::env::var("EERIE_MONGODB_URI")?;

    // initialize data base
    let mongodb_resolver_cfg = MResolverConfig::cloudflare();
    let mongodb_client_options =
        MClientOptions::parse_with_resolver_config(mongodb_uri, mongodb_resolver_cfg).await?;

    let bot = Bot {
        mongodb_client: MClient::with_options(mongodb_client_options)?,
        config: config::initialize_config()?,
    };

    // initialize discord client
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(bot).await?;

    // login
    client.start().await?;
    Ok(())
}
