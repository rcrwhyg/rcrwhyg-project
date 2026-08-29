# 部署门禁（Deploy Gating）

> 这一条覆盖一切会改变 **origin** 或 **生产部署** 的动作。
> 即使用户曾一次性授权过本地 commit / build，**也不**默认延伸到 push / tag / 部署 —— 每一项**每一回**都问。

## 范围内（需要用户明确同意）

下列动作**每一回**都要先向用户申请，得到明确"是/做/推"的回复后才能执行：

- `git push` 到 `origin`（任何分支，不只是 master）
- `git push --force` / `--force-with-lease`（会改写远端历史）
- 创建 git tag（`git tag -a vX.Y.Z`）
- 推送 git tag（`git push origin vX.Y.Z` 或 `git push --tags`）
- 删除远端 tag（`git push origin :refs/tags/...`）
- `workflow_dispatch` 触发 `.github/workflows/` 任何 workflow
- 任何 `gh` CLI 改写远端状态的动作（`gh release create/delete`、`gh workflow run` 等）

## 范围外（不需逐次问）

下列动作仍要走本地质量门禁（`./tools/test-local.sh`），但不需要逐次申请：

- 本地 `git commit`（已通过 pre-commit 钩子扫描）
- 本地 `git reset`、`git rebase`、`git branch -d`（仍在本地）
- 本地 `cargo` / `cargo leptos` / 测试 / 静态检查
- 启动本地 dev server 验证

## 执行流程

1. **问**：用 `AskUserQuestion` 或文本提出具体动作（"推 v0.4.0 触发 CD 吗？"），等待明确答复
2. **做**：收到"是/做/推"后，**只做这一项**，不要顺手做其他未授权的事
3. **回报**：执行后给用户回执——commit SHA、tag 名、CD run id、产线 curl 结果（如有）
4. **盯守**：CD 启动后用 `gh run watch` 盯到全绿；如果失败，**不要**自动重推，先看日志、修根因，再问用户

## 错误模式（必须避免）

- ❌ "用户之前让我 push 过了，所以这次也 push" — 错。每次都问
- ❌ "本地门禁全过，CD 流水线会兜底" — 错。本地绿 ≠ 自动部署
- ❌ "我先 push 上去再解释" — 错。先问再做
- ❌ "顺手把上一轮的 force-push 也补上" — 错。逐项申请
- ❌ "tag 没 push 不会触发 CD" — 错。本地 tag 也算仓库状态变化

## 撤销与例外

- **撤回已经错推的 tag / commit**：优先告知用户，由用户决定是否 force-push / 删 tag
- **紧急热修复**：CLAUDE.md 允许 `--no-verify` 走热修通道，但**仅限用户当场明确批准**，事后必须补齐门禁并把现场写在汇报里

## 引用

- `AGENT.md` § 权限边界（高层）
- `rules/git-workflow.md` § 推送流程（操作层）
- `docs/deploy-vps.md`（CD 流水线与生产环境细节）
