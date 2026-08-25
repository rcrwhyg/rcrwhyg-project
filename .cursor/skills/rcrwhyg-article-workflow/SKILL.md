---
name: rcrwhyg-article-workflow
description: Guides WeChat official-account article creation for rcrwhyg — template, static gates, review flow, publishing handoff. Use when writing or reviewing articles in articles/, or when the user mentions 公众号, 文章, 撰写, article, or publishing to WeChat.
---

# 公众号文章工作流

规范全文：[rules/content-quality.md](../../../rules/content-quality.md)、[specs/article-template.md](../../../specs/article-template.md)。

## 快速流程

1. 新建 `articles/NN-slug.md`（复制 `articles/templates/standard-template.md`）
2. 摘要行固定格式：`> **摘要**: …`（150–200 字）
3. 「## 参考资料」章节：`资料名称：完整URL`，**禁止** Markdown 超链接
4. 结尾含版权声明
5. 自检：`./tools/check-articles.sh`（pre-commit/pre-push 也会跑）
6. `content:` 前缀提交；状态更新 `articles/README.md`
7. 用户审核，意见记 `docs/article-reviews.md`；AI 不自行发布

## 硬规则

- 代码示例必须真实验证过；引用仓库代码注明文件路径
- 正文不泄露 `.env`、口令、服务器地址、私有仓库路径
- 版本号 + 写作日期写入「版本信息」行
- 站点博客事实源是 `posts`（DB + /admin）；articles/ 只是公众号渠道稿

## 常见静态门禁失败

| 报错 | 修法 |
|------|------|
| 缺少一级标题 | 首行改为 `# 标题` |
| 缺少摘要行 | 加 `> **摘要**: …`（注意是英文冒号模板） |
| 参考资料用了超链接 | 改成 `资料名称：https://…` 纯文本 |
| 缺少版权声明 | 按模板补版权行 |
