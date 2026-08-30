# 如何完成全栈应用线上部署？（二）：Caddy 网关与首次上线

> **摘要**: 本文是部署系列的第二篇，聚焦核心网关 Caddy 的进阶配置以及"从代码到域名访问"的完整首次上线流程。读者将了解 Caddy 相比 Nginx 的取舍、Caddyfile 的关键写法、Leptos 全栈项目里静态资源与 API 反代的处理模式，以及"编译 → 上传 → 起服 → 验证"四步走的实操流程。本文的操作基于 Ubuntu 26.04 LTS + 阿里云 ECS 轻量服务器 + Rust + Leptos 全栈项目。

## 目录

1. [核心网关：Caddy 的进阶配置](#核心网关caddy-的进阶配置)
2. [项目上线：从静态页面代码到域名访问](#项目上线从静态页面代码到域名访问)
3. [写在最后](#写在最后)
4. [总结](#总结)
5. [参考资料](#参考资料)

## 核心网关：Caddy 的进阶配置

### 为什么选择 Caddy 而不使用 Nginx？

Nginx 是行业标杆，但对个人开发者来说 Caddy 在两个维度上更友好：

1. **自动 HTTPS**。Caddy 默认会用 ACME 协议（Let's Encrypt / ZeroSSL）自动申请并续期证书，配置好域名即可启用 HTTPS；Nginx 需要手动申请证书、配置 Certbot 或类似客户端，并在到期前续期。对一个"配置好就忘"的个人站来说，Caddy 省掉了所有证书运维。
2. **配置更简洁**。同等功能下 Caddyfile 的行数大约是 Nginx 配置的 1/3 - 1/2。Nginx 经常需要拆 main 配置 + `sites-enabled` + 各种 `include`；Caddy 一个 Caddyfile 就够了。

Nginx 的优势是生态更成熟、性能极限更高、模块化更细。对 2C2G 个人站来说，Caddy 简洁的代价（性能差距）完全可以忽略。

### Caddyfile 关键写法

本项目当前线上使用的 Caddyfile（来自 `/etc/caddy/Caddyfile`，1203 字节）：

```caddyfile
{
    # 顶部 options 块：全局设置
    email rcrwhyg@sina.com   # Let's Encrypt 通知邮箱
}

rcrwhyg.com, www.rcrwhyg.com {
    # 启用 zstd 与 gzip 压缩
    encode zstd gzip

    # 反向代理到本地后端（Leptos Axum SSR 监听 0.0.0.0:3000）
    reverse_proxy localhost:3000

    # 安全响应头
    header {
        # HSTS：一年；建议生产改 2 年 + includeSubDomains + preload
        Strict-Transport-Security "max-age=31536000;"
        # 防点击劫持
        X-Frame-Options "DENY"
        # 禁内容类型嗅探
        X-Content-Type-Options "nosniff"
    }

    # 访问日志
    log {
        output file /var/log/caddy/access.log
    }
}
```

几个关键点：

- **`email` 字段**：放在顶层 options 块，作为 Let's Encrypt 账户邮箱。Caddy 在申请证书时用这个邮箱通知到期/失败；显式设置更可追溯。
- **`encode zstd gzip`**：双压缩。客户端支持 zstd 时优先 zstd（更快），否则回退到 gzip。
- **`reverse_proxy localhost:3000`**：所有 HTTP/HTTPS 流量都打给本地 3000 端口。Caddy 自动处理 WebSocket upgrade（如果后端用的话）。
- **HSTS**：让浏览器记住"本站只能用 HTTPS"。一年（31536000 秒）适合初次上线试探——若证书或子域有问题，一年后客户端可自然恢复；两年（63072000 秒）+ `includeSubDomains` + `preload` 是"锁定"形态，代价是浏览器拒绝 HTTP 回退、且子域名也必须全 HTTPS，若配置有误影响期更长。**本站当前生产配置即两年版**（`max-age=63072000; includeSubDomains; preload`），但未向 [hstspreload.org](https://hstspreload.org/) 提交 preload 列表——因为 `hub/social/search` 等子域还没有独立站点，preload 提交要等它们全部 HTTPS 后才会做；届时还会把该项影响全站子域的特点写清楚。
- **日志**：本站模板用 `output stdout`——日志写标准输出，由 systemd journald 统一收管（`journalctl -u caddy` 查看），自带轮转无需额外配置。若想独立日志文件并按大小切分，Caddy 的 `output file` 自带轮转参数：

  ```caddyfile
  log {
      output file /var/log/caddy/access.log {
          roll_size 10MiB      # 单个文件到 10MiB 切一个
          roll_keep 5          # 最多保留 5 个滚动文件
          roll_keep_for 240h   # 保留 10 天
      }
  }
  ```

### 静态资源与 API 服务的处理

Leptos 全栈项目的特点是：SSR HTML、API 路由（`/api/*`、`/health`、`/sse/*`、`/ws/*`）、前端 WASM（`/pkg/*.wasm`、`/pkg/*.js`）全部由同一个 Axum 进程提供，且应用自身就能服务 `site/` 下的静态文件（Leptos 的 `file_and_error_handler`）。所以 Caddy 的处理极简——所有请求反代到 `localhost:3000` 即可，静态资源无需额外配置。

```caddyfile
# 本站实际写法：一切交给后端
rcrwhyg.com, www.rcrwhyg.com {
    reverse_proxy 127.0.0.1:3000
    # …
}
```

**那文章、图片、视频到底怎么展示？** 本站文章是 Markdown 源文件，渲染链路如下：

```text
articles/*.md ──► (构建时打包进 site/articles) ──► 路由 /articles、/articles/:slug
   ──► 应用读取 md → pulldown-cmark 渲染为 HTML ──► 页面按 .markdown-body 预设样式展示
```

- 文字、代码块、表格、链接、提示框：全部来自 md 本身，不依赖外部资源；
- 图片：文章里写**相对路径**（如 `![示意图](/media/diagram.png)`），图片放进 `site/media/`（随 CD 一并打包），浏览器请求 `/media/…` 时由应用直接静态返回。**目前本站文章还没有插图**，等真有需求再按此加入，不必提前配 Caddy 的文件服务；
- 视频：与图片同思路，但体积大、占流量，暂不引入。

**那 Caddy 的 `file_server` 什么时候用？** 它适合"纯静态目录直接吐给浏览器、不经过应用"的场景：独立静态站点、用户上传的文件、交给 CDN 分发的大目录。对本站来说，文章需要应用渲染，其余静态资源应用也能服务，所以**目前用不到**；将来若要把某个大目录完全交给 Caddy 托管，再 `handle /media/* { root * /srv/media; file_server }` 即可，多一行配置的事。

## 项目上线：从静态页面代码到域名访问

本节描述"代码改完到用户能通过域名访问"的完整流程——本系列以手工部署为主线（当时 CD 尚未接入），如今这套已由流水线自动接管（见《[从零重建到一键部署：一条 CD 流水线的诞生](/articles/04-rebuild-to-cd-pipeline/)》）。

### 完成代码编写与编译构建

#### 1. 前端资源构建

```bash
# 在项目根目录
cargo leptos build --release
```

产出物路径：`target/site/`

- `target/site/pkg/rcrwhyg-server.js`、`target/site/pkg/rcrwhyg-server_bg.wasm`：WASM hydrate bundle
- `target/site/pkg/*.css`：编译后的样式（含 Tailwind）
- `target/site/index.html`：默认首页
- `target/site/404.html`：404 页面

> **💡 提示**
> `cargo leptos build` 会同时调用 `wasm-bindgen` 把 Rust 编译为浏览器端 WASM，再用 `lightningcss` 优化 CSS。`--release` 让最终 WASM 走 `wasm-release` profile（`opt-level = 'z'`、`lto = true`、`panic = "abort"`），大小通常比开发版小 3-5 倍。

#### 2. 服务端二进制构建（musl 静态链接）

```bash
# 确保 zig 与 cargo-zigbuild 已装
# brew install zig
# cargo install --locked cargo-zigbuild

# 跨平台构建 Linux musl 静态二进制
cargo zigbuild --release \
    --target x86_64-unknown-linux-musl \
    --features ssr \
    --bin rcrwhyg-server
```

产出物路径：`target/x86_64-unknown-linux-musl/release/rcrwhyg-server`

验证静态链接：

```bash
file target/x86_64-unknown-linux-musl/release/rcrwhyg-server
# 应该看到 "statically linked"
ldd target/x86_64-unknown-linux-musl/release/rcrwhyg-server
# 应该输出 "not a dynamic executable"
```

> **💡 提示**
> **为什么用 musl + zigbuild？** 本地 macOS 上用普通 `cargo build` 出来的 Linux 二进制会动态链接到 glibc，而 Ubuntu 不同小版本的 glibc ABI 偶尔不兼容。musl 静态二进制不依赖任何系统库，"一次编译到处运行"；加上 zigbuild 解决了 Rust musl 链接器缺失的问题，是个人站部署的"省心组合"。

### 服务器部署与验证

本节记录本站曾经的**手工部署**流程（系列一的主线）。部署系列收尾后，这套流程已由 CD 流水线自动接管（见《[从零重建到一键部署](/articles/04-rebuild-to-cd-pipeline/)》）。保留手工步骤仍有价值——它是理解流水线每一步在做什么的地图。

#### 1. 上传二进制

```bash
# 在本地
scp target/x86_64-unknown-linux-musl/release/rcrwhyg-server \
    rcrwhyg@<VPS_HOST>:/opt/rcrwhyg/rcrwhyg-server
```

#### 2. 上传前端资源

```bash
rsync -avz --delete target/site/ \
    rcrwhyg@<VPS_HOST>:/opt/rcrwhyg/site/
```

> **⚠️ 注意**
> 手工时期的部署路径并不统一（二进制曾在 `/root/apps/`）；重建后已统一到 `/opt/rcrwhyg/` 下、专用的 `rcrwhyg` 用户管理，并由 CD 流水线全自动执行。

#### 3. 写入环境变量

服务器端需要 `.env` 文件（owner 0640，组 `rcrwhyg`）：

```ini
# /opt/rcrwhyg/.env
DATABASE_URL=<连接串，格式见仓库 .env.example；生产口令只存在服务器 .env，绝不写入文章>
COOKIE_SECURE=true
SESSION_TTL_HOURS=72
RATE_LIMIT_PUBLIC_PER_MIN=180
RATE_LIMIT_AUTH_PER_MIN=8
LEPTOS_SITE_ROOT=site
LEPTOS_SITE_PKG_DIR=pkg
LEPTOS_OUTPUT_NAME=rcrwhyg-server
LEPTOS_SITE_ADDR=127.0.0.1:3000
LEPTOS_ENV=PROD
```

> **⚠️ 注意**
> 生产数据库务必使用独立账户 + 随机强口令；口令只存在于服务器 `.env`（权限 `root:rcrwhyg 0640`）与运维的密码管理器，不写入仓库与文章。

#### 4. 启动/重启 systemd 服务

```bash
# 首次部署：启动
sudo systemctl start rcrwhyg

# 后续部署：先停再起（避免并发连接到旧进程导致的数据竞争）
sudo systemctl stop rcrwhyg
sudo systemctl start rcrwhyg

# 看状态
sudo systemctl status rcrwhyg-server --no-pager
```

#### 5. 验证

```bash
# 本地后端健康检查
curl -i http://127.0.0.1:3000/health
# 应该看到 200 + {"ok":true,"db":"connected" | "unset"}

# 首页 SSR
curl -i http://127.0.0.1:3000/
# 应该看到 200 + HTML（Leptos SSR 输出）

# 公网域名（通过 Caddy）
curl -I https://rcrwhyg.com/
# 应该看到 200 + Strict-Transport-Security 等安全头

# 强制 HTTPS 重定向测试
curl -I http://rcrwhyg.com/
# 应该看到 308 → https://
```

#### 6. 看日志

```bash
# 后端日志
sudo journalctl -u rcrwhyg -f --no-pager

# Caddy 访问日志
sudo tail -f /var/log/caddy/access.log
```

## 写在最后（系列一收官）

至此，"如何完成全栈应用线上部署"这个系列告一段落：

- **（一）**《让 2C2G 服务器跑得更稳》——Swap 与 PostgreSQL 调优，把"丐版"地基打扎实；
- **（二）· 本文**《Caddy 网关与首次上线》——网关、Caddyfile、编译构建，与"代码到域名访问"的全链路；
- 序列也沉淀了手工部署的完整地图（编译 → 上传 → 起服 → 三层验证）。

一个诚实的提醒：**手工部署适合"第一次"，不适合"每一次"**。首站通线的第二天，我把服务器从零重建，把这套链路全部交给一条 CD 流水线——`git tag v0.1.0 && git push` 即上线。那趟重建踩了一长串"看着很小、咬起来很疼"的坑：《[从零重建到一键部署：一条 CD 流水线的诞生](/articles/04-rebuild-to-cd-pipeline/)》便是记录，欢迎继续。

> 部署过程中遇到过哪些 Caddy / 编译 / 链路相关的坑？欢迎在评论区交流。

## 总结

### 核心要点

1. **Caddy 是个人站的"省心网关"**：自动 HTTPS + 简洁配置 + 性能足够，胜过 Nginx 一点点
2. **Caddyfile 三个核心块**：顶部 options（email） + 站点块（encode / reverse_proxy / header） + 可选 log
3. **Leptos 全栈简化了反代策略**：HTML / WASM / API / SSE / WS 全部由同一后端进程提供，**所有请求一律 reverse_proxy 即可**，不需要 path 拆分
4. **musl + zigbuild 是跨平台 Rust 部署的"省心组合"**：静态二进制，跨 Ubuntu 小版本运行无虞
5. **部署四步走**：编译 → 上传 → 起服 → 验证；其中"验证"包含三层：本地后端 / 本地反代 / 公网域名
6. **`/health` 404 是版本信号**：如果遇到，恭喜你部署的是旧版——重新构建一次即可

## 参考资料

1. Caddy 官方文档：https://caddyserver.com/docs/
2. Caddyfile 语法：https://caddyserver.com/docs/caddyfile
3. Caddy `header` 指令：https://caddyserver.com/docs/caddyfile/directives/header
4. cargo-leptos 文档：https://github.com/leptos-rs/cargo-leptos
5. cargo-zigbuild 文档：https://github.com/rust-cross/cargo-zigbuild
6. Leptos 部署指南：https://leptos.dev/docs/deployment
7. HSTS Preload List：https://hstspreload.org/
8. musl libc：https://musl.libc.org/

**版本信息**: 本文基于 Caddy 2.11.4 / Leptos 0.8.20 / Axum 0.8.9 / Rust 1.98.0 / Ubuntu 26.04 LTS，写于 2026-08。

---

**版权声明**: 本文原创发布于个人网站 https://rcrwhyg.com/articles/03-deploy-full-stack-part-2/，作者：如春日午后阳光。未经授权请勿转载。
