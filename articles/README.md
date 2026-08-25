# 文章目录约定（公众号）

本项目公众号文章每篇只维护一份 Markdown 文件，撰写、审核、修改都在同一份文件上完成。用户确认内容后，将该文件交给用户指定的公众号排版工具发布。

- `articles/`：文章文件目录（`NN-slug.md` 编号递增）
- `articles/templates/`：模板目录，不参与发布，不参与静态检查
- 静态门禁：`tools/check-articles.sh`（pre-commit / pre-push / CI 自动执行）
- 审核记录：`docs/article-reviews.md`
- 规范：`rules/content-quality.md`、`specs/article-template.md`

## 状态标记

在本文件列表中用状态标注每篇文章：`草稿` → `待审核` → `已通过` → `已发布(日期)`

## 当前文章

（暂无。第一篇建议主题：用 Leptos + Axum 从零搭建赛博朋克个人站 —— 本项目本身就是素材。）

## 与站点博客的关系

站点博客的事实源是 `posts`（数据库 + `/admin`，见 ADR-002/011）；`articles/` 是公众号渠道稿，独立维护、可互相改写，不强制同步。
