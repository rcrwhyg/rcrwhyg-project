# Git 钩子说明

## 钩子一览

| 钩子 | 时机 | 内容 | 耗时 |
|------|------|------|------|
| `hooks/pre-commit` | `git commit` | 敏感文件拦截、暂存内容凭据扫描、`cargo fmt --check`、文章静态检查（articles 有改动时） | 秒级 |
| `hooks/pre-push` | `git push` | 全量门禁：fmt → clippy `-D warnings` → test → wasm 编译 → 文章检查 | 分钟级 |

## 安装

```bash
./tools/install-hooks.sh
```

钩子以文件形式维护在 `hooks/`，安装即复制到 `.git/hooks/`。钩子逻辑变更需重新运行安装脚本。

## 手动跑全量门禁

```bash
./tools/test-local.sh
```

## 跳过（不推荐）

- `git commit --no-verify`
- `git push --no-verify`

仅限紧急热修复；同一会话内必须补齐门禁，并在提交说明中记录原因。

## 凭据扫描规则（pre-commit）

拦截：`.env*`、私钥（`id_rsa`、`*.pem`、`*.key`、`credentials`）被暂存（`git rm --cached` 移除跟踪除外）。
扫描暂存 diff 新增行中的：带密码的 `postgres://` URL、形如 `api_key/secret/token = 16位以上密钥` 的写法（豁免 `.env.example` 占位模板）。
误报处理：改用占位符或环境变量引用；模板只放 `.env.example`。
