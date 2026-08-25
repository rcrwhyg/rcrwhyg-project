# 代码质量门禁

> 适用于 `src/`、`sql/`、`style/` 下的一切代码改动。远程 CI 与 `hooks/` 本地钩子执行同一套标准。

## 硬性门禁（任何一项失败即禁止合并/推送）

| 检查 | 命令 | 说明 |
|------|------|------|
| 格式 | `cargo fmt --all --check` | rustfmt 默认风格 |
| Lint | `cargo clippy --features ssr --all-targets -- -D warnings` | 警告即错误 |
| 单测 | `cargo test --features ssr` | 含软门控 DB 集成测试 |
| wasm | `cargo check --lib --features hydrate --target wasm32-unknown-unknown` | 前端目标可编译 |
| 敏感信息 | `hooks/pre-commit` 扫描 | 不提交密钥/凭据 |

## 编写代码的约定

### 依赖与 feature
- 服务器专用依赖必须 `optional`，只挂在 `ssr` feature 下
- 前端（hydrate）不得引入 `ssr` 依赖，保持 WASM 体积
- 优先 musl 友好依赖（`rustls` 优先，避免纯 glibc 原生库）

### 架构约束
- 保持单一 Leptos 路由树与单一站点外壳
- 服务端逻辑进 `src/server/`，领域模型进 `src/domain/`，视图进 `src/pages/`、`src/components/`
- 主题色走 `style/tokens.css` 变量，不在 Rust 视图里硬编码品牌色

### 测试要求
- 新增/修改 `domain` / `server` 逻辑时**同步补充单元测试**
- 涉及 Postgres 的用例走 `src/server/test_db.rs` 软门控（无 `DATABASE_URL` 时自动跳过，不红）
- 写/管理端 API（`admin_*`）上线前必须有覆盖成功与失败路径的测试（ADR-011）
- 不虚构覆盖率；测试文件与被测代码同提交

### 安全
- 会话/口令逻辑改动必须过 `src/server/password.rs`、`session.rs`、`rate_limit.rs` 既有测试
- 不在源码、日志、公开路由中出现密钥
- 公开接口保留限流中间件，不随意放宽阈值

## 修复门禁失败的流程

1. 读完整报错，定位根因，不做表面绕过（`#[allow(...)]` 需写明理由）
2. 修复后本地重跑 `./tools/test-local.sh`
3. 以独立 `fix:` / `style:` 提交，推送后确认远程 CI 转绿

---
*随项目演进更新；放宽任何一条需先在 ADR 中记录理由。*
