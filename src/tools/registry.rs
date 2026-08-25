use crate::domain::ToolMeta;

/// Static toolbox registry. Add a route in `app` when registering a tool.
pub fn all_tools() -> Vec<ToolMeta> {
    vec![ToolMeta {
        id: "echo",
        title: "Echo Placeholder",
        summary: "示例工具占位，验证注册表与路由约定。",
        path: "/tools/echo",
    }]
}

pub fn find_tool(id: &str) -> Option<ToolMeta> {
    all_tools().into_iter().find(|t| t.id == id)
}
