#![allow(unused)]

use std::{
    fmt, iter,
    sync::{Arc, Mutex},
};

use base_db::{
    salsa::{self, Durability},
    AnchoredPath, CrateDisplayName, CrateGraph, CrateId, Env, FileId, FileLoader,
    FileLoaderDelegate, FileSet, SourceDatabase, SourceDatabaseExt, SourceRoot,
    SourceRootId, Upcast, VfsPath,
};
use hir::Semantics;
use hir_def::db::DefDatabase;
use hir_expand::db::AstDatabase;
use hir_ty::db::HirDatabase;
use ide_db::symbol_index::{self, SymbolsDatabase};
use rustc_hash::FxHashSet;
use syntax::{
    ast, ast::MacroCall, ted, AstNode, NodeOrToken, SyntaxKind, SyntaxKind::*,
    SyntaxNode, WalkEvent, T,
};

#[salsa::database(
    base_db::SourceDatabaseExtStorage,
    base_db::SourceDatabaseStorage,
    hir_expand::db::AstDatabaseStorage,
    hir_def::db::InternDatabaseStorage,
    hir_def::db::DefDatabaseStorage,
    hir_ty::db::HirDatabaseStorage,
    symbol_index::SymbolsDatabaseStorage
)]
#[derive(Default)]
pub struct MacroExpander {
    storage: salsa::Storage<MacroExpander>,
    events: Mutex<Option<Vec<salsa::Event>>>,
}

impl fmt::Debug for MacroExpander {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TestDB").finish()
    }
}

