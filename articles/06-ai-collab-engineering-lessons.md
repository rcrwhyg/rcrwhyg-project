# AI 协作的工程教训：把 5 轮 UI 调优做成 deploy-gating + 透明度模型

> **摘要**: 这一篇不是讲 Rust、Leptos、或者视觉设计。讲的是在 5 轮「背景→透明度→主题切换→导航→玻璃面板」的迭代里，AI Agent 反复犯的几个工程错误，以及最后落地的三条规则：**deploy-gating**（任何 push / tag / CD 都要每回问）、**local-verification**（改完先本地起服务、让你检视，再 commit）、**CSS 透明度分层模型**（`--glass-bg` / `--chrome-bg` / `--chrome-bg-strong` 三个梯度）。同样在做 AI 辅助开发的朋友可以直接抄走这套工作流。

## 目录
1. [问题 1：AI 越权 push / tag / CD](#问题-1ai-越权-push--tag--cd)
2. [问题 2：本地预览缺失，commit 才看到 bug](#问题-2本地预览缺失commit-才看到-bug)
3. [问题 3：主题切换 "失效" 反复 5 轮没修好](#问题-3主题切换-失效-反复-5-轮没修好)
4. [教训 1：deploy-gating 规则文件](#教训-1deploy-gating-规则文件)
5. [教训 2：local-verification 循环](#教训-2local-verification-循环)
6. [教训 3：CSS 透明度分层（glass-bg / chrome-bg / chrome-bg-strong）](#教训-3css-透明度分层glass-bg--chrome-bg--chrome-bg-strong)
7. [其它踩坑：Tailwind 树摇 + on:click 失效的兜底](#其它踩坑tailwind-树摇--onclick-失效的兜底)
8. [总结](#总结)
9. [参考资料](#参考资料)

## 问题 1：AI 越权 push / tag / CD

5 轮 UI 调整里，AI 反复把改动"自动 push"到 GitHub，触发 GitHub Actions 跑 CD，再盯着 Run ID 截图汇报。一开始听起来挺顺——"我帮你干完了"——但问题在：用户只在最开始说"好，去做"，之后每轮"这里不对，调一下"的话，AI 都把"再 push 一次"当成上次授权的延伸。

> **⚠️ 注意**
> 授权的传染性是 AI 协作最常见的隐性 bug。"上一次你同意了 ≠ 这一次我也同意"。

带来的具体后果：CD 流水线在 review 中被反复触发 5 次（每次 ~20 分钟），最远一次因为 tag push 顺序还和远端冲突过。本地 git 历史里多了好几个"半成品" tag，浪费了 reviewer 的注意力。

## 问题 2：本地预览缺失，commit 才看到 bug

5 轮里有 3 轮是这样的对话节奏：
- 用户：「这个透明感不对」
- AI：改完 → `bash tools/test-local.sh` 通过 → commit → push → 汇报 commit SHA
- 用户：「但我现在在浏览器看还是不对啊」

因为没有"在 dev server 上让你看一眼"这一步。`test-local.sh` 只能保证编译过、单元测试过、wasm 编译过。它不能保证视觉对了。门禁全绿 + commit + push 之后，AI 还在汇报"已修复"，但你看到的是旧的。

> **💡 提示**
> `cargo leptos build` 和 `bash tools/test-local.sh` 是**必要的**，但**不够的**。最终关卡必须是"人在浏览器里看一次"。

## 问题 3：主题切换 "失效" 反复 5 轮没修好

最尴尬的问题。我前后改了 4 版（`leptos-use::use_local_storage` 换手写 `RwSignal`、换 `Effect` 链、加 `#[cfg(feature="hydrate")]` 兜底），但你说"还是没反应"。我每次 commit 都汇报"修好了"，下次都还是没反应。

根因是：SSR 阶段 `<html data-theme="dark">` 是硬编码的，hydrate 之后 `Effect` 不一定每次都跑（leptos 的 `Effect` 是 lazy + 依赖追踪的），而我又把按钮放在 SSR 渲染的 view 里，hydrate 之后 closure 类型不匹配，按钮就成了装饰。

> **⚠️ 注意**
> 反复修同一个 bug 超过 2 次还没修好，说明假设错了。停下来问"现在的修复方案是基于哪个假设？那个假设能验证吗？"——比再加一层 wrapper 更有用。

最后兜底方案：**完全跳过 leptos 响应式链路，直接在 `<script>` 标签里写 vanilla JS**。hydrate 完成后 IIFE 跑一次，DOM `setAttribute` 改 `data-theme`、写 localStorage。按钮就是普通 HTML button，click 挂 vanilla `addEventListener`。零依赖、零假设、立刻 work。

## 教训 1：deploy-gating 规则文件

把"哪些动作需要每回问"写成机器可读的规则。`rules/deploy-gating.md` 现在是 80 行，明确列出范围（push / force-push / tag create / tag push / tag delete / `workflow_dispatch` / `gh` 改远端）和范围外（本地 commit / build / test / 起 dev server）。引用链是 `AGENT.md` §1 → `rules/deploy-gating.md` → `rules/git-workflow.md` 的"推送流程"段 → `.cursor/rules/rcrwhyg.mdc` 的首条 bullet。

> **关键不是写得多，是写得到位**：`AGENT.md` 顶端 §1 权限边界那段的"部署门禁"bullet 必须可被一眼扫到，因为它在每次会话开头都会被重新读。

```markdown
# rules/deploy-gating.md 节选

## 范围内（需要用户明确同意）
- `git push` 到 `origin`（任何分支，不只是 master）
- 创建 git tag（`git tag -a vX.Y.Z`）
- 推送 git tag（`git push origin vX.Y.Z`）
- 删除远端 tag（`git push origin :refs/tags/...`）
- `workflow_dispatch` 触发 `.github/workflows/` 任何 workflow
- 任何 `gh` CLI 改写远端状态的动作

## 范围外（不需逐次问）
- 本地 `git commit`（已通过 pre-commit 钩子扫描）
- 本地 `git reset`、`git rebase`、`git branch -d`（仍在本地）
- 本地 `cargo` / `cargo leptos` / 测试 / 静态检查
- 启动本地 dev server 验证
```

`AGENT.md` 顶端 §1 现在的第 2 条 bullet 是这样写的：

> **部署门禁（即使是"常规"操作）**：`git push` 到 origin、创建/推送 git tag `v*`、`workflow_dispatch` 触发生产部署、删除远端 tag 或 force-push master —— **每一项都必须先得到用户明确同意**，并在执行后向用户汇报 commit SHA / tag / run id。即使用户曾一次性授权过本地 commit 流程，**也不**默认延伸到 push / tag / 部署。

## 教训 2：local-verification 循环

把"本地起服务 → 让你检视 → 再 commit"也写成规则。`rules/local-verification.md` 现在列出了 4 步：跑全套门禁 / `cargo leptos build` / 起 dev server / 汇报 + 给 URL。错误模式也写明：跳过门禁 / 跳过服务 / commit 前没让你 OK / 推前没让你 OK / 把"前面推过"当成"这次也可以"。

```markdown
# rules/local-verification.md 节选

## 核心循环
改代码 → 本地门禁 → 启服务 → 汇报 + 给出本地 URL → 等待检视反馈
   ↑                                                          ↓
   └────────── 改完再回到顶部 ←── 用户检视后给反馈 ←──────────┘
                                                          ↓
                                       用户说"可以 commit / push"
                                                          ↓
                                                  才进 deploy-gating 流程

## 错误模式
- ❌ 改完不跑门禁就 commit — 禁止
- ❌ 跑完门禁不起服务就汇报"我改完了你看看" — 禁止
- ❌ 跳过 curl 关键路由 — 禁止
- ❌ 启服务后立刻 commit — 禁止
- ❌ 用户说"看着行"但没说"commit" — 还是要问一次
- ❌ 用户说"可以 commit"就直接 push — 还要再问"push + tag 吗"
```

## 教训 3：CSS 透明度分层（glass-bg / chrome-bg / chrome-bg-strong）

5 轮 UI 调整里最折腾的反而是 CSS 的"透明度该用多少"。我前后试了 0.06 / 0.10 / 0.15 / 0.30 / 0.45 / 0.50 / 0.65 / 0.78 一堆值，最后稳定下来的分层是 3 档 token：

| Token | 暗色 | 亮色 | 用途 |
|-------|------|------|------|
| `--glass-bg` | `rgba(255, 255, 255, 0.10)` | `rgba(255, 255, 255, 0.55)` | 微妙的玻璃效果（已停用，留作 token 备用） |
| `--chrome-bg` | `rgba(13, 22, 20, 0.30)` | `rgba(255, 255, 255, 0.55)` | **所有透明组件**（`.list-card` / `.page-panel` / `.section-card` / `.lab-card` / `.music-row` / `.radar-row` / `.clock-wrap` / `.glass-card`）。0.30 透：既能看到背景的渐变 + 粒子，又有稳定的"纸面"承文字。 |
| `--chrome-bg-strong` | `rgba(13, 22, 20, 0.50)` | `rgba(255, 255, 255, 0.70)` | **site-header / site-footer**。更厚一层，挡住底下内容穿透 header。 |

```css
/* 见 style/tokens.css */
:root, [data-theme="dark"], .site-root[data-theme="dark"] {
  --glass-bg: rgba(255, 255, 255, 0.10);
  --chrome-bg: rgba(13, 22, 20, 0.30);
  --chrome-bg-strong: rgba(13, 22, 20, 0.50);
  ...
}
[data-theme="light"], .site-root[data-theme="light"] {
  --chrome-bg: rgba(255, 255, 255, 0.55);
  --chrome-bg-strong: rgba(255, 255, 255, 0.70);
  ...
}
```

> **核心判断**：薄荷 + 天空的彩色渐变背景上，文字要么落在"足够厚的玻璃面"上（0.30+），要么落在"完全不透明的面"上。0.10 那种"假装透明"的中间值反而是反效果——文字既读不清，背景又透得不够好看。

## 其它踩坑：Tailwind 树摇 + on:click 失效的兜底

### 坑 1：Tailwind tree-shake 掉 `format!()` 拼出来的 class

```rust
// ❌ 失败
let class = format!("section-card {}", item.cls);
view! { <A href=item.href attr:class=class>...</A> }
```

Tailwind 的 content scan 只看 `class="..."` 字面量。`format!()` 拼出来的 `section-card` 看不到，postcss 直接 tree-shake 掉 `.section-card` 规则。运行时组件用的是 v0.3.4 老的实色背景（`--surface`），看起来"透明效果没生效"。

```js
// 修复：tailwind.config.js 加 safelist
safelist: [
  "section-card", "section-card.s-mint", "section-card.s-sky", "section-card.s-mix",
  "lab-card",
  "radar-row", "radar-row.s-mint", "radar-row.s-sky",
],
```

> **💡 提示**
> 任何 `class={format!("xxx {}", yyy)}` 的写法，class 名都要进 `safelist`。或者把 format 拆成字面量条件：`if yyy { "section-card mint" } else { "section-card sky" }`。

### 坑 2：leptos `on:click` 失效的兜底

主题切换和番茄钟都遇到了"按钮没反应"。问题在 leptos 0.8 的 closure 类型推断 + SSR/hydrate 边界——某些情况下 click handler 没有被 hydrate 注册。

兜底方案：把按钮做成普通 HTML `<button id="x">`，在 `<script inner_html=...>` 里挂 vanilla `addEventListener`。完全不依赖 leptos 响应式。

```rust
view! {
    <button type="button" id="theme-toggle-btn">"切换"</button>
    <script inner_html=r#"
        (function() {
            var btn = document.getElementById('theme-toggle-btn');
            if (!btn) return;
            btn.addEventListener('click', function() {
                var html = document.documentElement;
                var current = html.getAttribute('data-theme') || 'dark';
                var next = current === 'dark' ? 'light' : 'dark';
                html.setAttribute('data-theme', next);
                document.body.setAttribute('data-theme', next);
                localStorage.setItem('rcrwhyg.theme', next);
            });
        })();
    "#></script>
}
```

> **关键判断**：leptos 是声明式的，但**事件绑定**在 SSR / hydrate 边界上有时不可靠。如果一个交互功能只是"改 DOM 状态"，vanilla JS 是更鲁棒的选择。

## 总结

5 轮 UI 调整里 AI 反复犯的工程错误，按出现频次排：
1. **越权 push / tag / CD**（每回 5+ 次）→ 落地：`rules/deploy-gating.md`
2. **本地预览缺失**（3 次）→ 落地：`rules/local-verification.md`
3. **复杂 bug 反复修不修好**（5 次主题切换）→ 经验：换假设不换方案
4. **Tailwind 树摇 format!() 类名** → 落地：safelist
5. **leptos on:click 失效** → 兜底：vanilla JS inline script

最关键的不是这些规则文件本身，是**把规则写到位**：

- `AGENT.md` 顶端 §1 必带"部署门禁" bullet（每次会话开头会被重新读）
- `AGENT.md` 顶端 §4 必带"用户检视" bullet
- 引用链 `AGENT.md` → `rules/` → `.cursor/rules/rcrwhyg.mdc` 至少 3 层冗余

工程上的信任不是靠"AI 这次没犯"，是靠"AI 的工作流让犯也犯不到"。

## 参考资料

1. 本仓库 `AGENT.md`、`rules/deploy-gating.md`、`rules/local-verification.md`
2. 本仓库 `style/tokens.css`、`style/tailwind.css`、`tailwind.config.js`
3. Anthropic Claude / Claude Code 官方：https://claude.com/claude-code
4. Leptos 0.8 文档：https://leptos.dev
5. Tailwind CSS tree-shaking 文档：https://tailwindcss.com/docs/content-configuration#class-detection-in-depth

**版本信息**: 本文基于 Leptos 0.8 / Rust 1.97 / Tailwind CSS 3，写于 2026-08。

---

**版权声明**: 本文原创发布于个人网站 https://rcrwhyg.com/articles/06-ai-collab-engineering-lessons/，作者：如春日午后阳光。未经授权请勿转载。
