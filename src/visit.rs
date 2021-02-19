use syntax::{
    ast::{
        self, ArgListOwner, AstNode, AstToken, AttrsOwner, DocCommentsOwner, GenericParamsOwner,
        LoopBodyOwner, ModuleItemOwner, NameOwner, TypeBoundsOwner, VisibilityOwner,
    },
    SourceFile, SyntaxKind, SyntaxNode,
};

pub trait RuleVisitor: Sized {
    fn visit_source_file(&mut self, file: SourceFile) {
        walk_souce_file(self, file)
    }

    fn visit_item(&mut self, item: ast::Item) {
        walk_item(self, item)
    }

    fn visit_comment(&mut self, com: Vec<ast::Comment>) {
        walk_comment(self, com)
    }

    fn visit_const(&mut self, con: ast::Const) {
        walk_const(self, con)
    }

    fn visit_enum(&mut self, enm: ast::Enum) {
        walk_enum(self, enm)
    }

    fn visit_extern_block(&mut self, blk: ast::ExternBlock) {
        walk_extern_block(self, blk)
    }

    fn visit_extern_crate(&mut self, krate: ast::ExternCrate) {
        walk_extern_crate(self, krate)
    }

    fn visit_fn(&mut self, func: ast::Fn) {
        walk_fn(self, func)
    }

    fn visit_impl(&mut self, imp: ast::Impl) {
        walk_impl(self, imp)
    }

    fn visit_mac_call(&mut self, mac_call: ast::MacroCall) {
        walk_mac_call(self, mac_call)
    }

    fn visit_mac_def(&mut self, mac_def: ast::MacroDef) {
        walk_mac_def(self, mac_def)
    }

    fn visit_mac_rules(&mut self, mac_rules: ast::MacroRules) {
        walk_mac_rules(self, mac_rules)
    }

    fn visit_mod(&mut self, module: ast::Module) {
        walk_mod(self, module)
    }

    fn visit_static(&mut self, stat: ast::Static) {
        walk_static(self, stat)
    }

    fn visit_struct(&mut self, struc: ast::Struct) {
        walk_struct(self, struc)
    }

    fn visit_trait(&mut self, trait_: ast::Trait) {
        walk_trait(self, trait_)
    }

    fn visit_type_alias(&mut self, ty_alias: ast::TypeAlias) {
        walk_type_alias(self, ty_alias)
    }

    fn visit_union(&mut self, union_: ast::Union) {
        walk_union(self, union_)
    }

    fn visit_use(&mut self, use_: ast::Use) {
        walk_use(self, use_)
    }

    fn visit_attr(&mut self, attrs: Vec<ast::Attr>) {
        walk_attr(self, attrs)
    }

    fn visit_vis(&mut self, vis: ast::Visibility) {
        walk_vis(self, vis)
    }

    fn visit_type(&mut self, ty: ast::Type) {
        walk_type(self, ty)
    }

    fn visit_variant(&mut self, ty: ast::Variant) {
        walk_variant(self, ty)
    }

    fn visit_struct_fields(&mut self, fields: ast::FieldList) {
        walk_struct_fields(self, fields)
    }

    fn visit_block(&mut self, b: ast::BlockExpr) {
        walk_block_expr(self, b)
    }
    fn visit_stmt(&mut self, s: ast::Stmt) {
        walk_stmt(self, s)
    }
    fn visit_params(&mut self, params: ast::ParamList) {
        walk_param(self, params)
    }
    fn visit_arm(&mut self, a: ast::MatchArm) {
        walk_arm(self, a)
    }
    fn visit_pat(&mut self, p: ast::Pat) {
        walk_pat(self, p)
    }
    fn visit_expr(&mut self, ex: ast::Expr) {
        walk_expr(self, ex)
    }
    fn visit_arg_list(&mut self, args: ast::ArgList) {
        walk_args(self, args)
    }
    fn visit_gen_param(&mut self, ty: ast::GenericParam) {
        walk_gen_param(self, ty)
    }

