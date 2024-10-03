use std::sync::Arc;

use time::{Date, Duration, PrimitiveDateTime};

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
	pub calendar: Vec<(Date, Duration)>,
}

pub struct Db {
	config: Arc<Config>,
}

impl Db {
	pub fn new(config: Arc<Config>) -> Self {
		Self { config }
	}

	// when user joins
	pub async fn joins(user: u64, when: PrimitiveDateTime) {
		todo!()
	}

	// when user leaves
	pub async fn leaves(user: u64, when: PrimitiveDateTime) {
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
