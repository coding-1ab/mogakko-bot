use std::env;

use dotenvy::dotenv;
use mogakko_bot::{Bot, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    let config = Config {
        token: env::var("DISCORD_TOKEN").expect("Environment Variable DISCORD_TOKEN not found!"),
        vc_id: env::var("CHANNEL_ID")
            .expect("Environment Variable CHANNEL_ID not found!")
            .parse()
            .unwrap(),
        database_url: env::var("DATABASE_URL").unwrap(),
    };

    let mut bot = Bot::new(config).await?;

    bot.start().await?;

    Ok(())
}
