---
name: rcrwhyg-article-workflow
description: Guides article creation for rcrwhyg — template, static gates, review flow, publishing handoff. Use when writing or reviewing articles in articles/, or when the user mentions 文章, 撰写, article, or publishing (including cross-posting to WeChat).
---

# 个人网站文章工作流

> 渠道说明：仓库内的 `articles/` 是**个人网站首发**的事实源。微信公众号【如春日午后阳光】是转载渠道——用户决定是否转载、如何排版、是否使用"阅读原文"链接回个人网站。AI 不参与公众号的发布与排版。

规范全文：[rules/content-quality.md](../../../rules/content-quality.md)、[rules/article-voice.md](../../../rules/article-voice.md)（**措辞与文体，撰写前必读**）、[specs/article-template.md](../../../specs/article-template.md)。

## 快速流程

1. 在 `articles/<合集>/` 或根目录新建 `NN-slug.md`（复制 `articles/templates/standard-template.md`）
2. 合集目录加 `_meta.json`：`{ "title": "合集名", "order": 1 }`
3. 摘要行固定格式：`> **摘要**: …`（150–200 字）
4. 「## 参考资料」章节：`资料名称：完整URL`，**禁止** Markdown 超链接
5. 结尾含「**版本信息**」与「**版权声明**」两行
6. 自检：`./tools/check-articles.sh`（递归扫描 `articles/**/*.md`；pre-commit/pre-push 也会跑）
7. `content:` 前缀提交；状态更新 `articles/README.md`
8. 用户审核，意见记 `docs/article-reviews.md`
9. 用户确认为最终版后，自行决定是否转载到微信公众号【如春日午后阳光】（用其指定工具，可加"阅读原文"指向个人网站 permalink）

## 措辞（成熟、稳重）

撰写或重构文章前读取 [rules/article-voice.md](../../../rules/article-voice.md)。要点：

- 工程复盘体：现象 → 根因 → 措施 → 仓库落点（规则/ADR/脚本路径）
- 避免口语与情绪化；「问题 N / 教训 N」优于「坑 N」
- 标杆：`articles/2c2g-server/06-ai-collab-engineering-lessons.md`
- 修订栈版本时，同步更新同合集全部文章的「版本信息」行

## 硬规则

- 代码示例必须真实验证过；引用仓库代码注明文件路径
- 正文不泄露 `.env`、口令、服务器地址、私有仓库路径
- 版本号 + 写作日期写入「版本信息」行
- 版权声明不写公众号专属信息——只声明作者与个人网站 URL；公众号转载由用户自行处理

## 常见静态门禁失败

| 报错 | 修法 |
|------|------|
| 缺少一级标题 | 首行改为 `# 标题` |
| 缺少摘要行 | 加 `> **摘要**: …`（注意是英文冒号模板） |
| 参考资料用了超链接 | 改成 `资料名称：https://…` 纯文本 |
| 缺少版权声明 | 按模板补版权行 |
