CREATE TABLE wechat_posts (
    id SERIAL PRIMARY KEY,
    media_id VARCHAR(128) UNIQUE NOT NULL,    -- 微信文章的唯一标识
    title VARCHAR(255) NOT NULL,              -- 文章标题
    author VARCHAR(100),                      -- 作者
    cover_image_url VARCHAR(512),             -- 替换为 OSS 后的封面图链接
    content_html TEXT NOT NULL,               -- 替换过图片链接的纯净 HTML
    wechat_url VARCHAR(512) NOT NULL,         -- 微信原始长链接（用于跳转评论）
    publish_time TIMESTAMPTZ NOT NULL,        -- 微信发布时间
    created_at TIMESTAMPTZ DEFAULT NOW()      -- 同步到网站的时间
);

-- 加个索引，方便按时间倒序在首页展示
CREATE INDEX idx_wechat_posts_publish_time ON wechat_posts(publish_time DESC);
