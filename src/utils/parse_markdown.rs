use comrak::{
    adapters::SyntaxHighlighterAdapter, markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins,
};
use std::collections::HashMap;

struct MockAdapter {}

impl SyntaxHighlighterAdapter for MockAdapter {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        String::from(format!(
            "<pre class=\"language-{}\"><code class=\"language-{}\">{}</code></pre>",
            lang.unwrap(),
            lang.unwrap(),
            code
        ))
    }

    fn build_pre_tag(&self, _: &HashMap<String, String>) -> String {
        String::from("")
    }

    fn build_code_tag(&self, _: &HashMap<String, String>) -> String {
        String::from("")
    }
}

pub fn parse_markdown(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    options.extension.strikethrough = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());

    let mut plugins = ComrakPlugins::default();

    let adapter = MockAdapter {};
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    markdown_to_html_with_plugins(markdown, &options, &plugins)
}
