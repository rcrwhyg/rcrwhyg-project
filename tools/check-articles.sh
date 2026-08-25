#!/usr/bin/env bash
# 公众号文章静态质量门禁（articles/*.md）
# 检查项（见 rules/content-quality.md 与 specs/article-template.md）：
#   1. 首行为一级标题 `# `
#   2. 含摘要行 `> **摘要**`
#   3. 含「## 参考资料」章节，且其中不使用 Markdown 超链接（须直接展示完整 URL）
#   4. 含版权声明
# 跳过：articles/README.md、articles/templates/

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

shopt -s nullglob
FILES=(articles/*.md)
TOTAL=0
FAILED=0

for f in "${FILES[@]}"; do
    base=$(basename "$f")
    [ "$base" = "README.md" ] && continue
    TOTAL=$((TOTAL + 1))
    errors=""

    # 1. 一级标题
    first_line=$(grep -m1 -v '^[[:space:]]*$' "$f" || true)
    case "$first_line" in
        '# '*) : ;;
        *) errors="${errors}\n    - 缺少一级标题（首行应为 '# 标题'）" ;;
    esac

    # 2. 摘要
    if ! grep -q '^> \*\*摘要\*\*' "$f"; then
        errors="${errors}\n    - 缺少摘要行（'> **摘要**: …'）"
    fi

    # 3. 参考资料 + 纯链接格式
    if ! grep -q '^## 参考资料' "$f"; then
        errors="${errors}\n    - 缺少「## 参考资料」章节"
    else
        # 截取参考资料之后的内容，检查是否用了 Markdown 超链接
        if awk '/^## 参考资料/{flag=1} flag' "$f" | grep -q '\](http'; then
            errors="${errors}\n    - 参考资料中使用了 Markdown 超链接，须改为「资料名称：完整URL」"
        fi
    fi

    # 4. 版权声明
    if ! grep -q '版权声明' "$f"; then
        errors="${errors}\n    - 缺少版权声明"
    fi

    if [ -n "$errors" ]; then
        FAILED=$((FAILED + 1))
        echo -e "${RED}[FAIL]${NC} $f"
        echo -e "$errors"
    else
        echo -e "${GREEN}[OK]${NC} $f"
    fi
done

echo ""
if [ "$TOTAL" -eq 0 ]; then
    echo -e "${YELLOW}[INFO]${NC} articles/ 下暂无文章，跳过"
    exit 0
fi

if [ "$FAILED" -gt 0 ]; then
    echo -e "${RED}[FAIL]${NC} 文章检查：$FAILED/$TOTAL 未通过（规范见 specs/article-template.md）"
    exit 1
fi
echo -e "${GREEN}[SUCCESS]${NC} 文章检查：$TOTAL/$TOTAL 通过"
