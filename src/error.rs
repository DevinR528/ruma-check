use syntax::{SyntaxNode, TextRange};

#[derive(Clone, Debug, Default)]
pub struct Emitter {
    diag: Vec<Diagnostic>,
    source_map: (),
}

impl Emitter {
    pub fn found_errors(&self) -> bool { !self.diag.is_empty() }

    pub fn sugg_with_span(
        &mut self,
        msg: &str,
        sugg: &str,
        span: TextRange,
        node: SyntaxNode,
        file: &str,
    ) {
        self.diag.push(Diagnostic::Spanned(SpannedError {
            msg: msg.to_owned(),
            suggestion: sugg.to_owned(),
            source: node,
            span,
            file: file.to_owned(),
        }));
    }

    pub fn simple_sugg(&mut self, msg: &str, sugg: &str, file: &str) {
        self.diag.push(Diagnostic::Simple(SimpleError {
            msg: msg.to_owned(),
            file: file.to_owned(),
            sugg: sugg.to_owned(),
        }));
    }

    pub fn emit(self) -> std::io::Result<()> {
        for err in self.diag {
            match err {
                Diagnostic::Spanned(spanned) => eprint!("{}", spanned.emit_error()),
                Diagnostic::Simple(simple) => {
                    eprintln!("error: {}", simple.msg);
                    eprintln!("--> {}", simple.file);
                    eprintln!("{}", simple.sugg);
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum Diagnostic {
    Spanned(SpannedError),
    Simple(SimpleError),
}

#[derive(Clone, Debug)]
pub struct SimpleError {
    pub msg: String,
    pub file: String,
    pub sugg: String,
}

#[derive(Clone, Debug)]
pub struct SpannedError {
    pub msg: String,
    pub suggestion: String,
    pub source: SyntaxNode,
    pub span: TextRange,
    pub file: String,
}

impl SpannedError {
    fn emit_error(&self) -> String {
        let mut buffer = String::new();
        let root = util::root_node(&self.source);
        // TODO: do this once and use it by mapping to Span/TextRange?
        let (row, col) = root.to_string().chars().take(self.span.start().into()).fold(
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
        buffer.push_str(&format!("--> {}:{}:{}\n", self.file, row, col));

        let snippet = self.source.to_string() + "\n";
        buffer.push_str(&snippet);

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
