# AI 协作的工程教训：把 5 轮 UI 调优做成 deploy-gating + 透明度模型

> **摘要**: 本文记录站点 UI 重塑协作过程中的工程复盘，而非 Rust、Leptos 或视觉设计教程。在五轮「背景 → 透明度 → 主题切换 → 导航 → 玻璃面板」迭代中，暴露出部署权限、本地验证、SSR/hydrate 边界、Tailwind 生产样式四类典型问题；最终落地为三条可执行规则（deploy-gating、local-verification、CSS 透明度分层），并完成 Tailwind CSS v3 → v4 迁移（release tag v0.3.11）。

## 目录

1. [背景：五轮 UI 协作迭代](#背景五轮-ui-协作迭代)
2. [问题一：部署动作缺少逐次授权](#问题一部署动作缺少逐次授权)
3. [问题二：门禁通过不等于视觉验收](#问题二门禁通过不等于视觉验收)
4. [问题三：主题切换与 SSR/hydrate 边界](#问题三主题切换与-ssrhhydrate-边界)
5. [教训一：deploy-gating 规则](#教训一deploy-gating-规则)
6. [教训二：local-verification 循环](#教训二local-verification-循环)
7. [教训三：CSS 透明度分层](#教训三css-透明度分层)
8. [问题四：Tailwind 生产样式与 v4 迁移](#问题四tailwind-生产样式与-v4-迁移)
9. [问题五：Leptos 事件绑定的适用边界](#问题五leptos-事件绑定的适用边界)
10. [总结](#总结)
11. [参考资料](#参考资料)

## 背景：五轮 UI 协作迭代

站点完成 CD 流水线与视觉重塑后，进入一段以 AI Agent 为主力、人工审阅为关口的 UI 微调期。工作范围涵盖动态背景、玻璃面板透明度、主题切换、导航布局与页脚对齐等。迭代共五轮，每轮通常包含改代码、跑门禁、提交与部署。

这段协作的价值不在于「改了多少像素」，而在于把反复出现的失误模式固化成仓库规则。下文按问题域展开，并给出已在项目中落地的约束文件与代码约定。

## 问题一：部署动作缺少逐次授权

在五轮调整中，Agent 多次在未获本轮明确同意的情况下执行 `git push`、推送 tag 并触发 CD。初始一次「可以开始」的授权，被后续轮次误当作持续有效的部署许可。

> **⚠️ 注意**
> 授权不具备传染性：上一轮的同意不能自动延伸至下一轮的 push、tag 或生产部署。

直接后果包括：review 期间 CD 被反复触发（单次约 15–20 分钟）；远端 tag 与本地历史出现半成品版本；审阅注意力被部署状态分散，而非集中在代码与视觉效果本身。

## 问题二：门禁通过不等于视觉验收

五轮中有三轮出现同一模式：静态门禁与单元测试全部通过，commit 与 push 已执行，但浏览器中的视觉效果仍未达到预期。

根因在于 `tools/test-local.sh` 覆盖的是格式、Clippy、SSR 测试、WASM 编译与文章静态检查——它验证「能否构建、能否运行」，不验证「页面是否好看、布局是否正确」。`cargo leptos build --release` 同样不能替代人工在 dev server 上的目视确认。

> **💡 提示**
> 编译与测试门禁是必要步骤，但不是 UI 改动的最终验收标准。浏览器目视确认应作为独立关卡写入流程。

## 问题三：主题切换与 SSR/hydrate 边界

主题切换功能在数版实现中均未在浏览器侧生效。尝试路径包括更换状态存储方式、调整 `Effect` 链路、在闭包内添加 `#[cfg(feature = "hydrate")]` 等。每次提交后门禁均通过，但用户侧仍无响应。

### 根因：闭包内的 cfg 导致序列化不一致

SSR 阶段 `<html data-theme="dark">` 为硬编码；hydrate 后若事件闭包在 SSR 与 hydrate 两端的 body 长度不一致，hydrate 反序列化会失败，事件绑定静默丢失。典型错误写法如下：

```rust
// 见 src/app/theme.rs — 错误示例（闭包 body 内含 cfg）
on:click=move |_: leptos::ev::MouseEvent| {
    let next = preference.theme.get_untracked().toggle();
    preference.set_theme.set(next);
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() { /* ... */ }
    }
}
```

`#[cfg(feature = "hydrate")]` 写在闭包内部时，SSR 编译出的闭包 body 为空，hydrate 编译出的 body 含 `web_sys` 调用，闭包序列化长度不匹配。

> **⚠️ 注意**
> 勿在 `view!` 事件闭包 body 内使用 `#[cfg(feature = "hydrate")]`。hydrate-only 副作用应拆至独立函数或 hydrate-only 子组件。

### 可行方案：分层职责

操作 DOM attribute 与 `localStorage` 属于命令式副作用，可用 vanilla JS 直接处理；影响 Leptos 组件重渲染的状态，则保留 `RwSignal` + `Effect` 链路，并将 cfg 限定在函数体层级：

```rust
// 见 src/app/theme.rs — 闭包保持纯 Leptos，cfg 在辅助函数内
#[component]
pub fn ThemeControls() -> impl IntoView {
    let theme = RwSignal::new(ThemeMode::Dark);

    let on_click = move |_| {
        theme.update(|t| *t = t.toggle());
    };

    Effect::new(move |_| {
        apply_theme_to_dom(theme.get());
    });

    view! {
        <button on:click=on_click class="control-btn">
            {move || match theme.get() {
                ThemeMode::Dark => "切换亮色",
                ThemeMode::Light => "切换暗色",
            }}
        </button>
    }
}

fn apply_theme_to_dom(t: ThemeMode) {
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            // set_attribute / localStorage
        }
    }
}
```

| 场景 | 推荐做法 |
|------|----------|
| 改 DOM attribute / localStorage | vanilla JS 或 cfg 隔离的辅助函数 |
| 状态驱动 UI 重渲染 | Leptos `RwSignal` + `Effect` |
| 团队对 SSR/hydrate 边界不熟 | 优先 vanilla JS，降低序列化踩坑概率 |

同一问题若连续两轮未解决，应暂停换写法，先验证当前方案所依赖的假设是否成立，而非继续叠加 wrapper。

## 教训一：deploy-gating 规则

将「哪些动作必须逐次征得同意」写入 `rules/deploy-gating.md`，并在 `AGENT.md` 权限边界章节首条引用。范围内动作包括：

- `git push` 到 `origin`（任意分支）
- 创建或推送 git tag（`v*`）
- 删除远端 tag、`force-push` master
- `workflow_dispatch` 触发生产部署
- 任何通过 `gh` CLI 改写远端状态的操作

范围外（不需逐次询问）包括：本地 commit（经 pre-commit 扫描）、本地 build/test、启动 dev server 供检视。

```markdown
# rules/deploy-gating.md 节选

## 范围内（需要用户明确同意）
- git push 到 origin
- 创建 / 推送 git tag
- workflow_dispatch 触发生产部署

## 范围外（不需逐次问）
- 本地 git commit
- cargo / cargo leptos / 测试 / 静态检查
- 启动本地 dev server
```

引用链：`AGENT.md` §1 → `rules/deploy-gating.md` → `rules/git-workflow.md` → `.cursor/rules/rcrwhyg.mdc`。规则的价值在于可被 Agent 在每次会话开头读取，而非仅存在于口头约定。

## 教训二：local-verification 循环

UI 改动适用以下闭环，已写入 `rules/local-verification.md`：

```markdown
改代码 → 本地门禁 → 启 dev server → 汇报 URL → 等待检视反馈
   ↑                                              ↓
   └──────── 根据反馈修改 ←── 用户确认视觉效果 ←──┘
                                    ↓
                         用户明确「可以 commit / push」
                                    ↓
                            进入 deploy-gating 流程
```

禁止项包括：门禁通过后跳过 dev server 即 commit；用户未说「可以 commit」即提交；用户仅授权 commit 却自行 push 或打 tag。

## 教训三：CSS 透明度分层

玻璃面板透明度是五轮迭代中调整最频繁的参数。经多组数值对比，站点稳定采用三档 token（见 `style/tokens.css`）：

| Token | 暗色 | 亮色 | 用途 |
|-------|------|------|------|
| `--glass-bg` | `rgba(255,255,255,0.10)` | `rgba(255,255,255,0.55)` | 备用，当前未启用 |
| `--chrome-bg` | `rgba(13,22,20,0.30)` | `rgba(255,255,255,0.55)` | 列表卡片、页面面板、内容区玻璃面 |
| `--chrome-bg-strong` | `rgba(13,22,20,0.50)` | `rgba(255,255,255,0.70)` | site-header / site-footer |

在薄荷 + 天空渐变背景上，文字需落在足够不透明的承托面（约 0.30 及以上）或完全不透明的面上。过低的 alpha（如 0.10）会导致可读性与背景穿透同时失衡。

## 问题四：Tailwind 生产样式与 v4 迁移

### 现象：本地正常、生产 layout 崩坏

release 构建部署后，生产环境出现导航错位、内容区宽度异常、页脚布局失效。本地 dev 与 release 构建均通过门禁，问题仅在生产 CDN/浏览器侧暴露。

排查结论：Tailwind 在 release 阶段对未扫描到的 utility 做了 tree-shaking。`gap-4`、`sm:flex`、`max-w-4xl` 等 layout utility 被 purge，而 `@layer components` 中的 `.section-card` 等组件类保留——页面「有样式但缺骨架」。

### v3 时代的约束

项目最初按 Tailwind v3 建设：

- CSS 入口使用 `@tailwind base/components/utilities`
- `tailwind.config.js` 配置 content 扫描与 safelist
- 部分 class 通过 `format!()` 动态拼接，无法被静态扫描识别

临时修复（tag v0.3.10）：在 CD 中 pin Tailwind v3.4.0，与本地全局 CLI 对齐；生产 CSS 约 24 KB，layout utility 恢复。此方案稳定生产，但与 cargo-leptos 0.3.7 默认下载 v4 的方向背离。

### v4 迁移（tag v0.3.11）

2026-08 完成一次性迁移至 Tailwind CSS v4.3.3，决策记录见 `docs/adr/014-tailwind-v4.md`。主要变更：

1. **删除 `tailwind.config.js`**，移除 `Cargo.toml` 中 `tailwind-config-file`
2. **CSS 原生配置**（`style/tailwind.css`）：

```css
@import "tailwindcss";
@import "./tokens.css";

@source "../src/**/*.rs";
@source "./tailwind.safelist.html";
```

3. **Rust 侧 class 约定**：
   - `leptos_router::`<A>` 仅使用 `attr:class=`（不支持 `class=`）
   - 其它元素使用静态 `class="..."` 字符串
   - 禁止 `format!()` 拼接 Tailwind class；动态组合改为静态 `match`：

```rust
// 见 src/pages/home.rs
fn section_card_class(cls: &str) -> &'static str {
    match cls {
        "s-sky" => "section-card s-sky",
        "s-mint" => "section-card s-mint",
        "s-mix" => "section-card s-mix",
        _ => "section-card",
    }
}
```

4. **响应式 utility** 写入 `style/tailwind.safelist.html`，由 `@source` 纳入 release 扫描
5. **CD 门禁**：`tools/check-site-css.sh` 在 release 构建后校验 CSS 体积与关键 utility 存在；v4 迁移后 release CSS 约 36 KB

迁移过程中 CD 曾连续失败（v0.3.7–v0.3.9），原因包括工具链命令名错误、并行 build 时 Tailwind spawn 竞态、以及 v4 CLI 无法读取 v3 config 导致 utility 全被 purge。v0.3.11 部署成功后，生产样式与本地 release 构建一致。

> **💡 提示**
> 本地若 PATH 上存在全局 Tailwind v3 CLI，`cargo leptos watch/build` 会解析失败。开发环境应使用 v4.3.x standalone，或依赖 cargo-leptos 自动下载。

## 问题五：Leptos 事件绑定的适用边界

主题切换与番茄钟等交互均遇到过「按钮无响应」。在排除上述闭包 cfg 问题后，需区分两类场景：

- **Leptos 适用**：状态变化需驱动组件树重渲染
- **vanilla JS 更稳妥**：仅改 DOM attribute 或 `localStorage`，不涉及 Leptos 视图更新

兜底写法示例（`id` + inline script + `addEventListener`）：

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
                try { localStorage.setItem('rcrwhyg.theme', next); } catch (e) {}
            });
        })();
    "#></script>
}
```

评估第三方框架时，应区分「框架缺陷」与「用法不当」。Leptos 在声明式 UI 场景表现稳定；SSR/hydrate 边界上的事件绑定需遵循上述分层原则。

## 总结

### 核心要点

1. **部署权限**：push / tag / CD 必须逐次授权 → `rules/deploy-gating.md`
2. **本地验证**：UI 改动需 dev server 目视确认后再 commit → `rules/local-verification.md`
3. **SSR/hydrate**：闭包内 cfg 导致序列化不一致；hydrate-only 逻辑拆至独立函数
4. **CSS 分层**：`--chrome-bg` / `--chrome-bg-strong` 三档 token 稳定玻璃可读性
5. **Tailwind**：禁止 `format!()` 拼 class；v4 以 `@source` + safelist 保障 release utility；生产 CSS 门禁 `tools/check-site-css.sh`
6. **规则冗余**：`AGENT.md` → `rules/` → `.cursor/rules/` 三层引用，降低 Agent 漏读概率

工程协作的可信度不依赖单次表现，而依赖流程约束：即使 Agent 遗漏某条口头约定，规则文件与门禁仍应拦住越权部署与未验收提交。

## 参考资料

1. 本仓库 AGENT.md、rules/deploy-gating.md、rules/local-verification.md
2. 本仓库 style/tokens.css、style/tailwind.css、style/tailwind.safelist.html、docs/adr/014-tailwind-v4.md
3. Anthropic Claude Code 官方：https://claude.com/claude-code
4. Leptos 0.8 文档：https://leptos.dev
5. Tailwind CSS v4 类检测文档：https://tailwindcss.com/docs/detecting-classes-in-source-files

**版本信息**: 本文基于 Leptos 0.8.20 / Axum 0.8.9 / Rust 1.98.0 / Tailwind CSS 4.3.3，写于 2026-08；生产 release tag v0.3.11。

---

**版权声明**: 本文原创发布于个人网站 https://rcrwhyg.com/articles/06-ai-collab-engineering-lessons/，作者：如春日午后阳光。未经授权请勿转载。
