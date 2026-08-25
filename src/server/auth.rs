use leptos::prelude::*;

use crate::domain::{AdminBootstrap, AdminPublic};

#[server(AdminBootstrapStatus)]
pub async fn admin_bootstrap_status() -> Result<AdminBootstrap, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::auth_http::{read_session_token, require_pool};
        use super::session::{admin_from_session_token, count_admins};

        let pool = require_pool()?;
        let has_admin = count_admins(&pool)
            .await
            .map_err(ServerFnError::new)?
            > 0;
        let token = read_session_token().await;
        let admin = match token.as_deref() {
            Some(raw) => admin_from_session_token(&pool, raw)
                .await
                .map_err(ServerFnError::new)?
                .map(|row| AdminPublic {
                    id: row.id,
                    email: row.email,
                }),
            None => None,
        };
        Ok(AdminBootstrap {
            has_admin,
            logged_in: admin.is_some(),
            admin,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(AdminBootstrap {
            has_admin: false,
            logged_in: false,
            admin: None,
        })
    }
}

#[server(AdminLogin)]
pub async fn admin_login(email: String, password: String) -> Result<AdminPublic, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use http::header::HeaderValue;

        use super::auth_http::{append_set_cookie, client_meta, require_pool};
        use super::rate_limit::shared_auth_limiter;
        use super::session::{build_session_cookie, create_session, verify_admin_credentials};

        let meta = client_meta().await;
        shared_auth_limiter()
            .check_auth_ip(&meta.ip)
            .map_err(|_| ServerFnError::new("登录尝试过于频繁，请稍后再试。"))?;

        let pool = require_pool()?;
        let admin = verify_admin_credentials(&pool, &email, &password)
            .await
            .map_err(ServerFnError::new)?;
        let Some(admin) = admin else {
            return Err(ServerFnError::new("邮箱或密码错误"));
        };
        let (raw, max_age) =
            create_session(&pool, admin.id, Some(meta.ip.as_str()), meta.ua.as_deref())
                .await
                .map_err(ServerFnError::new)?;
        let cookie = build_session_cookie(&raw, max_age);
        append_set_cookie(
            HeaderValue::from_str(&cookie.to_string())
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        )?;
        Ok(AdminPublic {
            id: admin.id,
            email: admin.email,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = (email, password);
        Err(ServerFnError::new("ssr only"))
    }
}

#[server(AdminLogout)]
pub async fn admin_logout() -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use http::header::HeaderValue;

        use super::auth_http::{append_set_cookie, read_session_token};
        use super::session::{clear_session_cookie, delete_session_by_token};

        if let Some(pool) = use_context::<sqlx::PgPool>() {
            if let Some(token) = read_session_token().await {
                let _ = delete_session_by_token(&pool, &token).await;
            }
        }
        let cookie = clear_session_cookie();
        append_set_cookie(
            HeaderValue::from_str(&cookie.to_string())
                .map_err(|e| ServerFnError::new(e.to_string()))?,
        )?;
        Ok(())
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(())
    }
}

#[server(RequireAdmin)]
pub async fn require_admin() -> Result<AdminPublic, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use super::auth_http::{read_session_token, require_pool};
        use super::session::admin_from_session_token;

        let pool = require_pool()?;
        let Some(token) = read_session_token().await else {
            return Err(ServerFnError::new("未登录"));
        };
        let admin = admin_from_session_token(&pool, &token)
            .await
            .map_err(ServerFnError::new)?;
        admin
            .map(|row| AdminPublic {
                id: row.id,
                email: row.email,
            })
            .ok_or_else(|| ServerFnError::new("未登录或会话已过期"))
    }
    #[cfg(not(feature = "ssr"))]
    {
        Err(ServerFnError::new("ssr only"))
    }
}
