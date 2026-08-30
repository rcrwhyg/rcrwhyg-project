/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["./*.html", "./style/tailwind.safelist.html", "./src/**/*.rs"],
        transform: {
            // attr:class= → class= so extract regex sees Leptos router links.
            rs: (content) => content.replace(/(?:^|\s)attr:class/g, " class"),
        },
        extract: {
            rs: (content) => {
                const matches = content.match(/class\s*=\s*["'{][^"'}]*["'}]/g);
                if (!matches) return [];

                return matches.flatMap((match) => {
                    const classContent =
                        match.match(/["']{([^}]+)}["']/)?.[1] ||
                        match.match(/["']([^"']+)["']/)?.[1];
                    return classContent ? classContent.split(/\s+/).filter(Boolean) : [];
                });
            },
        },
    },
    // format! / attr:class 动态 class、responsive variant 需 safelist。
    // 详见 articles/2c2g-server/06-ai-collab-engineering-lessons.md "坑 1"。
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
};
