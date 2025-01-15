#[cfg(test)]
pub mod test {
    use crate::{Context, Data, Error};
    use redis::Client as RedisClient;
    use std::sync::Arc;

    pub fn create_test_context() -> Context<'static> {
        let redis_client = Arc::new(RedisClient::open("redis://localhost").unwrap());
        let data = Data { redis_client };
        let ctx = Context::dummy().with_data(data);
        ctx
    }
}
