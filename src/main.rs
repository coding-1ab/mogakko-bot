use mogakko_bot::{Bot, Config};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let token =
		env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

	let config = Config { token };

	let mut bot = Bot::new(config).await?;

	bot.start().await?;

	Ok(())
}
