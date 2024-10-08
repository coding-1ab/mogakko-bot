use std::sync::Arc;

use sqlx::{Pool, Sqlite};
use time::{format_description::well_known::Iso8601, Date, Duration};

use crate::Config;

type User = u64;

pub struct LeaderboardRecord {
    pub user: User,
    pub days: u32,
    pub total_duration: Duration,
}

pub struct UserStatistics {
    pub rank: u32,
    pub user: User,
    pub days: u32,
    pub total_duration: Duration,
    pub calendar: Vec<Date>,
}

pub struct Db {
    #[allow(dead_code)]
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

    async fn find_lock(&self, user: User) -> anyhow::Result<Option<i64>> {
        let user = user.to_string();

        let lock = sqlx::query_file!("src/queries/find-lock.sql", user)
            .fetch_optional(&self.pool)
            .await?;

        Ok(lock.map(|r| r.id))
    }

    async fn is_first_time_today(&self, user: User) -> anyhow::Result<bool> {
        let user = user.to_string();

        let count = sqlx::query_file!(r#"src/queries/is-first-time-today.sql"#, user)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.count == 0)
    }

    // when user joins
    pub async fn joins(&self, user: User) -> anyhow::Result<bool> {
        let None = self.find_lock(user).await? else {
            return Ok(false);
        };

        let is_first_time_today = self.is_first_time_today(user).await?;

        let user = user.to_string();

        sqlx::query_file!("src/queries/join.sql", user)
            .execute(&self.pool)
            .await?;

        Ok(is_first_time_today)
    }

    /// Invoked when user leaves the voice channel
    ///
    /// Returns `true` if it is the first time the user left the voice channel today.
    /// Otherwise, returns `false`.
    pub async fn leaves(&self, user: User) -> anyhow::Result<bool> {
        let Some(id) = self.find_lock(user).await? else {
            return Ok(false);
        };

        let is_first_time_today = self.is_first_time_today(user).await?;

        sqlx::query_file!("src/queries/leave.sql", id)
            .execute(&self.pool)
            .await?;

        Ok(is_first_time_today)
    }

    pub async fn lookup_saved_participants(&self) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_file!("src/queries/lookup.sql")
            .fetch_all(&self.pool)
            .await?;

        Ok(users
            .into_iter()
            .map(|user| user.user.parse())
            .collect::<Result<Vec<_>, _>>()?)
    }

    // get server leaderboard
    pub async fn leaderboard(&self, limit: u32) -> anyhow::Result<Vec<LeaderboardRecord>> {
        Ok(sqlx::query_file!("src/queries/leaderboard.sql", limit)
            .map(|row| LeaderboardRecord {
                user: row.user.parse().unwrap(),
                days: row.days as u32,
                total_duration: Duration::seconds(row.total_duration),
            })
            .fetch_all(&self.pool)
            .await?)
    }

    // show user statistics
    pub async fn user_statistics(&self, user: User) -> anyhow::Result<Option<UserStatistics>> {
        let user = user.to_string();

        let st = sqlx::query_file!("src/queries/statistics.sql", user)
            .fetch_optional(&self.pool)
            .await?;

        let calendar = sqlx::query_file!("src/queries/statistics-calendar.sql", user)
            .fetch_all(&self.pool)
            .await?;

        Ok(st.map(|st| UserStatistics {
            rank: st.rank as u32,
            user: st.user.parse().unwrap(),
            days: calendar.len() as u32,
            total_duration: Duration::seconds(st.total_duration),
            calendar: calendar
                .into_iter()
                .map(|r| Date::parse(&r.date.unwrap(), &Iso8601::DATE).unwrap())
                .collect(),
        }))
    }
}
