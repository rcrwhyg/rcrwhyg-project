# Leptos 0.8 SSR / Hydrate 边界

> 总结 5 轮 UI 调优里踩的 leptos 闭包坑，落盘成规则防止再犯。来源：
> `articles/06-ai-collab-engineering-lessons.md` "问题 3" 节。

## 核心规则

**`#[cfg(feature = "hydrate")]` 不能写在 view! 闭包 body 里。**

`#[cfg(feature = "hydrate")]` 必须**只出现在以下位置**：

1. 整个 `#[component]` 函数的 cfg 标注（`#[cfg(feature = "hydrate")] #[component] fn ...`）
2. 独立 `Effect::new(...)` 块的 cfg 标注（`#[cfg(feature = "hydrate")] { Effect::new(...) }`）
3. **整个辅助函数的 cfg 标注**（`#[cfg(feature = "hydrate")] fn apply_xxx() { ... }`）—— Effect 调这个函数
4. 整个 `view! { ... }` 块的 cfg 标注

## 为什么

leptos 0.8 的 view! 宏在 SSR 和 hydrate 时各编译一份闭包。**闭包的类型签名保持一致**——但 cfg 决定 body 内容的长度：

- SSR：cfg 不命中 → body 为空 → 闭包序列化长度 N
- Hydrate：cfg 命中 → body 有 web_sys / localStorage 代码 → 闭包序列化长度 N + M

hydrate 阶段反序列化时按 SSR 的长度读，遇到 SSR 没有的字节就静默失败。**事件绑定丢失，但页面渲染正常、按钮在、什么错都不报**——排查极难。

## 错写法（闭包内 cfg）

```rust
// ❌ closure 序列化长度在 SSR/hydrate 不一致
on:click=move |_: leptos::ev::MouseEvent| {
    let next = preference.theme.get_untracked().toggle();
    preference.set_theme.set(next);
    #[cfg(feature = "hydrate")]
    {
        if let Some(window) = web_sys::window() {
            window.local_storage()...;
            document.document_element()...;
        }
    }
}
```

## 对写法 A：cfg 整块（用于纯 hydrate-only 组件）

```rust
// ✅ 整块 cfg，闭包 body 在 SSR/hydrate 都为空
#[cfg(feature = "hydrate")]
fn hydrate_only_logic() {
    Effect::new(move |_| { /* web_sys stuff */ });
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    view! {
        <button id="theme-toggle-btn" type="button">"切换"</button>
        {hydrate_only_logic()}  // SSR 时此函数不存在 → 视图正常
    }
}
```

## 对写法 B：hydrate-only 子组件

```rust
// ✅ hydrate-only 子组件，闭包类型不跨边界
#[component]
pub fn ThemeToggle() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            // 只放 SSR + hydrate 都能跑的纯逻辑
        }>"切换"</button>
        <HydrateOnlyEffect />
    }
}

#[cfg(feature = "hydrate")]
#[component]
fn HydrateOnlyEffect() -> impl IntoView {
    Effect::new(move |_| { /* web_sys stuff */ });
    view! { <></> }
}
```

## 对写法 C：cfg 在辅助函数里（Effect 调辅助函数）

```rust
// ✅ closure 零 cfg，cfg 整块在辅助函数
#[component]
pub fn ThemeControls() -> impl IntoView {
    let theme = RwSignal::new(ThemeMode::Dark);

    let on_click = move |_| {
        theme.update(|t| *t = t.toggle());   // 闭包零 cfg
    };

    Effect::new(move |_| {
        apply_theme_to_dom(theme.get());      // 调辅助函数
    });

    view! {
        <button on:click=on_click class="control-btn">
            {move || match theme.get() { ... }}
        </button>
    }
}

#[cfg(feature = "hydrate")]
fn apply_theme_to_dom(t: ThemeMode) {
    // 整个函数在 SSR 阶段不存在，hydrate 阶段才有 web_sys 代码
    if let Some(window) = web_sys::window() {
        if let Some(html) = window.document_element() {
            let _ = html.set_attribute("data-theme", t.as_str());
        }
        if let Some(storage) = window.local_storage() {
            let _ = storage.set_item("rcrwhyg.theme", t.as_str());
        }
    }
}
```

## 对写法 D：DOM 操作直接用 vanilla JS（推荐用于"改 DOM attribute / localStorage"类操作）

```rust
// ✅ 改 DOM attribute 这种操作本身就是 vanilla JS 风格，不进 leptos 链路
view! {
    <button type="button" id="theme-toggle-btn">"切换"</button>
    <script inner_html=r#"
        (function() {
            var btn = document.getElementById('theme-toggle-btn');
            if (!btn) return;
            btn.addEventListener('click', function() {
                document.documentElement.setAttribute('data-theme', 'light');
                try { localStorage.setItem('rcrwhyg.theme', 'light'); } catch (e) {}
            });
        })();
    "#></script>
}
```

## 判断选哪个

| 场景 | 推荐方案 |
|------|----------|
| 改 DOM attribute / localStorage（imperative 操作） | 写法 D（vanilla JS inline script）——10 行 JS 比 20+ 行 leptos 闭包 + Effect + 辅助函数更直接 |
| 需要响应式更新 leptos 状态影响 UI 重渲染 | 写法 C（RwSignal + Effect + cfg 辅助函数）——闭包零 cfg，签名 SSR/hydrate 一致 |
| 整块逻辑只在 hydrate 时跑 | 写法 A（cfg 整块函数） |
| hydrate-only 副作用需要复用 | 写法 B（cfg 整块子组件） |

**关键判断**：
- "改 DOM attribute / localStorage" 这种 **imperative 操作** → vanilla JS 真的更直接
- "改 UI 状态影响 leptos 组件重渲染" → leptos 的核心价值
- **不是"兜底"vs"正经"**——是**不同问题用不同工具**
- 之前我说"vanilla JS 是兜底"是叙事错误。准确说法：**leptos 修得对，但 vanilla JS 在这个具体场景里更轻**

## 检测方法

```bash
# 找 closure 里的 cfg(feature = "hydrate")
grep -rE 'move\s*\|[^|]*\|' src/ | grep -B0 'cfg(feature = "hydrate")' || echo "OK: no closure-internal cfg(hydrate)"
```

## 引用

- `articles/06-ai-collab-engineering-lessons.md` "问题 3" 节
- `.cursor/rules/rcrwhyg.mdc` 第 N 条 bullet
