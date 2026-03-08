-- 平台类型枚举
CREATE TYPE platform_type AS ENUM ('wechat', 'zhihu', 'xiaohongshu', 'weibo', 'toutiao', 'other');


-- 文章表 - 支持多平台
CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    platform platform_type NOT NULL,                                          -- 发布平台
    platform_article_id VARCHAR(256) NOT NULL,                                -- 平台文章的唯一标识
    platform_article_url VARCHAR(512) NOT NULL,                               -- 平台原始链接
    platform_publish_time TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- 发布时间
    title VARCHAR(255) NOT NULL,                                              -- 文章标题
    author VARCHAR(100),                                                      -- 作者
    cover_image_url VARCHAR(512),                                             -- 替换为 OSS 后的封面图链接
    summary TEXT,                                                             -- 文章摘要 (可选)
    content TEXT NOT NULL,                                                    -- 文章正文 HTML
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,            -- 入库时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,            -- 更新时间
    UNIQUE(platform, platform_article_id)                                     -- 同一平台内唯一
);

-- 创建自动更新 updated_at 的触发器函数
CREATE OR REPLACE FUNCTION update_articles_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器
CREATE TRIGGER trigger_articles_update_timestamp
BEFORE UPDATE ON articles
FOR EACH ROW
EXECUTE FUNCTION update_articles_timestamp();

-- 创建约束触发器：防止 created_at 被修改
CREATE OR REPLACE FUNCTION prevent_created_at_modification()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.created_at IS DISTINCT FROM NEW.created_at THEN
        RAISE EXCEPTION '不允许修改 created_at 字段';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_articles_protect_created_at
BEFORE UPDATE ON articles
FOR EACH ROW
EXECUTE FUNCTION prevent_created_at_modification();

-- 索引优化
CREATE INDEX idx_articles_platform_publish ON articles(platform, platform_publish_time DESC);
CREATE INDEX idx_articles_publish_time ON articles(platform_publish_time DESC);
CREATE INDEX idx_articles_platform ON articles(platform);