#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use std::net::SocketAddr;

    use axum::{Router, routing::get};
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use rcrwhyg_server::app::*;
    use rcrwhyg_server::get_wechat_access_token;
    // use sqlx::postgres::PgPoolOptions;
    use tokio::net::TcpListener;

    println!("Hello, world!");

    // 加载环境变量
    dotenvy::dotenv().ok();

    // 连接数据库
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&database_url)
    //     .await?;

    // println!("✅ 数据库连接成功！");

    let test = Router::new()
        // .route(
        //     "/db_version",
        //     get(move || async move {
        //         // 简单查询一下 PG 版本
        //         let row: (String,) = sqlx::query_as("SELECT version()")
        //             .fetch_one(&pool)
        //             .await
        //             .unwrap();
        //         format!("Database version: {:?}", row.0)
        //     }),
        // )
        .route(
            "/wechat_token",
            get(|| async {
                get_wechat_access_token("wx7433536eb1186906", "0e833f1386373787a727867c666e4e73")
                    .await
                    .unwrap_or_else(|e| e)
            }),
        );

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    // 构建路由
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        // .route("/", get(|| async { "Hello from Rust Server!" }))
        .nest("/test", test)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // 启动服务器
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    log!("🚀 Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
