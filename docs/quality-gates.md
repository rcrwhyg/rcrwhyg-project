# 质量门禁总览

> 代码质量与文章质量的统一门禁定义。原则：**本地钩子、手动命令、远程 CI 三者执行同一套检查**，任何一层失败都不得发布。

## 一、代码质量门禁

### 检查项

| # | 检查 | 命令 | 失败处理 |
|---|------|------|----------|
| 1 | 格式 | `cargo fmt --all --check` | `cargo fmt --all` 后重跑 |
| 2 | Lint | `cargo clippy --features ssr --all-targets -- -D warnings` | 修复警告，禁止无脑 `#[allow]` |
| 3 | 单元测试 | `cargo test --features ssr` | 修复用例或实现；DB 用例软门控 |
| 4 | 前端编译 | `cargo check --lib --features hydrate --target wasm32-unknown-unknown` | 修复 hydrate 目标错误 |
| 5 | 敏感信息 | `hooks/pre-commit` 内建扫描 | 移除凭据；只允许 `.env.example` 占位 |

### 执行层

| 层 | 时机 | 覆盖 | 实现 |
|----|------|------|------|
| 本地-轻 | `git commit` | 检查 1、5（+ 文章静态检查） | `hooks/pre-commit` |
| 本地-全 | `git push` / 手动 | 检查 1–5 + 文章 | `hooks/pre-push`、`tools/test-local.sh` |
| 远程 | 每次推送 | 检查 1–5 + 文章 | 云效 Flow（`yunxiao/flow.yml`）/ GitHub Actions（`.github/workflows/ci.yml`） |
| 发布 | 部署前 | `cargo leptos build --release` + musl 二进制构建成功 | 见 `docs/build-musl.md` |

### 约定

- 远程门禁是**最终裁决**：本地通过但远程失败，必须修到远程绿（日志为准）
- 部署只接受门禁全绿的 `master` 提交
- 放宽任何门禁需在 `docs/adr/` 记录理由

## 二、文章质量门禁

### 自动静态门禁（`tools/check-articles.sh`）

| # | 检查 | 失败示例 |
|---|------|----------|
| 1 | 首行一级标题 | 文件以引用块开头 |
| 2 | 摘要行 `> **摘要**` | 缺摘要 |
| 3 | 「## 参考资料」章节且为纯 URL 格式 | 参考资料用 `[文字](链接)` |
| 4 | 版权声明 | 缺版权行 |

执行层与代码门禁相同：pre-commit（articles 有改动时）、pre-push、远程 CI。

### 内容门禁（人工 + AI 清单）

- 清单定义：`rules/content-quality.md`
- 每篇发布前逐条核对，意见记入 `docs/article-reviews.md`
- 用户确认是发布的前置条件 —— AI 不得自行发布

## 三、一图流

```
写代码/写文章
   │
   ▼
git commit ── pre-commit（轻）── 失败 → 修复
   │
   ▼
本地全量 ./tools/test-local.sh（建议）
   │
   ▼
git push ── pre-push（全）── 失败 → 修复
   │
   ▼
远程 CI（云效/Actions）── 失败 → 看日志修复重推
   │
   ▼
代码：可部署            文章：进入人工审核 → 用户确认 → 公众号发布
```

---
*门禁代码：`hooks/`、`tools/check-articles.sh`、`.github/workflows/`、`yunxiao/`。改动门禁本身需要提交说明理由。*
