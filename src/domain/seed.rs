use crate::domain::Post;

/// Built-in posts used when Postgres is unavailable or empty.
pub fn seed_posts() -> Vec<Post> {
    vec![Post {
        id: Some(1),
        slug: String::from("plain-start"),
        title: String::from("朴素的启程：写在个人网站上线之前"),
        summary: Some(String::from(
            "一台 2C2G 云服务器，一次回归初心的个人站启程。",
        )),
        body_markdown: String::from(
            r#"大家好，很高兴在这里开启我的第一次分享。

起因很简单：我手上有一台 2 核 2G 配置的轻量云服务器。在这个动辄微服务、分布式的时代，它显得非常局促。但如果放任它吃灰，未免有些可惜。

于是我决定回归初心，动手给自己搭建一个完全属于自己的个人网站，并以此为契机，把日常的学习、踩坑和技术思考记录下来。

## 技术选型

作为喜欢折腾的开发者，我选择了 Rust + Leptos 作为技术栈。Rust 极低的内存占用非常适合我的 2C2G 服务器，而 Leptos 的服务端渲染（SSR）能带来极佳的首屏加载体验。

站点采用极致赛博朋克视觉：动态背景、鼠标追随光效，并支持暗色与亮色两套主题。博客与小工具集合会逐步长出来。

准备好了，我们这就出发。"#,
        ),
        tags: vec![String::from("随笔"), String::from("启程")],
        published_at: Some(String::from("2026-03-01")),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_plain_start_has_body_and_tags() {
        let posts = seed_posts();
        assert_eq!(posts.len(), 1);
        let post = &posts[0];
        assert_eq!(post.slug, "plain-start");
        assert!(post.body_markdown.contains("技术选型"));
        assert!(post.tags.iter().any(|t| t == "随笔"));
    }
}
