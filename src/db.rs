use std::sync::Arc;

use sqlx::{Pool, Sqlite};
use time::{Date, Duration, Month};

use crate::Config;

pub struct LeaderboardRecord {
    pub user: u64,
    pub days: u32,
    pub total_duration: Duration,
}

pub struct UserStatistics {
    pub user: u64,
    pub days: u32,
    pub total_duration: Duration,
    pub calendar: Vec<Date>,
}

pub struct Db {
    config: Arc<Config>,
    pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&config.database_url)
            .await?;

        Ok(Self { config, pool })
    }

    // when user joins
    pub async fn joins(user: u64) -> anyhow::Result<()> {
        println!("user {} joined!", user);
        Ok(())
    }

    // when user leaves
    pub async fn leaves(user: u64) -> anyhow::Result<()> {
        println!("user {} left!", user);
        Ok(())
    }

    // get server leaderboard
    pub async fn leaderboard() -> anyhow::Result<Vec<LeaderboardRecord>> {
        todo!()
    }

    // show user statistics
    pub async fn user_statistics() -> anyhow::Result<UserStatistics> {
        todo!()
    }
}