    fn visit_where_clause(&mut self, where_clause: ast::WhereClause) {
        walk_where_clause(self, where_clause)
    }

    fn visit_where_predicate(&mut self, where_pred: ast::WherePred) {
        walk_where_pred(self, where_pred)
    }

    fn visit_fn_return(&mut self, ret: ast::RetType) {
        walk_fn_ret(self, ret)
    }

    fn visit_assoc_ty(&mut self, assoc_ty: ast::AssocTypeArg) {
        walk_assoc_ty(self, assoc_ty)
    }

    fn visit_name(&mut self, name: ast::Name) {
        walk_name(self, name)
    }
    fn visit_path(&mut self, params: ast::Path) {
        walk_path(self, params)
    }
}

pub fn walk_souce_file<V: RuleVisitor>(vis: &mut V, file: SourceFile) {
    file.syntax()
        .descendants()
        .flat_map(ast::Item::cast)
        .for_each(|it| vis.visit_item(it));
}

pub fn walk_item<V: RuleVisitor>(vis: &mut V, item: ast::Item) {
    println!("ITEM, {:?}", item);
    match item {
        ast::Item::Const(con) => vis.visit_const(con),
        ast::Item::Enum(enm) => vis.visit_enum(enm),
        ast::Item::ExternBlock(blk) => vis.visit_extern_block(blk),
        ast::Item::ExternCrate(krate) => vis.visit_extern_crate(krate),
        ast::Item::Fn(func) => vis.visit_fn(func),
        ast::Item::Impl(imp) => vis.visit_impl(imp),
        ast::Item::MacroCall(mac) => vis.visit_mac_call(mac),
        ast::Item::MacroDef(mac_def) => vis.visit_mac_def(mac_def),
        ast::Item::MacroRules(mac) => vis.visit_mac_rules(mac),
        ast::Item::Module(m) => vis.visit_mod(m),
        ast::Item::Static(stat) => vis.visit_static(stat),
        ast::Item::Struct(struc) => vis.visit_struct(struc),
        ast::Item::Trait(tra) => vis.visit_trait(tra),
        ast::Item::TypeAlias(ty) => vis.visit_type_alias(ty),
        ast::Item::Union(un) => vis.visit_union(un),
        ast::Item::Use(u) => vis.visit_use(u),
    }
}

pub fn walk_comment<V: RuleVisitor>(vis: &mut V, com: Vec<ast::Comment>) {}

pub fn walk_const<V: RuleVisitor>(vis: &mut V, con: ast::Const) {
    vis.visit_comment(con.doc_comments().collect());
    vis.visit_attr(con.attrs().collect());

    if let Some(v) = con.visibility() {
        vis.visit_vis(v);
    }

    vis.visit_type(con.ty().unwrap());

    vis.visit_name(con.name().unwrap());

    if let Some(body) = con.body() {
        vis.visit_expr(body);
    }
}

pub fn walk_enum<V: RuleVisitor>(vis: &mut V, enum_: ast::Enum) {
    vis.visit_comment(enum_.doc_comments().collect());
    vis.visit_attr(enum_.attrs().collect());

    if let Some(v) = enum_.visibility() {
        vis.visit_vis(v);
    }
    vis.visit_name(enum_.name().unwrap());

    for param in enum_
        .generic_param_list()
        .iter()
        .flat_map(|gp| gp.generic_params())
    {
        vis.visit_gen_param(param)
    }

    for var in enum_.variant_list().iter().flat_map(|var| var.variants()) {
        vis.visit_variant(var);
    }
}

pub fn walk_variant<V: RuleVisitor>(vis: &mut V, ty: ast::Variant) {}

pub fn walk_extern_block<V: RuleVisitor>(vis: &mut V, blk: ast::ExternBlock) {
    vis.visit_attr(blk.attrs().collect());

    for ex_item in blk
        .extern_item_list()
        .iter()
        .flat_map(|it| it.extern_items())
    {
        match ex_item {
            ast::ExternItem::Fn(f) => vis.visit_fn(f),
            ast::ExternItem::MacroCall(mc) => vis.visit_mac_call(mc),
            ast::ExternItem::Static(stat) => vis.visit_static(stat),
            ast::ExternItem::TypeAlias(ty) => vis.visit_type_alias(ty),
        }
    }
}

