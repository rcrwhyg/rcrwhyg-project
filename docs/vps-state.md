# VPS State — 现状快照

> ## ⏩ 已重建（2026-08-28）
>
> 本文件描述的是**重建前**的旧机器状态，已全部被本次从零重建取代（Ubuntu 26.04 LTS /
> PGDG PostgreSQL 18 / `rcrwhyg` 专用用户 / `/opt/rcrwhyg` 布局 / CD 流水线打通，
> 见重构后的 `docs/deploy-vps.md` 与 `.github/workflows/cd.yml`）。保留本文件仅作历史记录。

> 2026-08-25 探查；任何 VPS 状态变更前先回看本文件。
> 由 AI 在用户配合下 SSH 探查后落盘；本文件不会自动更新。

## 基本信息

| 字段 | 值 |
|------|-----|
| 提供商 | 阿里云 ECS（轻量应用服务器） |
| 公网 IP | 47.92.108.240 |
| 操作系统 | Ubuntu 24.04.4 LTS (Noble Numbat) |
| 内核 | 6.8.0-90-generic |
| 架构 | x86_64 |
| 磁盘 | 40GB（已用 7.5GB，可用 30GB） |
| 内存 | 2GB（规格：轻量应用服务器） |
| SSH 账号 | root |

## 域名与 TLS

| 域名 | 状态 | 备注 |
|------|------|------|
| `rcrwhyg.com` | 备案 + Caddy 配置 | 主站，自动 HTTPS |
| `www.rcrwhyg.com` | 备案 + Caddy 配置 | 主站 alias |
| `hub.rcrwhyg.com` | 备案 | Caddyfile 中无 site 块 → 跳 HTTPS 后 404 |
| `social.rcrwhyg.com` | 备案 | 同上 |
| `search.rcrwhyg.com` | 备案 | 同上 |

- Caddy 自动 TLS 邮箱：`rcrwhyg@sina.com`（仅在 `/etc/caddy/Caddyfile` 顶部 `email` 字段）

## 部署布局（现状，与 CD 流水线预期不一致）

```
/root/apps/rcrwhyg-server   ← 二进制，PID 255321
                              （没有 /opt/rcrwhyg/ 目录）
```

- **没有** `rcrwhyg` 系统用户
- systemd 服务名：`rcrwhyg-server.service`（**不是** CD 流水线预期的 `rcrwhyg.service`）
- 服务以 **root** 身份运行（与流水线预期的专用 `rcrwhyg` 用户不符）
- 后端监听：`0.0.0.0:3000`（流水线预期 `127.0.0.1:3000`，仅 Caddy 可达）
- 静态资源路径：**未确认**（不在 `/opt/rcrwhyg/site/`，但具体路径待补）
- `/root/apps/` 没有写进任何仓库文档

## Caddy

- 版本：v2.11.2（PID 187709）
- 配置：`/etc/caddy/Caddyfile`（1203 字节）
- 已启用项：
  - 站点：`rcrwhyg.com, www.rcrwhyg.com`
  - 反代：`localhost:3000`
  - 压缩：zstd + gzip
  - 安全头：HSTS（1 年）、X-Frame-Options DENY、X-Content-Type-Options nosniff
  - 日志：`/var/log/caddy/access.log`
- **未配置项**：HSTS `includeSubDomains; preload`、`Referrer-Policy`、`Permissions-Policy`、子域名 `hub/social/search.rcrwhyg.com` 的 site 块

完整 Caddyfile 内容（已抓取，与仓库内 `deploy/caddy/Caddyfile` 模板对比后，模板是更严格的基线——含 HSTS 2y+preload、Referrer-Policy、Permissions-Policy、`-Server` 头剥离等）。

## PostgreSQL

- 服务：`postgresql.service`（Ubuntu 默认包，**不是** PGDG 18 路径）
- 监听：127.0.0.1:5432 + 127.0.1.1:5432
- 主进程 PID 1423897
- **实际配置路径：`/etc/postgresql/<ver>/main/postgresql.conf`**
- ⚠️ 公众号文章（二）中描述的 `/var/lib/pgsql/18/data/postgresql.conf` 是 PGDG 安装路径，与本机不符；发布前需修正文章内容或注明适用范围

## 监听端口汇总

