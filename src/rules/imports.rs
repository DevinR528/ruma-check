use syntax::{
    ast::{self, AstNode, AstToken},
    SyntaxNode, TextRange,
};

use crate::{error::ValidationError, rules::NodeRule};

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
    fn name(&self) -> &str {
        "Imports rule, ruma specifies an ordering of imports."
    }
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
        ast::Use::can_cast(node.kind())
    }

    fn validate(&self) -> Result<(), ValidationError> {
        Err(ValidationError {
            msg: "Hello message".to_string(),
            source: self.found.last().clone().unwrap().0.syntax().clone(),
            span: TextRange::new(3.into(), 8.into()),
            file: "src/rules/imports.rs".to_string(),
        })
    }
}
