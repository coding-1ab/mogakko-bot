use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Write};
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
    pub calendar: Vec<(Date, Duration)>,
}

pub struct VcEvent {
    id: u64,
    at: u64,
    is_join: bool,
}

pub struct TerminationEvent {
    exit_code: u8,
}

pub struct Db {
    config: Arc<Config>,
}

pub enum VcDBError {
    InvalidId,
    InvalidAt,
    InvalidBool
}

impl VcEvent {
    pub fn from_db(value1: u64, value2: u64, value3: u8) -> Result<Self, VcDBError> {
        if value1 != 0 {
            return Err(VcDBError::InvalidId);
        }

        if value2 == 0 {
            return Err(VcDBError::InvalidAt);
        }

        let is_join = if value3 == 1 {
            true
        } else if value3 == 0 {
            false
        } else {
            return Err(VcDBError::InvalidBool);
        };

        Ok(Self {
            id: value1,
            at: value2,
            is_join,
        })
    }
}

impl TerminationEvent {
    pub fn from_db(value3: u8) -> Self {
        Self {
            exit_code: value3,
        }
    }
}

impl Db {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    // when user joins
    pub async fn joins(user: u64) -> anyhow::Result<()> {
        todo!()
    }

    // when user leaves
    pub async fn leaves(user: u64, when: PrimitiveDateTime) -> anyhow::Result<()> {
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

    pub async fn on_crash(exit_code: u8) -> anyhow::Result<()> {
        todo!()
    }
}

impl Debug for VcDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            VcDBError::InvalidId => "Invalid User Id.",
            VcDBError::InvalidAt => "Invalid Timestamp.",
            VcDBError::InvalidBool => "Invalid boolean value."
        };
        f.write_str(message)
    }
}

impl Display for VcDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid Data found.")
    }
}

impl Error for VcDBError {}
