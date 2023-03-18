use serenity::prelude::TypeMapKey;

pub struct RedisClient;

impl TypeMapKey for RedisClient {
    type Value = redis::Client;
}