impl Upcast<dyn AstDatabase> for MacroExpander {
    fn upcast(&self) -> &(dyn AstDatabase + 'static) { &*self }
}

impl Upcast<dyn DefDatabase> for MacroExpander {
    fn upcast(&self) -> &(dyn DefDatabase + 'static) { &*self }
}

impl Upcast<dyn HirDatabase> for MacroExpander {
    fn upcast(&self) -> &(dyn HirDatabase + 'static) { &*self }
}

impl salsa::Database for MacroExpander {
    fn salsa_event(&self, event: salsa::Event) {
        let mut events = self.events.lock().unwrap();
        if let Some(events) = &mut *events {
            events.push(event);
        }
    }
}

impl salsa::ParallelDatabase for MacroExpander {
    fn snapshot(&self) -> salsa::Snapshot<MacroExpander> {
        salsa::Snapshot::new(MacroExpander {
            storage: self.storage.snapshot(),
            events: Default::default(),
        })
    }
}

impl FileLoader for MacroExpander {
    fn file_text(&self, file_id: FileId) -> Arc<String> {
        FileLoaderDelegate(self).file_text(file_id)
    }
    fn resolve_path(&self, path: AnchoredPath) -> Option<FileId> {
        FileLoaderDelegate(self).resolve_path(path)
    }
    fn relevant_crates(&self, file_id: FileId) -> Arc<FxHashSet<CrateId>> {
        FileLoaderDelegate(self).relevant_crates(file_id)
    }
}

impl MacroExpander {
    /// Parse "files" into a `TypeResolver` that can walk the items in a crate.
    ///
    /// The crate root must be the first file.
    pub fn parse_crate(inputs: Vec<&str>) -> (Self, Vec<FileId>) {
        let mut ids = vec![];
        let mut db = MacroExpander::default();

        let mut files = FileSet::default();
        for i in (0..inputs.len()).map(|i| FileId(i as u32)) {
            ids.push(i);
            files.insert(i, VfsPath::new_virtual_path("/test".to_string()));
        }

        let root = SourceRoot::new_library(files);
        // Since we will never change the DB set high durability.
        let durability = Durability::HIGH;

        // TODO: loop over crates when we do that...
        let lib_roots = root
            .iter()
            .enumerate()
            .map(|(i, _)| SourceRootId(i as u32))
            .collect::<FxHashSet<_>>();

        db.set_library_roots_with_durability(Arc::new(lib_roots), durability);

        for (i, file_id) in root.iter().enumerate() {
            db.set_file_source_root_with_durability(
                file_id,
                SourceRootId(i as u32),
                durability,
            );
        }

        for (file_id, text) in
            inputs.iter().enumerate().map(|(i, t)| (FileId(i as u32), t))
        {
            let _source_root_id = db.file_source_root(file_id);
            // XXX: can't actually remove the file, just reset the text
            db.set_file_text_with_durability(
                file_id,
                Arc::new(text.to_string()),
                durability,
            )
        }

        let mut crate_graph = CrateGraph::default();
        crate_graph.add_crate_root(
            ids[0],
            base_db::Edition::Edition2018,
            Some(CrateDisplayName::from_canonical_name("test".to_string())),
            Default::default(),
            Env::default(),
            vec![],
        );

        db.set_crate_graph_with_durability(Arc::new(crate_graph), durability);

        (db, ids)
    }
}

fn is_text(k: SyntaxKind) -> bool { k.is_keyword() || k.is_literal() || k == IDENT }

fn insert_whitespaces(syn: SyntaxNode) -> String {
    let mut res = String::new();
    let mut token_iter = syn
        .preorder_with_tokens()
        .filter_map(|event| {
            if let WalkEvent::Enter(NodeOrToken::Token(token)) = event {
                Some(token)
            } else {
                None
            }
        })
        .peekable();

    let mut indent = 0;
    let mut last: Option<SyntaxKind> = None;

    while let Some(token) = token_iter.next() {
        let mut is_next = |f: fn(SyntaxKind) -> bool, default| -> bool {
            token_iter.peek().map(|it| f(it.kind())).unwrap_or(default)
        };
        let is_last = |f: fn(SyntaxKind) -> bool, default| -> bool {
            last.map(f).unwrap_or(default)
        };

        match token.kind() {
            k if is_text(k) && is_next(|it| !it.is_punct(), true) => {
                res.push_str(token.text());
                res.push(' ');
            }
            L_CURLY if is_next(|it| it != R_CURLY, true) => {
                indent += 1;
                if is_last(is_text, false) {
                    res.push(' ');
                }
                res.push_str("{\n");
                res.extend(iter::repeat(" ").take(2 * indent));
            }
            R_CURLY if is_last(|it| it != L_CURLY, true) => {
                indent = indent.saturating_sub(1);
                res.push('\n');
                res.extend(iter::repeat(" ").take(2 * indent));
                res.push('}');
                if is_next(|it| !it.is_punct(), false) {
                    indent = indent.saturating_sub(1);
                    res.push('\n');
                }
            }
            R_CURLY => {
                res.push_str("}\n");
                res.extend(iter::repeat(" ").take(2 * indent));
            }
            LIFETIME_IDENT if is_next(|it| it == IDENT, true) => {
                res.push_str(token.text());
                res.push(' ');
            }
            T![;] => {
                res.push_str(";\n");
                res.extend(iter::repeat(" ").take(2 * indent));
            }
            T![->] => res.push_str(" -> "),
            T![=] => res.push_str(" = "),
            T![=>] => res.push_str(" => "),
            _ => res.push_str(token.text()),
        }

        last = Some(token.kind());
    }
    res
}

pub fn expand_macros<Db: HirDatabase>(
    db: &Semantics<Db>,
    mac: &MacroCall,
) -> Option<SyntaxNode> {
    db.expand(mac)
}

#[test]
fn call_mac_expand() {
    use syntax::AstNode;

    let text = include_str!("../fixtures/mbe.rs");
    let (db, id) = MacroExpander::parse_crate(vec![text]);

    let db = hir::Semantics::new(&db);

    let mac = db.parse(id[0]);
    let mut mac_call = None;
    for n in mac.syntax().descendants() {
        if MacroCall::can_cast(n.kind()) {
            mac_call = MacroCall::cast(n);
            break;
        }
    }
    println!(
        "{}",
        expand_macros(&db, &mac_call.unwrap()).map(insert_whitespaces).unwrap()
    );
}
