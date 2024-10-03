use serenity::async_trait;
use serenity::prelude::*;

pub struct Config {
	pub token: String,
}

pub struct Handler {
	config: Config,
}

#[async_trait]
impl EventHandler for Handler {}

impl Handler {
	pub fn new(config: Config) -> Self {
		Self { config }
	}
}

pub struct Bot {
	pub client: Client,
}

impl Bot {
	pub async fn new(config: Config) -> anyhow::Result<Self> {
		let client = Client::builder(
			&config.token,
			GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
		);

		let handler = Handler::new(config);

		let client = client.event_handler(handler).await?;

		Ok(Self { client })
	}

	pub async fn start(&mut self) -> serenity::Result<()> {
		self.client.start().await
	}
}
