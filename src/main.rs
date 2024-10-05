use std::env::var;

use dotenvy::dotenv;
use mogakko_bot::{Bot, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    let config = Config {
        token: var("DISCORD_TOKEN").expect("Environment Variable DISCORD_TOKEN not found!"),
        vc_id: var("CHANNEL_ID")
            .expect("Environment Variable CHANNEL_ID not found!")
            .parse()
            .unwrap(),
        database_url: var("DATABASE_URL").unwrap(),
    };

    let mut bot = Bot::new(config).await?;

    bot.start().await?;

    Ok(())
}
