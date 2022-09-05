use std::error::Error;

#[async_trait::async_trait]
pub trait Handler<Request: Send + Sync>: Send + Sync {
    type Response;

    async fn execute(&self, request: Request) -> Result<Self::Response, Box<dyn Error>>;
}
