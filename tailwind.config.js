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
                    const classContent = match.match(/["']{([^"'}]+)["'}]/)?.[1] ||
                        match.match(/["']([^"']+)["']/)?.[1];
                    return classContent ? classContent.split(/\s+/) : [];
                });
            }
        },
    },
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
