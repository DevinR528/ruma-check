use std::path::{Path, PathBuf};

use syntax::{
    ast::{self, AstNode},
    SourceFile, SyntaxNode, SyntaxToken,
};

use crate::error::ValidationError;

mod imports;

pub trait NodeRule {
    fn name(&self) -> &str;
    fn apply_rule(&mut self, node: &SyntaxNode);
    fn match_node(&self, node: &SyntaxNode) -> bool;
    fn validate(&self) -> Result<(), ValidationError>;
}

pub trait TokenRule {
    fn apply_rule(&mut self, node: &SyntaxToken);
    fn match_node(&self, node: &SyntaxToken) -> bool;
    fn validate(&self) -> Result<(), ValidationError>;
}

pub fn validate_source<P: AsRef<Path>>(text: &str, path: &P) -> Result<(), Vec<ValidationError>> {
    let mut errors = vec![];

    let source = SourceFile::parse(&text)
        .ok()
        .expect("TODO: SyntaxError's from SourceFile::parse");

    // println!("{:#?}", source.syntax());

    let mut rules = init_rules();

    for child in source.syntax().descendants() {
        let mut apply = rules
            .iter_mut()
            .filter(|r| r.match_node(&child))
            .collect::<Vec<_>>();

        for rule in &mut apply {
            rule.apply_rule(&child);
        }
    }

    for rule in &mut rules {
        if let Err(e) = rule.validate() {
            errors.push(e);
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}

fn init_rules() -> Vec<Box<dyn NodeRule>> {
    vec![imports::ImportsRule::new()]
}

#[test]
fn validate_import() {
    let text = r#"use std::{fs, path::Path};

use syntax::{
    ast::{
        AstNode, AstToken, {self},
    },
    SourceFile,
};
"#;
    println!(
        "{}",
        validate_source(text, &PathBuf::from("src/rules.rs"))
            .unwrap_err()
            .first()
            .unwrap()
    );
}
