use std::sync::Arc;

use crate::Config;

pub struct Db {
	config: Arc<Config>,
}

impl Db {
	pub fn new(config: Arc<Config>) -> Self {
		Self { config }
	}
}
