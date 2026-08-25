use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminPublic {
    pub id: i64,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdminBootstrap {
    pub has_admin: bool,
    pub logged_in: bool,
    pub admin: Option<AdminPublic>,
}
