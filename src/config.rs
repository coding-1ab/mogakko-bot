use std::num::NonZeroU64;

pub struct Config {
    pub token: Box<str>,
    pub vc_id: NonZeroU64,
}
