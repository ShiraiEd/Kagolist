use rapina::database::DatabaseConfig;
use rapina::middleware::RequestLogMiddleware;
use rapina::middleware::TraceIdMiddleware;
use rapina::prelude::*;

mod entity;
mod items;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let db_config = DatabaseConfig::from_env()?;
    Rapina::new()
        .with_tracing(TracingConfig::new())
        .middleware(TraceIdMiddleware::new())
        .middleware(RequestLogMiddleware::new())
        .with_database(db_config)
        .await?
        .with_health_check(true)
        .discover()
        .listen("127.0.0.1:3000")
        .await
}
