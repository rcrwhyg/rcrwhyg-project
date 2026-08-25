use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};
use leptos_router::NavigateOptions;

use crate::domain::{Post, PostInput};
use crate::server::{
    admin_create_post, admin_delete_post, admin_get_post, admin_list_posts, admin_update_post,
    require_admin,
};

#[component]
pub fn AdminPostsPage() -> impl IntoView {
    let auth = Resource::new(|| (), |_| async move { require_admin().await });
    let posts = Resource::new(
        || (),
        |_| async move {
            require_admin().await?;
            admin_list_posts().await
        },
    );
    let navigate = use_navigate();
    let delete_pending = RwSignal::new(Option::<i64>::None);
    let error = RwSignal::new(Option::<String>::None);

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <Suspense fallback=move || {
                view! { <p class="text-[color:var(--fg-dim)]">"加载中…"</p> }
            }>
                {move || match auth.get() {
                    Some(Err(_)) => {
                        let navigate = navigate.clone();
                        Effect::new(move |_| {
                            navigate("/admin/login", NavigateOptions::default());
                        });
                        view! { <p class="text-[color:var(--fg-dim)]">"需要登录…"</p> }.into_any()
                    }
                    Some(Ok(_)) => view! {
                        <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
                            <h1 class="page-title">"文章管理"</h1>
                            <div class="flex flex-wrap gap-2">
                                <A href="/admin/posts/new" attr:class="control-btn">"新建文章"</A>
                                <A href="/admin" attr:class="control-btn">"返回后台"</A>
                            </div>
                        </div>
                        <Show when=move || error.get().is_some() fallback=|| ()>
                            <p class="mb-4 text-sm text-[color:var(--danger)]">
                                {error.get().unwrap_or_default()}
                            </p>
                        </Show>
                        <Suspense fallback=move || {
                            view! { <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p> }
                        }>
                            {move || match posts.get() {
                                Some(Ok(list)) if list.is_empty() => view! {
                                    <p class="text-[color:var(--fg-muted)]">"还没有文章。"</p>
                                }
                                .into_any(),
                                Some(Ok(list)) => view! {
                                    <ul class="space-y-3">
                                        {list
                                            .into_iter()
                                            .map(|item| {
                                                let id = item.id.unwrap_or(0);
                                                let title = item.title.clone();
                                                let slug = item.slug.clone();
                                                let published = item.published_at.clone();
                                                let status = if published.is_some() {
                                                    "已发布"
                                                } else {
                                                    "草稿"
                                                };
                                                let href = format!("/admin/posts/{id}/edit");
                                                view! {
                                                    <li class="flex flex-wrap items-center justify-between gap-2 border-b border-[color:var(--border)] py-3">
                                                        <div class="min-w-0 space-y-1">
                                                            <p class="truncate font-medium">{title}</p>
                                                            <p class="text-xs text-[color:var(--fg-dim)]">
                                                                {format!("/{slug} · {status}")}
                                                            </p>
                                                        </div>
                                                        <div class="flex gap-2">
                                                            <A href=href attr:class="control-btn">"编辑"</A>
                                                            <button
                                                                type="button"
                                                                class="control-btn"
                                                                disabled=move || delete_pending.get() == Some(id)
                                                                on:click=move |_| {
                                                                    if id == 0 {
                                                                        return;
                                                                    }
                                                                    error.set(None);
                                                                    delete_pending.set(Some(id));
                                                                    leptos::task::spawn_local(async move {
                                                                        match admin_delete_post(id).await {
                                                                            Ok(()) => {
                                                                                posts.refetch();
                                                                                delete_pending.set(None);
                                                                            }
                                                                            Err(err) => {
                                                                                error.set(Some(err.to_string()));
                                                                                delete_pending.set(None);
                                                                            }
                                                                        }
                                                                    });
                                                                }
                                                            >
                                                                "删除"
                                                            </button>
                                                        </div>
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                }
                                .into_any(),
                                Some(Err(err)) => view! {
                                    <p class="text-[color:var(--danger)]">{format!("{err}")}</p>
                                }
                                .into_any(),
                                None => view! {
                                    <p class="text-[color:var(--fg-dim)]">"加载文章列表…"</p>
                                }
                                .into_any(),
                            }}
                        </Suspense>
                    }
                    .into_any(),
                    None => view! { <p class="text-[color:var(--fg-dim)]">"加载中…"</p> }.into_any(),
                }}
            </Suspense>
        </section>
    }
}

#[component]
pub fn AdminPostNewPage() -> impl IntoView {
    view! { <AdminPostEditor mode=EditorMode::Create /> }
}

#[component]
pub fn AdminPostEditPage() -> impl IntoView {
    let params = use_params_map();
    let id = Memo::new(move |_| {
        params
            .get()
            .get("id")
            .and_then(|v| v.parse::<i64>().ok())
    });

    view! {
        {move || match id.get() {
            Some(id) => view! { <AdminPostEditor mode=EditorMode::Edit(id) /> }.into_any(),
            None => view! {
                <section class="page-panel mx-auto my-8 max-w-3xl px-4">
                    <p class="text-[color:var(--danger)]">"无效的文章 id"</p>
                </section>
            }
            .into_any(),
        }}
    }
}

#[derive(Clone, Copy)]
enum EditorMode {
    Create,
    Edit(i64),
}

#[component]
fn AdminPostEditor(mode: EditorMode) -> impl IntoView {
    let auth = Resource::new(|| (), |_| async move { require_admin().await });
    let existing = Resource::new(
        move || match mode {
            EditorMode::Edit(id) => Some(id),
            EditorMode::Create => None,
        },
        |id| async move {
            require_admin().await?;
            match id {
                Some(id) => admin_get_post(id).await.map(Some),
                None => Ok(None),
            }
        },
    );

    let title = RwSignal::new(String::new());
    let slug = RwSignal::new(String::new());
    let summary = RwSignal::new(String::new());
    let body = RwSignal::new(String::new());
    let tags = RwSignal::new(String::new());
    let publish = RwSignal::new(false);
    let error = RwSignal::new(Option::<String>::None);
    let pending = RwSignal::new(false);
    let loaded = RwSignal::new(false);
    let navigate = use_navigate();

    Effect::new(move |_| {
        if loaded.get() {
            return;
        }
        if let Some(Ok(Some(post))) = existing.get() {
            fill_form(
                &post,
                title,
                slug,
                summary,
                body,
                tags,
                publish,
            );
            loaded.set(true);
        } else if matches!(mode, EditorMode::Create) {
            if let Some(Ok(None)) = existing.get() {
                loaded.set(true);
            }
        }
    });

    let heading = match mode {
        EditorMode::Create => "新建文章",
        EditorMode::Edit(_) => "编辑文章",
    };

    view! {
        <section class="page-panel mx-auto my-8 max-w-3xl px-4">
            <Suspense fallback=move || {
                view! { <p class="text-[color:var(--fg-dim)]">"加载中…"</p> }
            }>
                {move || match auth.get() {
                    Some(Err(_)) => {
                        let navigate = navigate.clone();
                        Effect::new(move |_| {
                            navigate("/admin/login", NavigateOptions::default());
                        });
                        view! { <p class="text-[color:var(--fg-dim)]">"需要登录…"</p> }.into_any()
                    }
                    Some(Ok(_)) => view! {
                        <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
                            <h1 class="page-title">{heading}</h1>
                            <A href="/admin/posts" attr:class="control-btn">"返回列表"</A>
                        </div>
                        <Show when=move || {
                            matches!(mode, EditorMode::Edit(_)) && existing.get().and_then(|r| r.ok()).flatten().is_none()
                                && existing.get().is_some()
                        } fallback=|| ()>
                            <p class="mb-4 text-sm text-[color:var(--danger)]">
                                {move || match existing.get() {
                                    Some(Err(err)) => err.to_string(),
                                    _ => "文章不存在".to_string(),
                                }}
                            </p>
                        </Show>
                        <form
                            class="space-y-4"
                            on:submit={
                                let navigate = navigate.clone();
                                move |ev| {
                                ev.prevent_default();
                                error.set(None);
                                pending.set(true);
                                let input = PostInput {
                                    title: title.get_untracked(),
                                    slug: slug.get_untracked(),
                                    summary: summary.get_untracked(),
                                    body_markdown: body.get_untracked(),
                                    tags: tags.get_untracked(),
                                    publish: publish.get_untracked(),
                                };
                                let navigate = navigate.clone();
                                leptos::task::spawn_local(async move {
                                    let result = match mode {
                                        EditorMode::Create => admin_create_post(input).await,
                                        EditorMode::Edit(id) => admin_update_post(id, input).await,
                                    };
                                    match result {
                                        Ok(_) => navigate("/admin/posts", NavigateOptions::default()),
                                        Err(err) => {
                                            error.set(Some(err.to_string()));
                                            pending.set(false);
                                        }
                                    }
                                });
                            }}
                        >
                            <label class="block space-y-1 text-sm">
                                <span class="text-[color:var(--fg-muted)]">"标题"</span>
                                <input
                                    class="control-btn w-full"
                                    type="text"
                                    required
                                    prop:value=move || title.get()
                                    on:input=move |ev| {
                                        let v = event_target_value(&ev);
                                        if matches!(mode, EditorMode::Create)
                                            && slug.get_untracked().is_empty()
                                        {
                                            slug.set(slugify_hint(&v));
                                        }
                                        title.set(v);
                                    }
                                />
                            </label>
                            <label class="block space-y-1 text-sm">
                                <span class="text-[color:var(--fg-muted)]">"Slug"</span>
                                <input
                                    class="control-btn w-full"
                                    type="text"
                                    required
                                    prop:value=move || slug.get()
                                    on:input=move |ev| slug.set(event_target_value(&ev))
                                />
                            </label>
                            <label class="block space-y-1 text-sm">
                                <span class="text-[color:var(--fg-muted)]">"摘要"</span>
                                <input
                                    class="control-btn w-full"
                                    type="text"
                                    prop:value=move || summary.get()
                                    on:input=move |ev| summary.set(event_target_value(&ev))
                                />
                            </label>
                            <label class="block space-y-1 text-sm">
                                <span class="text-[color:var(--fg-muted)]">"标签（逗号分隔）"</span>
                                <input
                                    class="control-btn w-full"
                                    type="text"
                                    prop:value=move || tags.get()
                                    on:input=move |ev| tags.set(event_target_value(&ev))
                                />
                            </label>
                            <label class="block space-y-1 text-sm">
                                <span class="text-[color:var(--fg-muted)]">"正文（Markdown）"</span>
                                <textarea
                                    class="control-btn min-h-64 w-full font-mono text-sm"
                                    required
                                    prop:value=move || body.get()
                                    on:input=move |ev| body.set(event_target_value(&ev))
                                />
                            </label>
                            <label class="flex items-center gap-2 text-sm">
                                <input
                                    type="checkbox"
                                    prop:checked=move || publish.get()
                                    on:change=move |ev| {
                                        publish.set(event_target_checked(&ev));
                                    }
                                />
                                <span class="text-[color:var(--fg-muted)]">"发布（取消勾选则为草稿）"</span>
                            </label>
                            <Show when=move || error.get().is_some() fallback=|| ()>
                                <p class="text-sm text-[color:var(--danger)]">
                                    {error.get().unwrap_or_default()}
                                </p>
                            </Show>
                            <button type="submit" class="control-btn" disabled=move || pending.get()>
                                {move || if pending.get() { "保存中…" } else { "保存" }}
                            </button>
                        </form>
                    }
                    .into_any(),
                    None => view! { <p class="text-[color:var(--fg-dim)]">"加载中…"</p> }.into_any(),
                }}
            </Suspense>
        </section>
    }
}

fn fill_form(
    post: &Post,
    title: RwSignal<String>,
    slug: RwSignal<String>,
    summary: RwSignal<String>,
    body: RwSignal<String>,
    tags: RwSignal<String>,
    publish: RwSignal<bool>,
) {
    title.set(post.title.clone());
    slug.set(post.slug.clone());
    summary.set(post.summary.clone().unwrap_or_default());
    body.set(post.body_markdown.clone());
    tags.set(post.tags.join(", "));
    publish.set(post.published_at.is_some());
}

fn slugify_hint(title: &str) -> String {
    title
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
