use std::fmt;

use syntax::{
    ast::{self, AstNode},
    SourceFile, SyntaxError, SyntaxNode, SyntaxToken, TextRange, TextSize,
};

#[derive(Debug)]
pub struct ValidationError {
    pub msg: String,
    pub source: SyntaxNode,
    pub span: TextRange,
    pub file: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.emit_error(), f)
    }
}

impl ValidationError {
    fn emit_error(&self) -> String {
        let mut buffer = String::new();
        let root = util::root_node(&self.source);
        let (row, col) = root.to_string().chars().take(self.span.end().into()).fold(
            (0, 0),
            |(mut r, mut c), ch| {
                if ch == '\n' {
                    r += 1;
                    c = 0;
                } else {
                    c += 1;
                }
                (r, c)
            },
        );

        buffer.push_str(&self.msg);
        buffer.push('\n');
        buffer.push_str(&format!("--> {}:{}:{}", self.file, row, col));

        buffer
    }
}

mod util {
    use syntax::SyntaxKind;

    use super::*;

    pub fn root_node(node: &SyntaxNode) -> SyntaxNode {
        if node.kind() == SyntaxKind::SOURCE_FILE {
            node.clone()
        } else if let Some(p) = node.parent() {
            root_node(&p)
        } else {
            unreachable!()
        }
    }
}