pub fn walk_extern_crate<V: RuleVisitor>(vis: &mut V, krate: ast::ExternCrate) {
    vis.visit_attr(krate.attrs().collect());

    if let Some(v) = krate.visibility() {
        vis.visit_vis(v);
    }
}

pub fn walk_fn<V: RuleVisitor>(vis: &mut V, func: ast::Fn) {
    vis.visit_comment(func.doc_comments().collect());
    vis.visit_attr(func.attrs().collect());
    if let Some(v) = func.visibility() {
        vis.visit_vis(v);
    }
    vis.visit_name(func.name().unwrap());

    for param in func
        .generic_param_list()
        .iter()
        .flat_map(|gp| gp.generic_params())
    {
        vis.visit_gen_param(param)
    }

    if let Some(ret) = func.ret_type() {
        println!("{:?}", ret);
        vis.visit_fn_return(ret);
    }

    if let Some(body) = func.body() {
        vis.visit_block(body);
    }
}

pub fn walk_impl<V: RuleVisitor>(vis: &mut V, impl_: ast::Impl) {}

pub fn walk_mac_call<V: RuleVisitor>(vis: &mut V, mac_call: ast::MacroCall) {}

pub fn walk_mac_def<V: RuleVisitor>(vis: &mut V, mac_def: ast::MacroDef) {}

pub fn walk_mac_rules<V: RuleVisitor>(vis: &mut V, mac_rules: ast::MacroRules) {}

pub fn walk_mod<V: RuleVisitor>(vis: &mut V, module: ast::Module) {}

pub fn walk_static<V: RuleVisitor>(vis: &mut V, static_: ast::Static) {}

pub fn walk_struct<V: RuleVisitor>(vis: &mut V, struct_: ast::Struct) {
    vis.visit_comment(struct_.doc_comments().collect());
    vis.visit_attr(struct_.attrs().collect());

    if let Some(v) = struct_.visibility() {
        vis.visit_vis(v);
    }

    vis.visit_name(struct_.name().unwrap());

    for param in struct_
        .generic_param_list()
        .iter()
        .flat_map(|gp| gp.generic_params())
    {
        vis.visit_gen_param(param)
    }

    if let Some(field) = struct_.field_list() {
        vis.visit_struct_fields(field);
    }
}

pub fn walk_struct_fields<V: RuleVisitor>(vis: &mut V, fields: ast::FieldList) {
    match fields {
        ast::FieldList::RecordFieldList(fields) => {
            for field in fields.fields() {
                vis.visit_comment(field.doc_comments().collect());
                vis.visit_attr(field.attrs().collect());

                if let Some(v) = field.visibility() {
                    vis.visit_vis(v);
                }
                vis.visit_name(field.name().unwrap());
                vis.visit_type(field.ty().unwrap());
            }
        }
        ast::FieldList::TupleFieldList(fields) => {
            for field in fields.fields() {
                vis.visit_comment(field.doc_comments().collect());
                vis.visit_attr(field.attrs().collect());

                if let Some(v) = field.visibility() {
                    vis.visit_vis(v);
                }
                vis.visit_type(field.ty().unwrap());
            }
        }
    }
}

pub fn walk_trait<V: RuleVisitor>(vis: &mut V, ty: ast::Trait) {}

pub fn walk_type_alias<V: RuleVisitor>(vis: &mut V, trait_: ast::TypeAlias) {}

pub fn walk_union<V: RuleVisitor>(vis: &mut V, union_: ast::Union) {}

pub fn walk_use<V: RuleVisitor>(vis: &mut V, file: ast::Use) {}

pub fn walk_attr<V: RuleVisitor>(vis: &mut V, attrs: Vec<ast::Attr>) {}

pub fn walk_vis<V: RuleVisitor>(vis: &mut V, visibility: ast::Visibility) {}

