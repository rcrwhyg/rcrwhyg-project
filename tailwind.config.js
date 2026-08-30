/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["./*.html", "./src/**/*.rs"],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
        extract: {
            rs: (content) => {
                const matches = content.match(/class\s*=\s*["'{][^"'}]*["'}]/g);
                if (!matches) return [];

                return matches.flatMap(match => {
                    const classContent = match.match(/["']{([^}]+)}["']/)?.[1] ||
                        match.match(/["']([^"']+)["']/)?.[1];
                    return classContent ? classContent.split(/\s+/) : [];
                });
            }
        },
    },
    // 这些 class 在 .rs 里是 format!("<class> {}", ...) 拼出来的，content
    // scan 看不到，所以手动列出来防止被 tree-shake 掉。
    //
    // 规则：
    // 1. 任何 `attr:class={format!("xxx {}", y)}` 的写法，xxx 必须在这里。
    // 2. xxx 跟其它 modifier（s-mint / s-sky / s-mix）也要逐条列出，
    //    safelist 不支持自动展开 modifier。
    // 3. 不想加 safelist 也可以：把 format 拆成字面量分支
    //    `if y { "xxx mint" } else { "xxx sky" }`，字面量会被 scan 到。
    // 详见 articles/06-ai-collab-engineering-lessons.md "坑 1" 节。
    safelist: [
        "section-card",
        "section-card::before",
        "section-card.s-mint",
        "section-card.s-sky",
        "section-card.s-mix",
        "lab-card",
        "radar-row",
        "radar-row.s-mint",
        "radar-row.s-sky",
    ],
    theme: {
        extend: {
            fontFamily: {
                display: ['Orbitron', '"Noto Sans SC"', 'sans-serif'],
                mono: ['"Share Tech Mono"', 'ui-monospace', 'monospace'],
                body: ['"Noto Sans SC"', 'system-ui', 'sans-serif'],
            },
        },
    },
    plugins: [],
}
