/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        relative: true,
        files: ["./*.html", "./src/**/*.rs"],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
        extract: {
            // 自定义 Rust 文件的类名提取规则
            rs: (content) => {
                // 匹配 class="xxx" 和 class={"xxx"}
                const matches = content.match(/class\s*=\s*["'{][^"'}]*["'}]/g);
                if (!matches) return [];

                // 提取引号中的内容
                return matches.flatMap(match => {
                    const classContent = match.match(/["']{([^"'}]+)["'}]/)?.[1] ||
                        match.match(/["']([^"']+)["']/)?.[1];
                    return classContent ? classContent.split(/\s+/) : [];
                });
            }
        },
    },
    theme: {
        extend: {},
    },
    plugins: [],
}