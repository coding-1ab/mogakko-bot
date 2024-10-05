use std::fmt::Debug;

use log::error;

pub use bot::*;
pub use config::*;

mod bot;
mod config;
mod db;
mod utils;

pub trait LogUtil<T> {
    fn report_on_error(self) -> Option<T>;
}

impl<R, E: Debug> LogUtil<R> for Result<R, E> {
    fn report_on_error(self) -> Option<R> {
        match self {
            Ok(v) => Some(v),
            Err(err) => {
                error!("{:?}", err);
                None
            }
        }
    }
}
