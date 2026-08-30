/// Shared Markdown → HTML (pulldown-cmark). Used by articles, about, etc.
#[cfg(feature = "ssr")]
pub fn markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn markdown_renders_heading() {
        let html = markdown_to_html("## 技术选型\n\nhello");
        assert!(html.contains("<h2>"));
        assert!(html.contains("技术选型"));
        assert!(html.contains("<p>hello</p>"));
    }
}
