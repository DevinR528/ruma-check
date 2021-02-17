use std::{fmt::Display, fs, path::Path};

use syntax::{
    ast::{
        AstNode, AstToken, {self},
    },
    SourceFile, SyntaxError, SyntaxNode, TextRange, TextSize,
};

mod imports;

#[derive(Debug, Default)]
pub struct ValidationError {
    msg: String,
    source: Option<SyntaxError>,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            Some(err) => write!(f, "{}{}", self.msg, err),
            None => write!(f, "{}", self.msg),
        }
    }
}

impl From<&str> for ValidationError {
    fn from(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
            ..Default::default()
        }
    }
}

impl From<String> for ValidationError {
    fn from(msg: String) -> Self {
        Self {
            msg,
            ..Default::default()
        }
    }
}

pub trait NodeRule {
    fn apply_rule(&mut self, node: &SyntaxNode);
    fn match_node(&self, node: &SyntaxNode) -> bool;
    fn validate(&self) -> Result<(), ValidationError>;
}

pub trait TokenRule: AstToken {
    fn validate(&self) -> Result<(), ValidationError>;
    fn match_node(&self, node: SyntaxNode) -> bool;
}

pub fn validate_source(text: &str) -> Result<(), Vec<ValidationError>> {
    let mut errors = vec![];

    let source = SourceFile::parse(&text)
        .ok()
        .expect("TODO: SyntaxError's from SourceFile parse");

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
    validate_source(text).unwrap();
}
