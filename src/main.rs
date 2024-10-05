use std::env::var;

use dotenvy::dotenv;

use mogakko_bot::{Bot, Config};
use serenity::all::validate_token;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

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

    bot.start().await?;

    Ok(())
}
