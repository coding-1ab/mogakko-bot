use std::{sync::Arc, u32};

use sqlx::{Pool, Sqlite};
use time::{Date, Duration, Month, OffsetDateTime, UtcOffset};

use crate::{
    utils::{is_valid_time, now_kst},
    Config,
};

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

    async fn find_lock(&self, user: String) -> anyhow::Result<Option<u32>> {
        let lock: Option<(u32,)> =
            sqlx::query_as(r#"select id from vc_activities where user = ? and left = null"#)
                .bind(user)
                .fetch_optional(&self.pool)
                .await?;

        Ok(lock.map(|(id,)| id))
    }

    // when user joins
    pub async fn joins(&self, user: String) -> anyhow::Result<()> {
        if !is_valid_time(now_kst()) {
            return Ok(());
        }

        sqlx::query(r#"insert into vc_activities (user) values (?)"#)
            .bind(user)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // when user leaves
    pub async fn leaves(user: String) -> anyhow::Result<()> {
        // sqlx::query(r#"update vc_activities set left = ? where "#)
    }

    pub async fn lookup_saved_participants(&self) -> anyhow::Result<Vec<String>> {
        todo!()
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