| 端口 | 进程 | 备注 |
|------|------|------|
| 22 | sshd | SSH |
| 53 | systemd-resolve | local DNS |
| 80, 443 | caddy | HTTP / HTTPS |
| 2019 | caddy | admin API（仅 127.0.0.1） |
| 3000 | rcrwhyg-server | SSR 后端，**当前绑定 0.0.0.0** |
| 5432 | postgres | local DB（127.0.0.1） |

## 已知问题

1. **`curl http://127.0.0.1:3000/health` 返回 404**。
   - 后端 SSR 主页可正常响应（`/` 返回 Leptos 的 modulepreload HTML）
   - `/health` 端点与 `docs/architecture.md` 描述不符
   - 可能原因：
     - 部署的二进制版本早于本仓库当前的 `src/server/health.rs` 实现
     - 实际端点路径不同（如 `/api/health`）
     - 当初部署时该功能未启用
   - **建议**：先用新 master 构建一次二进制、替换 `/root/apps/rcrwhyg-server` 再确认

2. **服务以 root 身份运行**：违反最小权限原则；CD 流水线预设的 `rcrwhyg` 专用用户未建立

3. **二进制路径非标**：`/root/apps/rcrwhyg-server`，不在 `/opt/rcrwhyg/`，与文档化部署布局不符

4. **多个子域名在 Caddyfile 中无 site 块**：`hub/social/search.rcrwhyg.com` 跳 HTTPS 后大概率 404；如未来要用，需补 site 块

## 与 CD 流水线的差异

| 项 | 现状 | CD 流水线预期（`deploy/`, `.github/workflows/cd.yml`） |
|----|------|---------|
| 系统用户 | root 跑一切 | `rcrwhyg` 专用用户 + 收窄 sudoers |
| 二进制路径 | `/root/apps/rcrwhyg-server` | `/opt/rcrwhyg/rcrwhyg-server` |
| 静态资源路径 | 未确认 | `/opt/rcrwhyg/site/` |
| systemd 服务名 | `rcrwhyg-server.service` | `rcrwhyg.service` |
| 后端绑定 | `0.0.0.0:3000` | `127.0.0.1:3000`（Caddy 反代） |
| Caddy 策略 | 用户手写版本 | 仓库 `deploy/caddy/Caddyfile` 模板（更严格基线） |
| 部署方式 | 手动 | tag `v*` 触发 + workflow_dispatch |

## 重建 / 对齐策略（待用户拍板）

### A. 迁移到 CD 流水线预设布局（推荐）

- 建 `rcrwhyg` 专用系统用户（无密码、无 sudo，home `/opt/rcrwhyg`）
- 安装 `/etc/sudoers.d/rcrwhyg` 收窄 systemctl/journalctl 权限
- 移二进制到 `/opt/rcrwhyg/rcrwhyg-server`
- 用 `deploy/systemd/rcrwhyg.service` 替换 `rcrwhyg-server.service`
- 把后端绑定改回 `127.0.0.1:3000`
- 拷贝 `deploy/caddy/Caddyfile` 覆盖 `/etc/caddy/Caddyfile`（更严格基线）
- 配合现有的 `docs/deploy-vps.md` 与 `deploy/` 资产
- **代价**：约 30 分钟停服（迁移期间 / 切流间隙会有 1-2 次短暂中断）

### B. 调整 CD 流水线适配现状

- 流水线改成以 root 部署，路径 `/root/apps/`
- service 名 `rcrwhyg-server.service`
- 收窄安全边界（不再讲"专用用户"的故事）
- **代价**：CD 流水线文档与现实长期分叉；新建/审 PR 时认知负担变大

### 默认建议

**A**。现状下 root 跑应用、路径非标都偏离原设计意图；CD 流水线已经按 A 写好，且 `docs/deploy-vps.md` 是 A 路径的手册。**等你拍板**；在拍板之前，文章发布与代码改动可继续推进，不依赖此决定。

## 仍需补全的探查项（可后续做）

- `ls -la /root/apps/` 看二进制旁边是否有 `site/`、`.env`、其他文件
- `cat /etc/systemd/system/rcrwhyg-server.service` 完整 service 文件内容
- 静态资源实际路径与加载方式（Caddy 当前只反代 `/`，没有 `file_server`）
- `curl http://127.0.0.1:3000/` 完整首页响应（确认功能/版本）
- 数据库表内容（确认 schema 是否与 `sql/posts.sql` / `sql/auth.sql` 一致）
