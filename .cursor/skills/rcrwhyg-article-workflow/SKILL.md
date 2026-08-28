---
name: rcrwhyg-article-workflow
description: Guides article creation for rcrwhyg — template, static gates, review flow, publishing handoff. Use when writing or reviewing articles in articles/, or when the user mentions 文章, 撰写, article, or publishing (including cross-posting to WeChat).
---

# 个人网站文章工作流

> 渠道说明：仓库内的 `articles/` 是**个人网站首发**的事实源。微信公众号【如春日午后阳光】是转载渠道——用户决定是否转载、如何排版、是否使用"阅读原文"链接回个人网站。AI 不参与公众号的发布与排版。

规范全文：[rules/content-quality.md](../../../rules/content-quality.md)、[specs/article-template.md](../../../specs/article-template.md)。

## 快速流程

1. 新建 `articles/NN-slug.md`（复制 `articles/templates/standard-template.md`）
2. 摘要行固定格式：`> **摘要**: …`（150–200 字）
3. 「## 参考资料」章节：`资料名称：完整URL`，**禁止** Markdown 超链接
4. 结尾含「**版本信息**」与「**版权声明**」两行
5. 自检：`./tools/check-articles.sh`（pre-commit/pre-push 也会跑）
6. `content:` 前缀提交；状态更新 `articles/README.md`
7. 用户审核，意见记 `docs/article-reviews.md`
8. 用户确认为最终版后，自行决定是否转载到微信公众号【如春日午后阳光】（用其指定工具，可加"阅读原文"指向个人网站 permalink）

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
