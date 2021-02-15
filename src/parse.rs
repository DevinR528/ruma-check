use std::path::Path;

use std::fs;

use syntax::SourceFile;

pub struct TokenTree {}

pub fn to_tokens(p: &Path) -> Result<TokenTree, String> {
    let text = fs::read_to_string(p).map_err(|e| e.to_string())?;

    let parse = SourceFile::parse(&text);
    println!("{:#?}", parse.syntax_node());
    Ok(TokenTree {})
}
