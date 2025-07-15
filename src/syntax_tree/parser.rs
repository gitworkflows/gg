use tree_sitter::{Parser, Language, Node, Tree};
use log::{info, error};

// You would typically load the language dynamically or from a pre-compiled source.
// For demonstration, we'll assume `tree_sitter_bash` is available.
extern "C" { fn tree_sitter_bash() -> Language; }

/// A parser for generating syntax trees from code.
pub struct SyntaxTreeParser {
    parser: Parser,
}

impl SyntaxTreeParser {
    pub fn new() -> anyhow::Result<Self> {
        let mut parser = Parser::new();
        let language = unsafe { tree_sitter_bash() }; // Load Bash language
        parser.set_language(language)?;
        info!("SyntaxTreeParser initialized with Bash language.");
        Ok(SyntaxTreeParser { parser })
    }

    /// Parses a source code string and returns its syntax tree.
    pub fn parse(&mut self, source_code: &str) -> Option<Tree> {
        info!("Parsing source code ({} bytes)...", source_code.len());
        self.parser.parse(source_code, None)
    }

    /// Prints a simplified representation of the syntax tree.
    pub fn print_tree_structure(tree: &Tree, source_code: &str) {
        let mut cursor = tree.walk();
        let mut indent_level = 0;

        info!("--- Syntax Tree Structure ---");
        loop {
            let node = cursor.node();
            let node_text = node.utf8_text(source_code.as_bytes()).unwrap_or("[ERROR DECODING TEXT]");
            info!("{}{} ({}): '{}'",
                "  ".repeat(indent_level),
                node.kind(),
                node.range(),
                node_text.trim()
            );

            if cursor.goto_first_child() {
                indent_level += 1;
                continue;
            }

            if cursor.goto_next_sibling() {
                continue;
            }

            let mut retreated = false;
            while cursor.goto_parent() {
                indent_level -= 1;
                retreated = true;
                if cursor.goto_next_sibling() {
                    break;
                }
            }

            if !retreated {
                break;
            }
        }
        info!("--- End Syntax Tree Structure ---");
    }

    /// Finds all occurrences of a specific node kind in the tree.
    pub fn find_nodes_by_kind<'a>(tree: &'a Tree, kind: &str) -> Vec<Node<'a>> {
        let mut nodes = Vec::new();
        let mut cursor = tree.walk();
        let mut visited_children = false;

        loop {
            let node = cursor.node();
            if node.kind() == kind {
                nodes.push(node);
            }

            if cursor.goto_first_child() {
                visited_children = true;
            } else if cursor.goto_next_sibling() {
                visited_children = false;
            } else {
                loop {
                    if !cursor.goto_parent() {
                        return nodes;
                    }
                    if cursor.goto_next_sibling() {
                        break;
                    }
                }
                visited_children = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_initialization() {
        let parser = SyntaxTreeParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parse_simple_bash() {
        let mut parser = SyntaxTreeParser::new().unwrap();
        let source_code = "echo \"hello world\"";
        let tree = parser.parse(source_code);
        assert!(tree.is_some());
        let tree = tree.unwrap();
        assert_eq!(tree.root_node().kind(), "program");
        assert!(tree.root_node().child_count() > 0);
    }

    #[test]
    fn test_find_nodes_by_kind() {
        let mut parser = SyntaxTreeParser::new().unwrap();
        let source_code = r#"
            #!/bin/bash
            VAR="value"
            if [ "$VAR" = "value" ]; then
                echo "It's a match!"
            fi
        "#;
        let tree = parser.parse(source_code).unwrap();

        let commands = SyntaxTreeParser::find_nodes_by_kind(&tree, "command");
        assert!(!commands.is_empty());
        assert!(commands.iter().any(|n| n.utf8_text(source_code.as_bytes()).unwrap().contains("echo")));

        let variables = SyntaxTreeParser::find_nodes_by_kind(&tree, "variable_name");
        assert!(!variables.is_empty());
        assert!(variables.iter().any(|n| n.utf8_text(source_code.as_bytes()).unwrap() == "VAR"));
    }

    #[test]
    fn test_print_tree_structure() {
        let mut parser = SyntaxTreeParser::new().unwrap();
        let source_code = "ls -la";
        let tree = parser.parse(source_code).unwrap();
        // This test primarily checks if the function runs without panicking
        // and produces log output. Actual output verification would be complex.
        SyntaxTreeParser::print_tree_structure(&tree, source_code);
    }
}
