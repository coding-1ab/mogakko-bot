use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};

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
    event_kind: VcEventKind,
}

#[repr(u64)]
pub enum VcEventKind {
    Join = 0,
    Leave,
    Detected,
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
    InvalidKind,
}

impl VcEvent {
    pub fn from_db(value1: u64, value2: u64, value3: u8) -> Result<Self, VcDBError> {
        if value1 != 0 {
            return Err(VcDBError::InvalidId);
        }

        if value2 == 0 {
            return Err(VcDBError::InvalidAt);
        }

        let event_kind = match value3 {
            0 => VcEventKind::Join,
            1 => VcEventKind::Leave,
            2 => VcEventKind::Detected,
            _ => return Err(VcDBError::InvalidKind),
        };

        Ok(Self {
            id: value1,
            at: value2,
            event_kind,
        })
    }
}

impl TerminationEvent {
    pub fn from_db(value3: u8) -> Self {
        Self { exit_code: value3 }
    }
}

impl Db {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    // when user joins
    pub async fn joins(user: u64, when: PrimitiveDateTime) -> anyhow::Result<()> {
        println!("user {} joined!", user);
        Ok(())
    }

    // when user leaves
    pub async fn leaves(user: u64, when: PrimitiveDateTime) -> anyhow::Result<()> {
        println!("user {} left!", user);
        Ok(())
    }

    pub async fn detected(user: u64) -> anyhow::Result<()> {
        println!("user {} is already in vc!", user);
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

    pub async fn on_crash(exit_code: u8) -> anyhow::Result<()> {
        todo!()
    }
}

impl Debug for VcDBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            VcDBError::InvalidId => "Invalid User Id.",
            VcDBError::InvalidAt => "Invalid Timestamp.",
            VcDBError::InvalidKind => "Invalid Event Kind.",
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
