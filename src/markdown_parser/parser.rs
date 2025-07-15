use pulldown_cmark::{Parser, Options, html};

/// A utility for parsing Markdown content into HTML.
pub struct MarkdownParser;

impl MarkdownParser {
    /// Parses a Markdown string into an HTML string.
    ///
    /// # Arguments
    /// * `markdown_input` - The Markdown string to parse.
    ///
    /// # Returns
    /// A `String` containing the rendered HTML.
    pub fn to_html(markdown_input: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = Parser::new_ext(markdown_input, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }

    /// Parses a Markdown string into a plain text string (stripping formatting).
    /// This is a simplified implementation and might not perfectly represent
    /// how a human would read the plain text.
    pub fn to_plain_text(markdown_input: &str) -> String {
        let parser = Parser::new(markdown_input);
        let mut plain_text = String::new();
        for event in parser {
            match event {
                pulldown_cmark::Event::Text(text) => plain_text.push_str(&text),
                pulldown_cmark::Event::Code(code) => plain_text.push_str(&code),
                pulldown_cmark::Event::Html(html) => plain_text.push_str(&html), // Might want to strip HTML tags
                pulldown_cmark::Event::SoftBreak | pulldown_cmark::Event::HardBreak => plain_text.push(' '),
                pulldown_cmark::Event::Start(tag) => {
                    match tag {
                        pulldown_cmark::Tag::Heading(_, _, _) => plain_text.push_str("\n\n"),
                        pulldown_cmark::Tag::Paragraph => plain_text.push_str("\n\n"),
                        pulldown_cmark::Tag::Item => plain_text.push_str("- "),
                        _ => {}
                    }
                },
                pulldown_cmark::Event::End(tag) => {
                    match tag {
                        pulldown_cmark::Tag::Heading(_, _, _) => plain_text.push('\n'),
                        pulldown_cmark::Tag::Paragraph => plain_text.push('\n'),
                        _ => {}
                    }
                },
                _ => {} // Ignore other events like footnotes, links, images etc.
            }
        }
        plain_text.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_html_basic() {
        let markdown = "# Hello\n\nThis is **bold** text.";
        let html = MarkdownParser::to_html(markdown);
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_to_html_tables() {
        let markdown = "| Header 1 | Header 2 |\n|---|---|\n| Cell 1 | Cell 2 |";
        let html = MarkdownParser::to_html(markdown);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>Header 1</th>"));
        assert!(html.contains("<td>Cell 1</td>"));
    }

    #[test]
    fn test_to_plain_text_basic() {
        let markdown = "# Title\n\nSome *italic* and **bold** text.\n\n- List item 1\n- List item 2\n\n`inline code`";
        let plain_text = MarkdownParser::to_plain_text(markdown);
        let expected = "Title\n\nSome italic and bold text.\n\n- List item 1 \n- List item 2 \n\ninline code";
        assert_eq!(plain_text.trim(), expected.trim());
    }

    #[test]
    fn test_to_plain_text_code_block() {
        let markdown = "```rust\nfn main() {}\n```";
        let plain_text = MarkdownParser::to_plain_text(markdown);
        assert_eq!(plain_text.trim(), "fn main() {}");
    }
}
