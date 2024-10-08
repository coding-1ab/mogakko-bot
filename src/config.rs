use std::num::NonZeroU64;

pub struct Config {
    pub token: String,
    pub vc_id: NonZeroU64,
    pub database_url: String,
}
