use std::env::var;

use dotenvy::dotenv;
use serenity::all::ChannelId;
use tokio_cron::{daily, Job, Scheduler};

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
        database_url: var("DATABASE_URL")?,
    };

    let vc_id = config.vc_id;
    let mut bot = Bot::new(config).await?;
    let http = bot.client.http.clone();
    let cache1 = bot.client.cache.clone();
    let cache2 = bot.client.cache.clone();
    let db = bot.db.clone();

    let mut scheduler = Scheduler::utc();
    scheduler.add(Job::named("six", daily("18"), move || async {
        http.get_channel(ChannelId::new(vc_id.get()))
            .await
            .unwrap()
            .guild()
            .unwrap()
            .members(cache1)
            .unwrap()
            .into_iter()
            .for_each(|member| db.joins(member.user.id.get().to_string()));
    }));

    scheduler.add(Job::named("ten", daily("22"), move || async {
        http.get_channel(ChannelId::new(vc_id.get()))
            .await
            .unwrap()
            .guild()
            .unwrap()
            .members(cache2)
            .unwrap()
            .into_iter()
            .for_each(|member| db.leaves(member.user.id.get().to_string()));
    }));

    bot.start().await?;

    Ok(())
}
