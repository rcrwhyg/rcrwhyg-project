use serde::{Deserialize, Serialize};

/// Metadata for a tool registered in the site toolbox.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolMeta {
    pub id: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub path: &'static str,
}
