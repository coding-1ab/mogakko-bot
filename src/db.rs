use std::{sync::Arc, u32};

use sqlx::{Pool, Sqlite};
use time::{format_description::well_known::Rfc3339, Date, Duration};

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

    async fn find_lock(&self, user: &str) -> anyhow::Result<Option<i64>> {
        let lock = sqlx::query!(
            r#"select id from vc_activities where user = ? and left is null"#,
            user
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(lock.map(|r| r.id))
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

    /// Invoked when user leaves the voice channel
    ///
    /// Returns `true` if it is the first time the user left the voice channel today.
    /// Otherwise, returns `false`.
    pub async fn leaves(&self, user: String) -> anyhow::Result<bool> {
        let Some(id) = self.find_lock(&user).await? else {
            return Ok(false);
        };

        let now = now_kst();
        let today = now.date().format(&Rfc3339)?;

        let count = sqlx::query!(r#"select count(*) as count from vc_activities where date(joined, '+09:00') = ? and left is not null"#, today)
            .fetch_one(&self.pool)
            .await?;

        let count = count.count;

        let now_str = now.format(&Rfc3339)?;

        sqlx::query!(
            r#"update vc_activities set left = ? where id = ?"#,
            now_str,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(count == 0)
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
