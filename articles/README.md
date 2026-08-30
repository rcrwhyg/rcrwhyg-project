# 文章目录约定（个人网站首发，公众号转载）

本项目文章每篇只维护一份 Markdown 文件，撰写、审核、修改都在同一份文件上完成。

## 发布渠道

- **个人网站（canonical）**：仓库内的 `articles/` 是事实源。模板与规范不依赖任何具体分发渠道——可被个人网站渲染、被转载到微信公众号、也可以被任何其他渠道引用。
- **微信公众号【如春日午后阳光】（转载渠道）**：由用户自行决定是否转载、如何排版、是否使用"阅读原文"链接回个人网站 URL。AI 不参与公众号的发布与排版。

## 目录结构

- `articles/`：文章根目录
- `articles/<合集目录>/`：一个合集（子目录名是 slug，展示名在 `_meta.json`）
  - `_meta.json`：`{ "title": "合集名", "order": 1 }`；占位合集加 `"placeholder": true`
  - `NN-slug.md`：合集内文章（编号在合集内递增）
- `articles/templates/`：模板，不参与发布
- 根目录下的 `NN-slug.md`：无合集归属的随笔
- 静态门禁：`tools/check-articles.sh`
- 审核记录：`docs/article-reviews.md`
- 规范：`rules/content-quality.md`、`specs/article-template.md`

## 状态标记

在本文件列表中用状态标注每篇文章：`草稿` → `待审核` → `已通过` → `已发布(日期)`

> "已发布"在此指**个人网站**已发布；公众号转载状态由用户独立维护。

## 当前文章

| 编号 | 文件 | 标题 | 状态 | 个人网站发布日 | 公众号转载日 |
|------|------|------|------|---------------|--------------|
| 01 | [2c2g-server/01-the-modest-start.md](2c2g-server/01-the-modest-start.md) | 朴素的启程：从一台 2C2G 服务器开始 | 已发布 | — | 2026-02-28 |
| 02 | [2c2g-server/02-deploy-full-stack-part-1.md](2c2g-server/02-deploy-full-stack-part-1.md) | 如何完成全栈应用线上部署？（一）：让 2C2G 服务器跑得更稳 | 已发布 | — | 2026-03-12 |
| 03 | [2c2g-server/03-deploy-full-stack-part-2.md](2c2g-server/03-deploy-full-stack-part-2.md) | 如何完成全栈应用线上部署？（二）：Caddy 网关与首次上线 | 待审核（系列一收官） | — | — |
| 06 | [2c2g-server/06-ai-collab-engineering-lessons.md](2c2g-server/06-ai-collab-engineering-lessons.md) | AI 协作的工程教训：把 5 轮 UI 调优做成 deploy-gating + 透明度模型 | 待审核 | — | — |

> 01、02 在"网站首发"模型确立前以公众号【如春日午后阳光】首发（日期见上）；本仓库内的 Markdown 是按统一规范整理的 canonical 副本。如需在个人网站补发，可直接用本仓库文件发布，届时回填"个人网站发布日"列并更新 permalink。
>
> 系列规划：01/02/03 属于"全栈部署（手工）"系列，03 收束并衔接 04；04 起开启"重建与持续部署"系列。

## 与站点文章系统

`articles/` 是站点**唯一**的内容事实源：git 管理、CD 部署、前台 `/articles` 渲染。管理员登录（`/admin`）用于站点运维，**不在浏览器内编辑文章**——撰写与发布在仓库内完成。

### 当前合集

| 目录 | 展示名 | 说明 |
|------|--------|------|
| `2c2g-server/` | 拥有一台 2C2G 的服务器，能做点什么？ | 现有 01–06 文章 |
| `cangjie-learn/` | 系统学习仓颉编程语言 | 占位，待从开源项目同步 |

### 排序规则（前台）

1. 仅展示 `NN-slug.md` 且未标记 `> **站点发布**: 否` 的文件
2. 倒序：文件名编号 → 版本日期 → slug
3. 合集名来自子目录 `_meta.json`，列表右上角徽章展示
