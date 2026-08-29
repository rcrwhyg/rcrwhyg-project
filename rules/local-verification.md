# 本地验证 / 用户检视（Local Verification）

> 这一条决定 **AI Agent 何时可以问"可以 commit 吗"或"可以 push 吗"**。
> 跟 [`rules/deploy-gating.md`](deploy-gating.md) 配合：deploy-gating 管"对外动作要问"，本规则管"问之前**自己**要做到哪一步"。

## 核心循环（每一轮都走完，不要跳）

```
改代码 → 本地门禁 → 本地起服务 → 汇报 + 给出本地 URL → 等待检视反馈
   ↑                                                          ↓
   └────────── 改完再回到顶部 ←── 用户检视后给反馈 ←──────────┘
                                                          ↓
                                       用户说"可以 commit / push"
                                                          ↓
                                                  才进 deploy-gating 流程
```

## 步骤硬要求

### 1. 改完一轮后

| 步骤 | 工具 / 命令 | 不能跳的理由 |
|------|------------|-------------|
| 跑全套本地门禁 | `bash tools/test-local.sh` | 提前发现 clippy / test / wasm / 文章检查的回归 |
| `cargo leptos build` | （门禁未跑这一项；额外跑一次） | 提前发现 lightningcss / tailwind / 模板语法问题 |
| 启 dev server | `cargo run --features ssr --bin rcrwhyg-server` | 给用户检视用 |
| 等待 /health 200 | `curl http://127.0.0.1:3000/health` | 确认服务起来 |
| curl 关键路由 | 至少 `curl /`、`/radar`、`/articles/<slug>` | 提前发现 SSR 渲染问题 |

### 2. 汇报模板

每次跑完本地起服务后，给用户的汇报必须包含：

- ✅ / ✅❌ 本地门禁（5/5 ok 或具体哪条挂）
- ✅ cargo leptos build
- ✅ dev server 在 `http://127.0.0.1:3000`
- 当前改动摘要（哪个文件、做了什么、为什么）
- **等用户检视，不主动 commit / push**

### 3. 用户反馈

- 用户可能在浏览器里指出视觉 / 交互问题
- 用户可能指出代码 / 逻辑问题
- 用户可能直接说"OK 提交吧"
- 接到反馈后**回到循环顶部**：改代码 → 重跑门禁 → 重启服务 → 重新汇报 → 等

### 4. 用户说"可以 commit / push" 之后

才进 [`rules/deploy-gating.md`](deploy-gating.md)：

- 仍要 **逐项申请**：commit 行、tag 名、push 范围（master / master+tag / force-push / …）
- 用户确认 → 做 → 汇报 commit SHA / tag / run id

## 错误模式

- ❌ 改完不跑门禁就 commit — 禁止
- ❌ 跑完门禁不起服务就汇报"我改完了你看看" — 禁止。门禁过了 ≠ 能直接看视觉
- ❌ 跳过 curl 关键路由 — SSR 可能在某路由 panic，光跑门禁看不出来
- ❌ 启服务后立刻 commit — 禁止。**必须等用户检视**
- ❌ 用户说"看着行"但没说"commit" — 还是要问一次"那我 commit 了？"
- ❌ 用户说"可以 commit"就直接 push — 还要再问"push + tag 吗"
- ❌ 把"我前面推过类似的"当成"这次也可以" — 每次都问

## 引用

- [`rules/deploy-gating.md`](deploy-gating.md) — 对外动作的批准门禁（push / tag / CD）
- `AGENT.md` § 4 用户主导 — 高层原则
