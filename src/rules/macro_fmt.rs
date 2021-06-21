#![allow(unused)]

use std::iter;

use syntax::{
    ast::{self, AstNode, AstToken, TokenTree},
    Direction, SourceFile, SyntaxKind, SyntaxNode, SyntaxToken, TextRange, WalkEvent, T,
};

use crate::{error::Emitter, rules::NodeRule};

type NodeOrToken = syntax::NodeOrToken<SyntaxNode, SyntaxToken>;

const LINE_LEN: usize = 80;
const INDENT: usize = 4;

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
            let indent = walk_ancestors_until(mac.syntax(), |kind| {
                println!("{:?}", kind);
                kind == SyntaxKind::WHITESPACE
            })
            .map_or(0, |ws| {
                ws.to_string().replace("\t", "    ").chars().filter(|c| *c == ' ').count()
            });

            if needs_formatting(&mac, indent) {
                self.found.push(mac.syntax().clone());
            }
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

fn needs_formatting(mac: &ast::MacroCall, indent: usize) -> bool {
    let mut token_iter = mac
        .syntax()
        .preorder_with_tokens()
        .filter_map(|event| {
            if let WalkEvent::Enter(NodeOrToken::Token(token)) = event {
                Some(token)
            } else {
                None
            }
        })
        .peekable();

    let mut last = None;
    let mut current_line_len = 0;
    let mut indent_level = indent / INDENT;
    while let Some(token) = token_iter.next() {
        let next = token_iter.peek();
        let mut is_next = |f: fn(SyntaxKind) -> bool, default| -> bool {
            next.map(|it| f(it.kind())).unwrap_or(default)
        };
        let is_last = |f: fn(SyntaxKind) -> bool, default| -> bool {
            last.map(f).unwrap_or(default)
        };
        let is_next_nl = || -> bool {
            matches!(
                next,
                Some(t) if t.kind() == SyntaxKind::WHITESPACE && t.text().contains('\n')
            )
        };

        println!("{} {} {:?} {:?}", current_line_len, indent_level, token, next);

        let text = token.text();
        if text.contains('\n') {
            if current_line_len > LINE_LEN
                || text.chars().filter(|c| *c == ' ').count() != indent_level * INDENT
            {
                return true;
            }
            current_line_len = text.replace("\n", "").len();
        } else {
            current_line_len += text.len();
        }

        match token.kind() {
            SyntaxKind::L_CURLY if is_next(|it| it != SyntaxKind::R_CURLY, true) => {
                indent_level += 1;
            }
            SyntaxKind::R_CURLY if is_next(|it| !it.is_punct(), true) => {
                indent_level = indent_level.saturating_sub(1);
            }
            SyntaxKind::L_PAREN if is_next_nl() => {
                indent_level += 1;
            }
            SyntaxKind::R_PAREN if is_next_nl() => {
                indent_level = indent_level.saturating_sub(1);
            }
            T![!] => {}
            T![;] => {}
            _ => {}
        }

        last = Some(token.kind());
    }

    println!("FALSE");
    true
}

fn walk_ancestors_until(
    node: &SyntaxNode,
    f: impl Fn(SyntaxKind) -> bool,
) -> Option<NodeOrToken> {
    for p in node.ancestors() {
        if f(p.kind()) {
            return Some(NodeOrToken::Node(p));
        }
        let mut sib = p.prev_sibling_or_token();
        while let Some(s) = sib {
            if f(s.kind()) {
                return Some(s);
            }
            sib = s.prev_sibling_or_token();
        }
    }
    None
}
