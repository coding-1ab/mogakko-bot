use std::env::var;

use dotenvy::dotenv;
use libsqlite3_sys::SQLITE_VERSION;

use mogakko_bot::{Bot, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;

    println!("{:?}", String::from_utf8_lossy(SQLITE_VERSION));

    let config = Config {
        token: var("DISCORD_TOKEN").expect("Environment Variable DISCORD_TOKEN not found!"),
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