pub fn walk_type<V: RuleVisitor>(vis: &mut V, ty: ast::Type) {}

pub fn walk_name<V: RuleVisitor>(vis: &mut V, name: ast::Name) {}

pub fn walk_gen_param<V: RuleVisitor>(vis: &mut V, gen: ast::GenericParam) {}

pub fn walk_block_expr<V: RuleVisitor>(vis: &mut V, blk: ast::BlockExpr) {
    vis.visit_attr(blk.attrs().collect());

    for stmt in blk.statements() {
        vis.visit_stmt(stmt);
    }

    if let Some(expr) = blk.tail_expr() {
        vis.visit_expr(expr)
    }
}

pub fn walk_expr<V: RuleVisitor>(vis: &mut V, expr: ast::Expr) {
    vis.visit_attr(expr.attrs().collect());

    match expr {
        ast::Expr::ArrayExpr(arr) => {
            for e in arr.exprs() {
                vis.visit_expr(e);
            }
            if let Some(ex) = arr.expr() {
                vis.visit_expr(ex);
            }
        }
        ast::Expr::AwaitExpr(await_ex) => {
            if let Some(ex) = await_ex.expr() {
                vis.visit_expr(ex);
            }
        }
        ast::Expr::BinExpr(bin) => panic!("BinExpr"),
        ast::Expr::BlockExpr(blk_expr) => {
            vis.visit_block(blk_expr);
        }
        ast::Expr::BoxExpr(box_expr) => {
            if let Some(e) = box_expr.expr() {
                vis.visit_expr(e)
            }
        }
        ast::Expr::BreakExpr(brk_expr) => {
            if let Some(e) = brk_expr.expr() {
                vis.visit_expr(e)
            }
        }
        ast::Expr::CallExpr(call) => {
            if let Some(args) = call.arg_list() {
                vis.visit_arg_list(args);
            }
            if let Some(e) = call.expr() {
                vis.visit_expr(e)
            }
        }
        ast::Expr::CastExpr(cast) => {
            if let Some(expr) = cast.expr() {
                vis.visit_expr(expr);
            }
            if let Some(ty) = cast.ty() {
                vis.visit_type(ty);
            }
        }
        ast::Expr::ClosureExpr(closure) => {
            if let Some(params) = closure.param_list() {
                vis.visit_params(params);
            }
            if let Some(expr) = closure.body() {
                vis.visit_expr(expr);
            }
        }
        ast::Expr::ContinueExpr(_) => {}
        ast::Expr::EffectExpr(eff) => {
            if let Some(expr) = eff.block_expr() {
                vis.visit_block(expr);
            }
        }
        ast::Expr::FieldExpr(_) => {}
        ast::Expr::ForExpr(for_ex) => {
            if let Some(pat) = for_ex.pat() {
                vis.visit_pat(pat);
            }

            if let Some(e) = for_ex.iterable() {
                vis.visit_expr(e);
            }

            if let Some(blk) = for_ex.loop_body() {
                vis.visit_block(blk);
            }
        }
        ast::Expr::IfExpr(expr) => {
            if let Some(cond) = expr.condition() {
                if let Some(pat) = cond.pat() {
                    vis.visit_pat(pat);
                }

                if let Some(e) = cond.expr() {
                    vis.visit_expr(e);
                }
            }
        }
        ast::Expr::IndexExpr(idx) => {}
        ast::Expr::Literal(lit) => {
            println!("LIT {:?}", lit);
        }
        ast::Expr::LoopExpr(_) => {}
        ast::Expr::MacroCall(_) => {}
        ast::Expr::MatchExpr(_) => {}
        ast::Expr::MethodCallExpr(meth) => {}
        ast::Expr::ParenExpr(_) => {}
        ast::Expr::PathExpr(_) => {}
        ast::Expr::PrefixExpr(_) => {}
        ast::Expr::RangeExpr(_) => {}
        ast::Expr::RecordExpr(_) => {}
        ast::Expr::RefExpr(_) => {}
        ast::Expr::ReturnExpr(_) => {}
        ast::Expr::TryExpr(_) => {}
        ast::Expr::TupleExpr(_) => {}
        ast::Expr::WhileExpr(_) => {}
        ast::Expr::YieldExpr(_) => {}
    }
}

