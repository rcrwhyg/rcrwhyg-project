use std::net::SocketAddr;

use axum::{Router, routing::get};
use rcrwhyg_server::get_wechat_access_token;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    // 加载环境变量
    dotenvy::dotenv().ok();

    // 连接数据库
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ 数据库连接成功！");

    let test = Router::new()
        .route(
            "/db_version",
            get(move || async move {
                // 简单查询一下 PG 版本
                let row: (String,) = sqlx::query_as("SELECT version()")
                    .fetch_one(&pool)
                    .await
                    .unwrap();
                format!("Database version: {:?}", row.0)
            }),
        )
        .route(
            "/wechat_token",
            get(|| async {
                get_wechat_access_token("wx7433536eb1186906", "0e833f1386373787a727867c666e4e73")
                    .await
                    .unwrap_or_else(|e| e)
            }),
        );

    // 构建路由
    let app = Router::new()
        .route("/", get(|| async { "Hello from Rust Server!" }))
        .nest("/test", test);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
