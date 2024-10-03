use dotenvy::dotenv;
use mogakko_bot::{Bot, Config};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	dotenv()?;

	let config = Config {
		token: env::var("DISCORD_TOKEN")?,
	};

	let mut bot = Bot::new(config).await?;

	bot.start().await?;

	Ok(())
}
