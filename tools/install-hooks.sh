#!/usr/bin/env bash
# rcrwhyg Git 钩子安装脚本
# 将 hooks/ 下的 pre-commit / pre-push 安装到 .git/hooks/

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo "=========================================="
echo "Git 钩子安装脚本"
echo "=========================================="
echo ""

if [ ! -d ".git" ]; then
    echo -e "${RED}[ERROR]${NC} 当前目录不是 Git 仓库，请在项目根目录运行"
    exit 1
fi
mkdir -p .git/hooks

install_hook() {
    local name="$1"
    if [ -f "hooks/$name" ]; then
        cp "hooks/$name" ".git/hooks/$name"
        chmod +x ".git/hooks/$name"
        chmod +x "hooks/$name"
        echo -e "${GREEN}[OK]${NC} 已安装 $name 钩子"
    else
        echo -e "${YELLOW}[WARN]${NC} 未找到 hooks/$name，跳过"
    fi
}

install_hook pre-commit
install_hook pre-push

echo ""
echo -e "${GREEN}[SUCCESS]${NC} Git 钩子安装完成"
echo ""
echo "  git commit  -> 运行轻量校验（格式 + 敏感信息 + 文章静态检查）"
echo "  git push    -> 运行完整门禁（格式 + clippy + 测试 + wasm + 文章）"
echo "  紧急跳过（不推荐）：--no-verify"
echo ""
