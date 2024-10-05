use std::sync::Arc;

use serenity::{
    all::{Channel, ChannelType, Ready, VoiceState},
    async_trait,
    prelude::*,
};

use crate::{db::Db, Config};

const TARGET_CHANNEL_ID: u64 = 1130799520545001482;

pub struct Handler {
    config: Arc<Config>,
    db: Db,
}

impl Handler {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            db: Db::new(config.clone()),
            config,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    // Crash recovery.
    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        let channel = match ctx.http.get_channel(self.config.vc_id.into()).await {
            Ok(Channel::Guild(v)) => v,
            Ok(_) => {
                eprintln!("Specified channel is not from guild!");
                return;
            }
            Err(e) => {
                eprintln!("Invalid channel id! Error: {:?}", e);
                return;
            }
        };

        if ChannelType::Voice != channel.kind {
            eprintln!("Specified channel is not vc!");
            return;
        }

        let Some(members) = channel.members(ctx.cache).report_on_error() else {
            return;
        };
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        let user_id = new.user_id.get();
        let was_in_vc = old
            .and_then(|v| v.channel_id)
            .map(|v| v.get() == self.config.vc_id.get())
            .unwrap_or(false);
        let now_in_vc = new
            .channel_id
            .map(|v| v.get() == self.config.vc_id.get())
            .unwrap_or(false);

        if !was_in_vc && now_in_vc {
            Db::joins(user_id).await.report_on_error();
        }

        if was_in_vc && !now_in_vc {
            Db::leaves(user_id).await.report_on_error();
        }
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
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_VOICE_STATES,
        )
        .event_handler(handler)
        .await?;

        Ok(Self { client })
    }

    pub async fn start(&mut self) -> serenity::Result<()> {
        self.client.start().await
    }
}
