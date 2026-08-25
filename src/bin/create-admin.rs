//! Create the solo admin account from the server shell (not the public web).
//!
//! Usage:
//!   cargo run --features ssr --bin create-admin -- you@example.com 'your-strong-password'
//!
//! Requires DATABASE_URL and sql/auth.sql applied. Fails if an admin already exists.

use std::env;
use std::process::ExitCode;

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> ExitCode {
    dotenvy::dotenv().ok();

    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("用法: create-admin <email> <password>");
        eprintln!(
            "示例: cargo run --features ssr --bin create-admin -- you@example.com 'long-password'"
        );
        return ExitCode::from(2);
    }

    let password = args.pop().unwrap();
    let email = args.pop().unwrap();

    let Ok(url) = env::var("DATABASE_URL") else {
        eprintln!("错误: 未设置 DATABASE_URL");
        return ExitCode::from(1);
    };

    let pool = match PgPoolOptions::new().max_connections(2).connect(&url).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("错误: 无法连接数据库: {err}");
            return ExitCode::from(1);
        }
    };

    match rcrwhyg_server::server::create_admin_account(&pool, &email, &password).await {
        Ok(admin) => {
            println!("已创建管理员: {} (id={})", admin.email, admin.id);
            println!("请使用浏览器打开 /admin/login 登录。");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("错误: {err}");
            ExitCode::from(1)
        }
    }
}
