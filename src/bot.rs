use std::sync::Arc;

use serenity::async_trait;
use serenity::prelude::*;

use crate::Config;

pub struct Handler {
	config: Arc<Config>,
}

#[async_trait]
impl EventHandler for Handler {}

impl Handler {
	pub fn new(config: Arc<Config>) -> Self {
		Self { config }
	}
}

pub struct Bot {
	pub client: Client,
}

impl Bot {
	pub async fn new(config: Config) -> anyhow::Result<Self> {
		let config = Arc::new(config);

		let handler = Handler::new(config.clone());

		let client = Client::builder(
			&config.token,
			GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT,
		)
		.event_handler(handler)
		.await?;

		Ok(Self { client })
	}

	pub async fn start(&mut self) -> serenity::Result<()> {
		self.client.start().await
	}
}
