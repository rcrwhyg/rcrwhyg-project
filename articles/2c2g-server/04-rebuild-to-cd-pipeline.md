# 从零重建到一键部署：一条 CD 流水线的诞生

> **摘要**: 一台个人站服务器从"能跑就行"到被从零重建、并接上 CD 流水线的完整记录。本文不按教程顺序复盘，而是以**踩坑为纲**——cargo-leptos 多 bin 报错、Zig 换了下载地址、GitHub Action 静默空跑、scp 把文件传进嵌套目录……每个坑都按"现象 → 根因 → 修法"展开。读完你会理解：个人站的"一键发布"，本质是把每一步手动操作，诚实且可复查地交给机器。

## 目录

1. [为什么要把服务器重来一遍](#为什么要把服务器重来一遍)
2. [从零铺设：系统与基础服务](#从零铺设系统与基础服务)
3. [一条 CD 流水线的诞生](#一条-cd-流水线的诞生)
4. [踩过的七个坑](#踩过的七个坑)
5. [现在的状态](#现在的状态)
6. [总结](#总结)
7. [参考资料](#参考资料)

## 为什么要把服务器重来一遍

起因是老服务器进入"能跑就行"的松散状态：系统还是 Ubuntu 24.04 出厂默认，应用二进制躺在 `/root/apps/` 下、用 **root** 身份跑，连健康检查端点都没有；部署靠手动 scp + 手敲 systemctl，改一次代码要整套手工流程重走一遍。

正好本地仓库早已把部署方案写成了"配置即代码"（`deploy/` 目录 + ADR 文档），我做重来一遍之前先定了三件事：**重装系统**（升级到 Ubuntu 26.04 LTS）、**数据库换 PGDG 官方源装 PostgreSQL 18**、**把服务器对齐到仓库里设计的"专用用户 + /opt/rcrwhyg 布局 + systemd 加固"**。顺便把一直缺的 CI/CD 一起补上——这就是本篇文章的记录。

## 从零铺设：系统与基础服务

重装后按仓库里的幂等脚本把地基打牢（细节见仓库 `docs/deploy-vps.md`，这里不重复步骤，只讲值得记住的结论）：

- **系统**：阿里云 ECS 重置为 Ubuntu 26.04 LTS（resolute），域名备案与公网 IP 都不受影响；
- **数据库**：PGDG 官方 apt 源装 PostgreSQL 18。**一个容易误导人的点**：网上的教程普遍写 `/var/lib/pgsql/<版本>/data`，但那是 **RPM 发行版（RHEL/Fedora）** 的路径；Ubuntu/Debian 上 PGDG 依然是 Debian 集群布局——配置在 `/etc/postgresql/18/main/postgresql.conf`，服务是 `postgresql@18-main`；
- **反代**：Caddy 2.11，自动签 Let's Encrypt 证书，只把 22/80/443 放给公网；
- **应用布局**：专用 `rcrwhyg` 系统用户 + `/opt/rcrwhyg/{rcrwhyg-server, site/, bin/, var/}`，`systemctl` 只给流水线账号开几条白名单，后端只监听 `127.0.0.1:3000`——Caddy 在前面兜流量。

## 一条 CD 流水线的诞生

先厘清"推代码"与"发版"的区别——CD 只认 **`v*` 标签**，不是每次 push 都触发：

```text
git push origin master            ：普通推送，只跑 CI（格式/clippy/测试/wasm/文章检查），不部署
git tag v0.1.0 && git push origin v0.1.0 ：发版标签，触发 CI + CD，构建并部署上线
```

流水线在 GitHub Actions 里长这样：

```text
push tag v* ──► 构建 site/（cargo leptos build --release）
            ──► 构建 musl 静态二进制（cargo zigbuild）
            ──► scp 二进制 + tar 流式上传 site/ 与 articles/
            ──► 停服务 → 原子替换 → 起服务 → smoke 探测 /health → 写 deploy.log
```

部署脚本 `deploy/remote.sh` 是仓库里的版本化文件，每次部署都跟随最新代码上传——服务器上没有"漂移的配置"。

## 踩过的七个坑

这是本文的核心。每个坑都按"现象 → 根因 → 修法"来。

### 坑 1：cargo-leptos 分不清两个 bin

**现象**：CD 构建第一步就挂：`Several bin targets found for member "rcrwhyg-server"`。

**根因**：cargo-leptos 需要知道哪个 `[[bin]]` 是 web 应用；而仓库新增了 `create-admin` 这个 CLI bin 后它不会自己猜。

**修法**：在 `Cargo.toml` 的 `[package.metadata.leptos]` 里显式声明：

```toml
bin-target = "rcrwhyg-server"
```

### 坑 2：Zig 换了下载地址，setup-zig 还在用旧 URL

**现象**：`mlugg/setup-zig@v1` 报 404，镜像全 503/502，最后 `zig: command not found`。

**根因**：2026 年 ziglang 把产物路径从 `ziglang.org/builds/zig-linux-x86_64-<ver>.tar.xz` 迁到了 `ziglang.org/download/<ver>/zig-x86_64-linux-<ver>.tar.xz`，第三方 action 没跟上。

**修法**：不引第三方分支，直接按新 URL 下载安装（本仓库固定 0.16.0，与本地 brew 一致）：

```bash
curl -fsSLo zig.tar.xz "https://ziglang.org/download/${ZIG_VER}/zig-x86_64-linux-${ZIG_VER}.tar.xz"
tar -xJf zig.tar.xz -C /usr/local/lib
```

### 坑 3：Zig 的两连击——`lib/` 目录 & PATH 生效时机

**现象**：二进制拷到 `/usr/local/bin/zig` 后，`zig cc` 报 `unable to find zig installation directory`；改成整目录后又报 `zig: command not found`。

**根因**：① Zig 靠可执行文件路径向上找它的 `lib/` —— 只拷单个二进制等于把"家"丢了，必须整目录解压；② GitHub Actions 里 `"$GITHUB_PATH"` 只对**后面的步骤**生效，当前步骤里还找不到 `zig`。

**修法**：整目录放 `/usr/local/lib/zig-.../` 并把它加进 `$GITHUB_PATH`；当前步骤内用绝对路径调用，`lib/` 才能被正确解析。

### 坑 4：环境保护规则不放行 tag

**现象**：CD job 直接失败：`Tag "v0.1.0" is not allowed to deploy to production`。

**根因**：GitHub `production` 环境的 *Deployment branches and tags* 保护规则没把 `v*` tag 加进去（只认分支/或规则没生效）。这是配置问题，不是代码问题。

**修法**：环境设置里把 tag pattern `v*` 加进放行列表（或干脆选 *All branches and tags*——反正流水线本身只在 `v*` 触发）。配好环境再推 tag。

### 坑 5：ssh-action 的 `command` 被静默忽略

**现象**：部署步骤全部显示"成功"，但服务器上什么都没发生（服务没停、文件没换、日志没写）。CI 日志里只有一行警告：`Unexpected input(s) 'command'`。

**根因**：`appleboy/ssh-action@v1` 只认 **`script`**，不认 `command`（后者是旧版名字）。而**未识别的输入被直接忽略**，没有报错——于是每个 ssh 步骤都在"空跑"。

**修法**：改成 `script`；后来我们把所有 SSH/SCP 均改为 runner 自带 openssh（见坑 6），从根上避开这类"静默不同步"的接口。

### 坑 6：scp-action 把我文件传进嵌套目录

**现象**：部署"成功"后，在服务器上看到 `/opt/rcrwhyg/bin/deploy/remote.sh`、`/opt/rcrwhyg/target/x86_64-unknown-linux-musl/.../rcrwhyg-server` 这样的目录树——文件全进错了位置。

**根因**：`scp-action` 会**保留 source 的相对路径**解压到 target，并不会"拍平"到 target 根。

**修法**：弃用第三方传文件 action，改用 `scp` 精确传二进制、`tar` 流式传目录（`tar -cf - -C site . -C . articles | ssh '... tar -xf -'`）。**git tag** 一按，文件落在确定的位置。

### 坑 7：finalize 的清理管道被 `set -e` 误杀

**现象**：swap、start、smoke 全部通过（`/health` 都返回 `{"ok":true,"db":"connected"}` 了），却在最后一步以 exit 2 收场。

**根因**：`ls xxx.prev.* 2>/dev/null | tail | xargs` 在**没有任何 .prev 目录**（首次部署）时，`ls` 退出码为 1，配合 `set -euo pipefail` 整条管道被判失败。

**修法**：管道结尾加 `|| true`。一行注释：首次部署没有历史可清，不该算失败。

## 现在的状态

- `https://rcrwhyg.com` 与 `www` 双域名 200，HSTS（含 preload）、nosniff 等安全头齐全；
- `GET /health` 返回 `{"ok":true,"db":"connected"}`——旧机器上缺失的健康检查桥接完成；
- 发布 = `git tag vX.Y.Z && git push origin vX.Y.Z`；回滚 = 把 tag 指向历史好 sha 再推一次；
- 每次部署的 `date / tag / sha / 操作人 / 原因` 都记进 `/opt/rcrwhyg/var/deploy.log`；
- 数据库口令只在服务器 `.env`（`root:rcrwhyg 0640`）与运维密码管理器里，不出现在本仓库。

## 总结

### 核心要点

1. **"重来一遍"的价值在于对齐**：趁重装把"能跑就行"变成"配置即代码"，后续就是 git tag 一件事
2. **第三方 action 也是"供应商"**：接口变化（`command`→`script`、URL 迁移）不声张就会静默咬人；要么读官方文档核对输入名，要么退回 openssh 原语
3. **日记式部署**：deploy.log 记录每个版本的 tag/sha/操作人，回滚不是玄学
4. **副作用要显式兜底**：`set -euo pipefail` 是朋友，但像"没有旧快照可清理"这种 no-op 要 `|| true`
5. **布局与权限的最小面**：专用用户、白名单 sudoers、只监听回环——安全头与"看不见的端口"同样重要
6. **健康检查是发布的门**：没有 `/health`，自动部署就无从谈"验证成功"

## 参考资料

1. GitHub Actions 文档：https://docs.github.com/en/actions
2. Zig 官方下载：https://ziglang.org/download/
3. cargo-zigbuild：https://github.com/rust-cross/cargo-zigbuild
4. cargo-leptos：https://github.com/leptos-rs/cargo-leptos
5. Caddy 官方文档：https://caddyserver.com/docs/
6. PGDG apt 源说明：https://wiki.postgresql.org/wiki/Apt
7. systemd 单元加固指引：https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html

**版本信息**: 本文基于 Ubuntu 26.04 LTS (resolute) / PostgreSQL 18 (PGDG) / Caddy 2.11.4 / Rust 1.97.1 / Leptos 0.8.20，写于 2026-08。

---

**版权声明**: 本文原创发布于个人网站 https://rcrwhyg.com/articles/04-rebuild-to-cd-pipeline/，作者：如春日午后阳光。未经授权请勿转载。