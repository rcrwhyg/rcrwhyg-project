# 关于

> **摘要**: 我是一名在 Java / Rust 主线、React 副线、Zig / MoonBit / Cangjie 多元学习的开发者。本站建于 2026 年 3 月，原名「如春日午后阳光」，初衷是给自己一个长期可维护的写作与作品仓库。

## 创刊叙事

最早一个版本在 3 月上线，仅做"能跑就行"。中间试过**像素游戏机**与**命令行终端**两种主题风格——访客反馈割裂，遂统一为"用设计语言表达个性"的现代 + 薄荷 + 天空方案，并于 2026-08 整体重建并接上 CD 流水线，文章改走 `articles/*.md`（git + CD），后台用 Postgres 表管博客。

## 板块速览

- **文章** (`/articles`)：技术 / 经验分享
- **工具** (`/tools`)：自用小工具集
- **学习雷达** (`/radar`)：多生态学习进度
- **实验室** (`/lab`)：实验 / 小游戏 / demo
- **音乐** (`/music`)：氛围音乐
- **番茄钟** (`/clock`)：纯前端专注计时

## 设计语言

薄荷绿 + 天空蓝 + 阴影 + 光 + 全站玻璃面。整套令牌单源在 `style/tokens.css`，预览见 `docs/palette-preview.html`。

## 技术栈

Leptos 0.8 + Axum 0.8 + Rust 1.97（musl 静态二进制）；PostgreSQL 18（PGDG）；Caddy 2.11 反代 + 自动 HTTPS；CD 走 GitHub Actions（zigbuild + scp）。

## 联系方式

- 微信公众号：如春日午后阳光
- 文章原文链接：`<站点>/articles/<slug>/`
- 邮箱：rcrwhyg@sina.com
