# Git 工作流规范

> 本项目托管于阿里云效 Codeup（`master` 分支），本地门禁与远程 CI 双重把关。

## 分支策略

- `master`：主分支，只接受通过全部门禁的提交
- 个人站点为单人维护，允许直接向 `master` 提交；较大改动建议用 `feature/<topic>` 分支再合并

## 质量门禁（必须全部通过）

| 时机 | 门禁 | 工具 |
|------|------|------|
| 提交前 | 敏感文件/凭据扫描、`cargo fmt`、文章静态检查 | `hooks/pre-commit` |
| 推送前 | 格式 + clippy(-D warnings) + 测试 + wasm 编译 + 文章检查 | `hooks/pre-push` |
| 推送后 | 远程 CI 全绿 | 云效 Flow / GitHub Actions（见 `docs/quality-gates.md`） |

- 首次克隆后运行 `./tools/install-hooks.sh` 安装钩子
- 本地手动跑全套：`./tools/test-local.sh`
- **禁止**在门禁失败时使用 `--no-verify` 绕过（真正的紧急情况除外，事后必须补齐）

## 提交规范

### 提交信息格式

```
<type>(<scope>): <subject>

<body 可选>

<footer 可选>
```

### 类型

- `feat`：新功能 / 新页面 / 新工具
- `fix`：缺陷修复
- `content`：文章 / 文案内容（博客 posts 或公众号 articles）
- `docs`：文档、ADR
- `style`：`cargo fmt` 等纯格式调整
- `refactor`：重构
- `test`：测试相关
- `chore`：构建、钩子、CI、依赖维护

### 原则

- 一次提交只做一件事；格式修复（`cargo fmt`）单独成一次提交
- 涉及 AI 协作产出的提交，保留 `Co-Authored-By` 尾注
- **永不提交**：`.env`、私钥、口令、token（pre-commit 会拦截）

## 推送流程（AI Agent 必须遵守）

1. 本地 `./tools/test-local.sh` 通过（pre-push 钩子会再跑一次）
2. `git push`
3. 推送后**盯紧远程 CI**：云效流水线 / Actions 全绿才算完成；失败必须查看日志、修复根因、重新推送，不能以本地通过代替远程验证

## 密钥与敏感信息

- 本地配置只放 `.env`（已 gitignore），模板见 `.env.example`
- 历史中若发现已泄露的凭据：立即轮换该凭据，再评估是否需要清理历史
