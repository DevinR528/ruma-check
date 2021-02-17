use syntax::{
    ast::{self, AstNode, AstToken},
    SourceFile, SyntaxKind, SyntaxNode,
};

use crate::rules::{NodeRule, TokenRule, ValidationError};

#[derive(Debug, Default)]
pub struct ImportsRule {
    found: Vec<(ast::Path, Option<ast::Whitespace>)>,
}

impl ImportsRule {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}
impl NodeRule for ImportsRule {
    fn apply_rule(&mut self, node: &SyntaxNode) {
        if let Some(import) = ast::Use::cast(node.clone()) {
            if let Some(path) = import.use_tree().unwrap().path() {
                self.found.push((
                    path,
                    node.next_sibling_or_token()
                        .and_then(|t| t.into_token())
                        .and_then(ast::Whitespace::cast),
                ));
            }
        }
    }

    fn match_node(&self, node: &SyntaxNode) -> bool {
        node.kind() == SyntaxKind::USE
    }

    fn validate(&self) -> Result<(), ValidationError> {
        println!(
            "{:?}",
            self.found
                .iter()
                .map(|(p, w)| (p.to_string(), w))
                .collect::<Vec<_>>()
        );
        Ok(())
    }
}
