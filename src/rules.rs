use std::path::Path;

use syntax::{ast::AstNode, SourceFile, SyntaxNode, SyntaxToken};

use crate::error::Emitter;

mod ban_mod;
mod macro_fmt;

pub trait NodeRule {
    fn name(&self) -> &str;
    fn apply_rule(&mut self, node: &SyntaxNode);
    fn match_node(&self, node: &SyntaxNode) -> bool;
    fn validate(&self, path: &str, emitter: &mut Emitter) -> Result<(), crate::EzError>;
}

pub trait TokenRule {
    fn apply_rule(&mut self, node: &SyntaxToken);
    fn match_node(&self, node: &SyntaxToken) -> bool;
    fn validate(&self, emitter: &mut Emitter) -> Result<(), crate::EzError>;
}

pub fn validate_source<P: AsRef<Path>>(
    path: &P,
    text: &str,
    emitter: &mut Emitter,
) -> Result<(), crate::EzError> {
    let source =
        SourceFile::parse(text).ok().expect("TODO: SyntaxError's from SourceFile::parse");

    // println!("{:#?}", source.syntax());

    let mut rules = init_rules();

    for child in source.syntax().descendants() {
        let mut apply =
            rules.iter_mut().filter(|r| r.match_node(&child)).collect::<Vec<_>>();

        for rule in &mut apply {
            rule.apply_rule(&child);
        }
    }

    for rule in &mut rules {
        rule.validate(
            path.as_ref().to_str().ok_or(format!(
                "Failed to convert path to string `{}`",
                path.as_ref().display()
            ))?,
            emitter,
        )?;
    }

    Ok(())
}

fn init_rules() -> Vec<Box<dyn NodeRule>> {
    vec![ban_mod::BanMod::new(), macro_fmt::MacroFmt::new()]
}

#[test]
fn ban_mod() {
    let text = r#"use std::fs;

use syntax::{
    ast::{
        AstNode, AstToken, {self},
    },
    SourceFile,
};
"#;
    let mut emitter = Emitter::default();
    validate_source(&std::path::PathBuf::from("src/rules/mod.rs"), text, &mut emitter)
        .unwrap();
    emitter.emit().unwrap();
}

#[test]
fn macro_call() {
    let mut emitter = Emitter::default();
    let text = r#"use std::fs;

fn main() {
    macro_call!(
        foo,
        Foo::A(true)
    );
}
"#;
    validate_source(&std::path::PathBuf::from("src/rules.rs"), text, &mut emitter)
        .unwrap();
    emitter.emit().unwrap();
}
