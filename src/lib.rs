pub mod commands;
pub mod handlers;
pub mod libs;
pub mod utils;

#[cfg(test)]
pub mod test_helpers;

use std::error::Error as StdError;
use redis::Client as RedisClient;
use std::sync::Arc;

// Type aliases for convenience
pub type Error = Box<dyn StdError + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
pub struct Data {
    pub redis_client: Arc<RedisClient>,
}

#[cfg(test)]
pub mod test {
    use super::*;
    use poise::serenity_prelude::*;

    pub fn create_test_context() -> Context<'static> {
        let redis_client = Arc::new(RedisClient::open("redis://localhost").unwrap());
        let data = Data { redis_client };
        let ctx = Context::dummy().with_data(data);
        ctx
    }
}
