use syntax::{
    ast::{self, AstNode, AstToken, TokenTree},
    Direction, SourceFile, SyntaxKind, SyntaxNode, TextRange,
};

use crate::{error::Emitter, rules::NodeRule};

#[derive(Debug, Default)]
pub struct MacroFmt {
    found: Vec<SyntaxNode>,
}

impl MacroFmt {
    pub fn new() -> Box<Self> { Box::new(Self::default()) }
}
impl NodeRule for MacroFmt {
    fn name(&self) -> &str { "Correct formatting of macro calls." }
    fn apply_rule(&mut self, node: &SyntaxNode) {
        if let Some(mac) = ast::MacroCall::cast(node.clone()) {
            dbg!(
                SourceFile::parse(&format!(
                    r#"fn junk() {{
{}
}}"#,
                    mac.syntax().to_string().replace("!", "")
                ))
                .syntax_node()
            );
            println!("{:#?}", mac);
            self.found.push(mac.syntax().clone());
        }
    }

    fn match_node(&self, node: &SyntaxNode) -> bool {
        ast::MacroCall::can_cast(node.kind())
    }

    fn validate(&self, path: &str, emitter: &mut Emitter) -> Result<(), crate::EzError> {
        for mac in &self.found {
            emitter.sugg_with_span(
                "Hello message",
                "",
                mac.text_range(),
                mac.clone(),
                path,
            );
        }

        Ok(())
    }
}
