use pulldown_cmark::{Parser, Options, html};
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWith  Endings};

pub struct MarkdownParser {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl MarkdownParser {
    pub fn new() -> Self {
        MarkdownParser {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn parse_to_html(&self, markdown_input: &str) -> String {
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

    pub fn highlight_code(&self, code: &str, language: &str) -> String {
        let syntax = self.syntax_set.find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut h = syntect::highlighting::Highlighter::new(&self.theme_set.themes["base16-ocean.dark"]);
        
        let mut highlighted_code = String::new();
        for line in LinesWithEndings::new(code) {
            let regions = h.highlight_line(line, syntax).unwrap();
            highlighted_code.push_str(&as_24_bit_terminal_escaped(&regions[..], true));
        }
        highlighted_code
    }
}
