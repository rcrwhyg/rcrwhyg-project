# 如何完成全栈应用线上部署？（一）：让 2C2G 服务器跑得更稳

> **摘要**: 本文是个人网站全栈部署系列的第一篇。域名 ICP 备案通过后，准备在 2C2G 丐版轻量服务器上正式上线全栈应用，但裸机默认配置远不够稳——一旦内存吃紧就会被 OOM Killer 误杀核心进程。本文系统分享两个关键的基建优化：开启 Swap 给系统加一层内存缓冲、PostgreSQL 参数调优降低 OOM 风险。文中所有命令与参数都给出可直接复制粘贴的版本，并解释为什么这样设置，便于读者照搬完成自己低配服务器的基础环境准备。

## 目录

1. [备案过后：从域名到可上线环境](#备案过后从域名到可上线环境)
2. [基建优化一：开启 Swap](#基建优化一开启-swap)
3. [基建优化二：PostgreSQL 性能调优](#基建优化二postgresql-性能调优)
4. [写在最后](#写在最后)
5. [总结](#总结)
6. [参考资料](#参考资料)

## 备案过后：从域名到可上线环境

网站域名 ICP 备案已经审核通过，可以着手进行服务器配置优化以及个人网站全栈应用部署上线，虽然还没有正式开始写代码，但可以先部署一版简单静态页面。

## 基建优化一：开启 Swap

在云原生时代，我们常被"横向扩展"的理念洗礼，但对于独立开发者或者小微企业来说，每一分钱都要花在刀刃上。面对 2G 甚至 1G/512MB 内存的"丐版"服务器，如果配置不当，系统动辄就会因为 OOM（Out of Memory）杀掉核心进程。

通过以下两步核心优化，我们可以显著提升低配服务器的抗压能力和数据库运行效率。

### 为什么低配机必须开 Swap？

当物理内存耗尽时，Linux 内核会触发 OOM Killer，随机杀掉占用内存高的进程（通常是你的数据库或应用）。Swap（交换分区）系统将暂时不用的内存数据写入磁盘，从而"虚拟"出更多的内存空间。

虽然磁盘（即使是 SSD）的速度远慢于内存，但在低配环境下，Swap 的意义不在于加速，而在于容错。它能给系统提供一个缓冲带，避免因瞬间的流量激增或内存泄漏导致整个服务宕机。

### 配置步骤（以 Ubuntu 为例）

执行以下命令，快速创建一个 4GB 的 Swap 文件（对于 40GB 硬盘来说，划出 10% 买个保险非常划算）：

```bash
# 检查当前 Swap 状态
sudo swapon --show
# 如果没有任何输出，说明当前没有开启 Swap

# 创建 4GB 的交换文件（这里的 4G 可以根据需要调整，2G 内存推荐配 2G-4G 的 Swap）
sudo fallocate -l 4G /swapfile
# (如果 fallocate 提示失败，可以用更通用的 dd 命令：
# sudo dd if=/dev/zero of=/swapfile bs=1M count=4096)

# 设置权限（这一步极其重要，必须保证只有 root 用户能读写这个文件，否则会有安全隐患）
sudo chmod 600 /swapfile

# 格式化为 Swap 格式
sudo mkswap /swapfile

# 启用 Swap
sudo swapon /swapfile
# 此时可以再次检查当前 Swap 状态
# sudo swapon --show
# free -h
# 应该可以看到 Swap 行显示 Total 为 4.0G

# 设置永久生效（持久化设置，防止重启失效）

# 在 /etc/fstab 中添加如下配置
# 备份 fstab 文件（好习惯）
sudo cp /etc/fstab /etc/fstab.bak
# 将配置追加到文件末尾
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

### 关键参数：Swappiness

`swappiness` 决定了内核使用 Swap 的积极程度（0-100）。Linux 默认 `swappiness` 值通常是 60，意味着内存用到 40% 时就开始使用 Swap。对于有数据库的服务器，我们不希望它频繁读写硬盘（慢）。我们希望："除非物理内存真的快满了，否则别碰硬盘"。

对于低配置服务器，建议设置为 10。这样既能保证物理内存被优先使用，又能确保在压力大时及时触发 Swap。

```bash
# 临时生效
sudo sysctl vm.swappiness=10

# 永久生效（写入配置文件）
echo 'vm.swappiness=10' | sudo tee -a /etc/sysctl.conf
```

### 一键脚本（GPT 整理，未在生产实测）

> **⚠️ 注意**
> 下面脚本由 GPT 整理，作者本人未在生产环境跑过。上生产前请先在测试机验证。

```bash
#!/bin/bash
# 2核2G 服务器专用 Swap 开启脚本
# 建议 Swap 大小：与内存 1:1 或 1:2，这里设为 4GB
set -e

SWAP_FILE="/swapfile"
SWAP_SIZE="4G"

echo "1. 正在检查是否存在 Swap..."
if [ $(free | grep -i swap | awk '{print $2}') -gt 0 ]; then
    echo "Swap 已存在，跳过创建。"
else
    echo "2. 正在创建 ${SWAP_SIZE} 的 Swap 文件..."
    sudo fallocate -l $SWAP_SIZE $SWAP_FILE

    echo "3. 设置权限..."
    sudo chmod 600 $SWAP_FILE

    echo "4. 格式化为 Swap..."
    sudo mkswap $SWAP_FILE

    echo "5. 启用 Swap..."
    sudo swapon $SWAP_FILE

    echo "6. 设置永久生效 (添加到 /etc/fstab)..."
    echo "$SWAP_FILE none swap sw 0 0" | sudo tee -a /etc/fstab

    echo "Swap 创建完成！"
fi

echo "7. 调整 Swappiness 为 10 (低配机推荐)..."
sudo sysctl vm.swappiness=10
echo "vm.swappiness=10" | sudo tee -a /etc/sysctl.conf

echo "--- 最终内存状态 ---"
free -h
```

## 基建优化二：PostgreSQL 性能调优

PostgreSQL 默认配置非常保守（通常仅针对 256MB 内存设计）。在低配机上，如果不调整参数，不仅性能差，还容易导致内存碎片。

### 配置文件位置说明

> **⚠️ 注意**
> Ubuntu 24.04 默认源安装的 PostgreSQL 配置文件位于 `/etc/postgresql/<版本>/main/postgresql.conf`。**本系列采用 PGDG 官方 apt 源安装 PostgreSQL 18**，Ubuntu/Debian 上 PGDG 同样是 Debian 集群布局：配置 `/etc/postgresql/18/main/postgresql.conf`、数据目录 `/var/lib/postgresql/18/main`、服务 `postgresql@18-main`。（`/var/lib/pgsql/<版本>/data` 是 RPM 发行版路径，不适用 Ubuntu。）

```bash
sudo vim /etc/postgresql/18/main/postgresql.conf
```

针对 1G-2G 内存的生产环境，建议重点优化以下 8 个参数。

### 1. shared_buffers（共享缓冲区）

这是最关键的参数，PG 独占的内存区域，决定了 PostgreSQL 缓存数据页的大小。

- **官方建议**：25% 的物理内存。例如，1GB 内存设置 256MB。
- **2C2G 建议值：256MB**

理由：我们还有 Rust 应用和 Caddy 要跑。如果设置太大，系统自身的缓存（OS Cache）会受限，容易导致频繁磁盘读写。PostgreSQL 即使 `shared_buffers` 小一点，它也会利用操作系统的 Page Cache，效率依然很高。给 Rust 留点活路。

```ini
shared_buffers = 256MB
```

### 2. max_connections（最大连接数）

每个连接都会消耗少量的 RAM。

- **默认**：100
- **2C2G 建议值：30**

理由：个人网站并发量不超过 50。而且 Leptos 全栈方案将使用 sqlx 的连接池，维持 10-20 个长连接就足够处理几千 QPS 了，设置太大浪费内存。

```ini
max_connections = 30
```

### 3. work_mem（工作内存）

每个查询连接（Query Session）可以独立使用的内存，用于排序（ORDER BY）、去重（DISTINCT）和哈希连接（Hash Join）。

- **默认**：4MB
- **2C2G 建议值：4MB**

理由：100 个连接就会消耗 100 × 4MB = 400MB，如果你的并发连接数较多（如 50 个连接），务必将此值设置小点，防止内存溢出。如果 SQL 写得很烂（大量的复杂排序），且并发很高，这个值大了会导致内存瞬间爆炸。4MB 是个平衡点。

```ini
work_mem = 4MB
```

### 4. maintenance_work_mem（维护工作内存）

主要用于索引重建（CREATE INDEX）、清理（VACUUM）等任务。

- **建议范围**：64MB - 128MB
- **2C2G 建议值：64MB**

理由：即使是低配机，维护操作也需要一定的内存来快速完成，否则 VACUUM 会拖累系统整体性能。

```ini
maintenance_work_mem = 64MB
```

### 5. effective_cache_size（有效缓存大小）

这只是一个估算值，并不实际分配内存。它告诉 PostgreSQL 计划器系统（OS + DB）有多少缓存可用。

- **建议范围**：50% - 75% 的总内存
- **2C2G 建议值：1GB**

理由：除去 Rust 应用，剩下的内存和 Swap 基本都可以被 OS 用作文件缓存。

```ini
effective_cache_size = 1GB
```

### 6. random_page_cost（随机页成本）

SSD 优化。如果云服务器是 SSD 盘（通常都是），默认 4.0 太保守，改成 1.1 让 PG 更倾向于走索引。

**默认值 4.0 的由来**：这个默认值是针对以前的机械硬盘（HDD）设定的。机械硬盘的随机 IO（寻道）速度远慢于顺序 IO，因此默认将随机访问的代价设为顺序访问（seq_page_cost = 1.0）的 4 倍，目的是让 PG 尽量避免走索引扫描（因为索引通常涉及大量随机跳转），转而走全表扫描（顺序读取）。

**为什么要改成 1.1？**

- **物理现实的改变**：现在的云服务器几乎全部采用 SSD。SSD 的特性是随机读取和顺序读取的性能差距极小。
- **优化器的"偏见"**：如果不修改这个值，即使创建了索引，PG 的优化器可能依然会"觉得"走索引太贵（因为代价被乘以 4），从而选择效率低下的全表扫描。

**改成 1.1 的效果**：相当于告诉数据库："我的磁盘随机读取非常快，请大胆地使用索引！"这能显著提升复杂查询（如多表 join 或大表过滤）的响应速度。

```ini
random_page_cost = 1.1
```

### 7. wal_buffers（预写日志缓存）

WAL（Write-Ahead Logging）是 PostgreSQL 保证数据不丢失的核心机制。在修改数据页之前，必须先将这些修改记录到 WAL 日志中并刷入磁盘。

**作用**：是内存中专门用来存放这些"待写入磁盘"的日志记录的缓冲区。

**为什么要设置为 8MB？**

- **默认逻辑**：PG 默认将其设为 `shared_buffers` 的 1/32。如果 `shared_buffers` 是 256MB，那么默认值只有 8MB。
- **实际需求**：对于大多数生产环境，一次事务产生的日志量并不会瞬间撑爆几兆空间。根据以往经验看，8MB 到 16MB 已经足以支撑绝大多数高并发写入场景。
- **2C2G 环境的权衡**：设为 8MB 既能保证在大量写入（如批量 INSERT）时，日志记录能平滑地从内存排队刷向磁盘，又不会浪费宝贵的内存。

```ini
wal_buffers = 8MB
```

### 8. min_wal_size / max_wal_size（预写日志 WAL）

控制磁盘写入频率和空间，防止日志文件无限制增长，吃掉 40G 硬盘。

```ini
min_wal_size = 80MB
max_wal_size = 500MB
```

### 修改后生效

```bash
# 重载配置（无需重启；PGDG on Ubuntu 的单元是 postgresql@18-main）
sudo systemctl reload postgresql@18-main
# 或优雅重启（shared_buffers / max_connections / wal_buffers 需重启生效）
sudo systemctl restart postgresql@18-main
# 更简单： sudo -u postgres pg_ctlcluster 18 main reload|restart
```

## 写在最后

好了，经过这一通操作，这台"丐版"服务器足够满足使用了。

备案通过只是拿到了入场券，而这些优化则是给小破车换上了耐磨的轮胎。虽然现在车上还空无一物，但至少不用担心它在半路抛锚了。

下一篇将进行网关和应用的安装部署。

> 在部署个人项目时，遇到过哪些让你头疼的"内存背刺"时刻？或者你有更好的低配服务器优化心得？欢迎在评论区交流。

## 总结

### 核心要点

1. **Swap 是低配机的安全网，不是加速器**：磁盘速度再慢也比被 OOM Killer 杀掉服务好；4GB Swap + `swappiness=10` 是 2C2G 的实用组合
2. **PostgreSQL 默认配置针对 256MB 内存**：直接跑在 2GB 机器上会触发 OOM 与性能塌方，必须显式调参
3. **八个关键参数都对应真实场景**：`shared_buffers`、`max_connections`、`work_mem`、`maintenance_work_mem`、`effective_cache_size`、`random_page_cost`、`wal_buffers`、`min_wal_size`/`max_wal_size`——8 个值根据内存与磁盘特性逐项推导，不是网上随手抄的模板
4. **SSD 上必须改 `random_page_cost`**：4.0 是 HDD 时代的遗产，1.1 才能让优化器不冤枉索引
5. **所有改动都要 reload，不是 restart**：reload 不切断现有连接，生产更友好
6. **脚本要标注未实测**：GPT 整理的 Swap 脚本标注"未测试"，提醒读者先验证再上生产

## 参考资料

1. PostgreSQL 官方文档：https://www.postgresql.org/docs/
2. PostgreSQL 资源消耗文档：https://www.postgresql.org/docs/current/runtime-config-resource.html
3. Linux man page: swapon(8)：https://man7.org/linux/man-pages/man8/swapon.8.html
4. Linux man page: sysctl(8)：https://man7.org/linux/man-pages/man8/sysctl.8.html
5. Ubuntu 24.04 PostgreSQL 指南：https://help.ubuntu.com/community/PostgreSQL
6. 阿里云轻量应用服务器：https://www.aliyun.com/product/swas
7. 阿里云 PostgreSQL 镜像：https://developer.aliyun.com/mirror/postgresql

**版本信息**: 本文基于 PostgreSQL 18（PGDG 官方源）/ Ubuntu 26.04 LTS（重建时以 24.04 为备选）/ Rust 1.97.1，写于 2026-08。

---

**版权声明**: 本文原创发布于个人网站 https://rcrwhyg.com/articles/02-deploy-full-stack-part-1/，作者：如春日午后阳光。未经授权请勿转载。
