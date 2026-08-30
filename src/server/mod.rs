mod about;
mod articles;
mod auth;
mod home;
mod lab;
mod markdown;
mod music;
mod ping;
mod radar;

pub use articles::{
    ArticleListView, ArticleMeta, ArticleRendered, CollectionPlaceholder, get_site_article,
    list_site_articles,
};
pub use auth::{admin_bootstrap_status, admin_login, admin_logout, require_admin};
pub use home::{RecentItem, recent_items};
pub use ping::{db_status, server_ping};

pub use about::{AboutContent, get_about};
pub use lab::{LabEntry, list_lab};
pub use music::{MusicEntry, list_music};
pub use radar::{RadarEntry, list_radar};

#[cfg(feature = "ssr")]
mod auth_http;
#[cfg(feature = "ssr")]
mod health;
#[cfg(feature = "ssr")]
mod password;
#[cfg(feature = "ssr")]
mod rate_limit;
#[cfg(feature = "ssr")]
mod redirects;
#[cfg(feature = "ssr")]
mod session;
#[cfg(feature = "ssr")]
mod sse;
#[cfg(feature = "ssr")]
mod state;
#[cfg(feature = "ssr")]
mod ws;

#[cfg(all(test, feature = "ssr"))]
pub mod test_db;

#[cfg(feature = "ssr")]
pub use health::health;
#[cfg(feature = "ssr")]
pub use rate_limit::global_rate_limit_middleware;
#[cfg(feature = "ssr")]
pub use redirects::{blog_redirect_index, blog_redirect_slug};
#[cfg(feature = "ssr")]
pub use session::{AdminRow, create_admin as create_admin_account};
#[cfg(feature = "ssr")]
pub use sse::sse_heartbeat;
#[cfg(feature = "ssr")]
pub use state::AppState;
#[cfg(feature = "ssr")]
pub use ws::ws_echo;
