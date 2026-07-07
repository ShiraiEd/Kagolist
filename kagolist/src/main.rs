use rapina::prelude::*;
use rapina::middleware::RequestLogMiddleware;
use rapina::schemars;

#[derive(Serialize, JsonSchema)]
struct MessageResponse {
    message: String,
}

#[get("/")]
async fn hello() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "Hello from Rapina!".to_string(),
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    let router = Router::new()
        .get("/", hello);
    
    Rapina::new()
        .with_tracing(TracingConfig::new())
        .middleware(RequestLogMiddleware::new())
        .with_health_check(true)
        .router(router)
        .listen("127.0.0.1:3000")
        .await
}
