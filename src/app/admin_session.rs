use leptos::prelude::*;

use crate::domain::AdminPublic;
use crate::server::admin_bootstrap_status;

/// Shared admin session for chrome (header) + login/logout pages.
#[derive(Clone, Copy)]
pub struct AdminSession {
    pub admin: RwSignal<Option<AdminPublic>>,
    pub ready: RwSignal<bool>,
}

pub fn provide_admin_session() {
    let admin = RwSignal::new(None::<AdminPublic>);
    let ready = RwSignal::new(false);
    provide_context(AdminSession { admin, ready });

    Effect::new(move |_| {
        leptos::task::spawn_local(async move {
            match admin_bootstrap_status().await {
                Ok(status) => admin.set(status.admin),
                Err(_) => admin.set(None),
            }
            ready.set(true);
        });
    });
}

pub fn use_admin_session() -> AdminSession {
    use_context::<AdminSession>().expect("provide_admin_session() missing")
}

pub fn set_logged_in_admin(session: AdminSession, admin: AdminPublic) {
    session.admin.set(Some(admin));
    session.ready.set(true);
}

pub fn clear_admin_session(session: AdminSession) {
    session.admin.set(None);
    session.ready.set(true);
}
