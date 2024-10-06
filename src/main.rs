use std::{env::var, io::ErrorKind};

use dotenvy::{dotenv, Error};

use mogakko_bot::{Bot, Config};
use serenity::all::validate_token;
use tracing::info;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match dotenv() {
        Err(Error::Io(e)) if e.kind() == ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
        _ => (),
    }

    init();

    let token = var("DISCORD_TOKEN").expect("Environment Variable DISCORD_TOKEN not found!");
    validate_token(&token)?;

    let config = Config {
        token,
        vc_id: var("CHANNEL_ID")
            .expect("Environment Variable CHANNEL_ID not found!")
            .parse()
            .unwrap(),
        database_url: var("DATABASE_URL")?,
    };

    let mut bot = Bot::new(config).await?;

    info!("Starting bot");

    bot.start().await?;

    Ok(())
}
