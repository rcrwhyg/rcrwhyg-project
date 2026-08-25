//! Server-side admin sessions (cookie holds raw token; DB stores SHA-256 hex).

use chrono::{Duration, Utc};
use cookie::{Cookie, SameSite};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use super::password::{hash_password, verify_password};
use argon2::password_hash::rand_core::{OsRng, RngCore};

pub const SESSION_COOKIE: &str = "rcrwhyg_session";
pub const MIN_PASSWORD_LEN: usize = 12;

#[derive(Clone, Debug)]
pub struct AdminRow {
    pub id: i64,
    pub email: String,
}

pub fn session_ttl_hours() -> i64 {
    std::env::var("SESSION_TTL_HOURS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(72)
}

pub fn cookie_secure() -> bool {
    match std::env::var("COOKIE_SECURE").ok().as_deref() {
        Some("0") | Some("false") | Some("FALSE") => false,
        Some("1") | Some("true") | Some("TRUE") => true,
        // Local leptos watch is usually http://127.0.0.1 — Secure cookies would not stick.
        _ => std::env::var("LEPTOS_ENV").ok().as_deref() == Some("PROD"),
    }
}

pub fn hash_session_token(raw: &str) -> String {
    let digest = Sha256::digest(raw.as_bytes());
    hex::encode(digest)
}

pub fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn build_session_cookie(raw_token: &str, max_age_secs: i64) -> Cookie<'static> {
    let mut builder = Cookie::build((SESSION_COOKIE, raw_token.to_string()))
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(cookie::time::Duration::seconds(max_age_secs));
    if cookie_secure() {
        builder = builder.secure(true);
    }
    builder.build()
}

pub fn clear_session_cookie() -> Cookie<'static> {
    let mut builder = Cookie::build((SESSION_COOKIE, ""))
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(cookie::time::Duration::seconds(0));
    if cookie_secure() {
        builder = builder.secure(true);
    }
    builder.build()
}

pub async fn count_admins(pool: &PgPool) -> Result<i64, String> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM admins")
        .fetch_one(pool)
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_admin(pool: &PgPool, email: &str, password: &str) -> Result<AdminRow, String> {
    if password.len() < MIN_PASSWORD_LEN {
        return Err(format!("密码至少 {MIN_PASSWORD_LEN} 位"));
    }
    let email = email.trim().to_lowercase();
    if email.is_empty() || !email.contains('@') {
        return Err(String::from("邮箱无效"));
    }
    if count_admins(pool).await? > 0 {
        return Err(String::from("管理员已存在，禁止重复初始化"));
    }

    let password_hash = hash_password(password)?;
    let row = sqlx::query_as::<_, (i64, String)>(
        r#"
        INSERT INTO admins (email, password_hash)
        VALUES ($1, $2)
        RETURNING id, email
        "#,
    )
    .bind(&email)
    .bind(&password_hash)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(AdminRow {
        id: row.0,
        email: row.1,
    })
}

pub async fn verify_admin_credentials(
    pool: &PgPool,
    email: &str,
    password: &str,
) -> Result<Option<AdminRow>, String> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        email: String,
        password_hash: String,
    }

    let email = email.trim().to_lowercase();
    let row = sqlx::query_as::<_, Row>(
        "SELECT id, email, password_hash FROM admins WHERE email = $1 LIMIT 1",
    )
    .bind(&email)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    let Some(row) = row else {
        return Ok(None);
    };

    if verify_password(password, &row.password_hash)? {
        Ok(Some(AdminRow {
            id: row.id,
            email: row.email,
        }))
    } else {
        Ok(None)
    }
}

pub async fn create_session(
    pool: &PgPool,
    admin_id: i64,
    ip: Option<&str>,
    user_agent: Option<&str>,
) -> Result<(String, i64), String> {
    let raw = generate_session_token();
    let token_hash = hash_session_token(&raw);
    let hours = session_ttl_hours();
    let expires_at = Utc::now() + Duration::hours(hours);

    // Single active session per admin (kick previous).
    sqlx::query("DELETE FROM admin_sessions WHERE admin_id = $1")
        .bind(admin_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(
        r#"
        INSERT INTO admin_sessions (admin_id, token_hash, expires_at, user_agent, ip)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(admin_id)
    .bind(&token_hash)
    .bind(expires_at)
    .bind(user_agent)
    .bind(ip)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((raw, hours * 3600))
}

pub async fn admin_from_session_token(
    pool: &PgPool,
    raw_token: &str,
) -> Result<Option<AdminRow>, String> {
    if raw_token.is_empty() {
        return Ok(None);
    }
    let token_hash = hash_session_token(raw_token);

    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        email: String,
    }

    let row = sqlx::query_as::<_, Row>(
        r#"
        SELECT a.id, a.email
        FROM admin_sessions s
        INNER JOIN admins a ON a.id = s.admin_id
        WHERE s.token_hash = $1 AND s.expires_at > NOW()
        LIMIT 1
        "#,
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(row.map(|r| AdminRow {
        id: r.id,
        email: r.email,
    }))
}

pub async fn delete_session_by_token(pool: &PgPool, raw_token: &str) -> Result<(), String> {
    let token_hash = hash_session_token(raw_token);
    sqlx::query("DELETE FROM admin_sessions WHERE token_hash = $1")
        .bind(&token_hash)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::test_db::shared_pool;

    #[test]
    fn token_hash_is_stable_hex() {
        let h = hash_session_token("abc");
        assert_eq!(h.len(), 64);
        assert_eq!(h, hash_session_token("abc"));
        assert_ne!(h, hash_session_token("abcd"));
    }

    #[tokio::test]
    async fn db_admins_table_when_auth_sql_applied() {
        let Some(pool) = shared_pool().await else {
            eprintln!("skip db_admins_table_when_auth_sql_applied: DATABASE_URL unavailable");
            return;
        };
        let result = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM admins")
            .fetch_one(pool)
            .await;
        assert!(
            result.is_ok(),
            "admins table missing — apply sql/auth.sql: {:?}",
            result.err()
        );
    }
}