pub fn walk_pat<V: RuleVisitor>(vis: &mut V, pat: ast::Pat) {
    match pat {
        ast::Pat::IdentPat(ident) => {
            vis.visit_attr(ident.attrs().collect());
            vis.visit_name(ident.name().unwrap());

            if let Some(pat) = ident.pat() {
                vis.visit_pat(pat);
            }
        }
        ast::Pat::BoxPat(box_pat) => {
            if let Some(pat) = box_pat.pat() {
                vis.visit_pat(pat);
            }
        }
        ast::Pat::RestPat(_) => {}
        ast::Pat::LiteralPat(_) => {}
        ast::Pat::MacroPat(mac) => {
            if let Some(mac_call) = mac.macro_call() {
                vis.visit_mac_call(mac_call);
            }
        }
        ast::Pat::OrPat(or) => {
            for pat in or.pats() {
                vis.visit_pat(pat);
            }
        }
        ast::Pat::ParenPat(ppat) => {
            if let Some(pat) = ppat.pat() {
                vis.visit_pat(pat);
            }
        }
        ast::Pat::PathPat(path) => {
            if let Some(pat) = path.path() {
                vis.visit_path(pat);
            }
        }
        ast::Pat::WildcardPat(_) => {}
        ast::Pat::RangePat(_) => {}
        ast::Pat::RecordPat(record) => {}
        ast::Pat::RefPat(re) => {}
        ast::Pat::SlicePat(slice) => {}
        ast::Pat::TuplePat(tup) => {}
        ast::Pat::TupleStructPat(struc) => {}
        ast::Pat::ConstBlockPat(cons) => {}
    }
}

pub fn walk_stmt<V: RuleVisitor>(vis: &mut V, stmt: ast::Stmt) {
    match stmt {
        ast::Stmt::ExprStmt(expr) => {
            if let Some(expr) = expr.expr() {
                vis.visit_expr(expr);
            }
        }
        ast::Stmt::Item(item) => vis.visit_item(item),
        ast::Stmt::LetStmt(let_stmt) => {
            if let Some(expr) = let_stmt.pat() {
                vis.visit_pat(expr);
            }
            if let Some(expr) = let_stmt.ty() {
                vis.visit_type(expr);
            }
            if let Some(expr) = let_stmt.initializer() {
                vis.visit_expr(expr);
            }
        }
    }
}

pub fn walk_param<V: RuleVisitor>(vis: &mut V, ty: ast::ParamList) {}

pub fn walk_path<V: RuleVisitor>(vis: &mut V, ty: ast::Path) {}

pub fn walk_arm<V: RuleVisitor>(vis: &mut V, ty: ast::MatchArm) {}

pub fn walk_where_clause<V: RuleVisitor>(vis: &mut V, ty: ast::WhereClause) {}

pub fn walk_where_pred<V: RuleVisitor>(vis: &mut V, ty: ast::WherePred) {}

pub fn walk_assoc_ty<V: RuleVisitor>(vis: &mut V, ty: ast::AssocTypeArg) {}

pub fn walk_args<V: RuleVisitor>(vis: &mut V, ty: ast::ArgList) {}

pub fn walk_fn_ret<V: RuleVisitor>(vis: &mut V, ty: ast::RetType) {}

#[test]
fn visit_test() {
    struct TestVis;
    impl RuleVisitor for TestVis {}

    let text = r#"
pub fn main() -> Result<(), Error> {
    let x = "hello world";
    if x.starts_with('h') {
        foo();
    } else {
        loop {
            x += 1;
        }
    }
}
"#;

    let mut vis = TestVis;

    let source = SourceFile::parse(&text)
        .ok()
        .expect("TODO: SyntaxError's from SourceFile::parse");

    vis.visit_source_file(source);
}
