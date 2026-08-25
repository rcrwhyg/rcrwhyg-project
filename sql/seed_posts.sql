-- Seed data for local / staging. Apply after sql/posts.sql.
-- Example: psql "$DATABASE_URL" -f sql/posts.sql -f sql/seed_posts.sql

INSERT INTO posts (slug, title, summary, body_markdown, published_at)
VALUES (
    'plain-start',
    '朴素的启程：写在个人网站上线之前',
    '一台 2C2G 云服务器，一次回归初心的个人站启程。',
    $md$大家好，很高兴在这里开启我的第一次分享。

起因很简单：我手上有一台 2 核 2G 配置的轻量云服务器。在这个动辄微服务、分布式的时代，它显得非常局促。但如果放任它吃灰，未免有些可惜。

于是我决定回归初心，动手给自己搭建一个完全属于自己的个人网站，并以此为契机，把日常的学习、踩坑和技术思考记录下来。

## 技术选型

作为喜欢折腾的开发者，我选择了 Rust + Leptos 作为技术栈。Rust 极低的内存占用非常适合我的 2C2G 服务器，而 Leptos 的服务端渲染（SSR）能带来极佳的首屏加载体验。

站点采用极致赛博朋克视觉：动态背景、鼠标追随光效，并支持暗色与亮色两套主题。博客与小工具集合会逐步长出来。

准备好了，我们这就出发。$md$,
    TIMESTAMPTZ '2026-03-01T00:00:00Z'
)
ON CONFLICT (slug) DO UPDATE SET
    title = EXCLUDED.title,
    summary = EXCLUDED.summary,
    body_markdown = EXCLUDED.body_markdown,
    published_at = EXCLUDED.published_at;

INSERT INTO tags (name) VALUES ('随笔'), ('启程')
ON CONFLICT (name) DO NOTHING;

INSERT INTO post_tags (post_id, tag_id)
SELECT p.id, t.id
FROM posts p
CROSS JOIN tags t
WHERE p.slug = 'plain-start'
  AND t.name IN ('随笔', '启程')
ON CONFLICT DO NOTHING;
