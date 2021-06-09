use std::path::PathBuf;

use syntax::SyntaxNode;

use crate::{error::Emitter, rules::NodeRule};

#[derive(Debug, Default)]
pub struct BanMod;

impl BanMod {
    pub fn new() -> Box<Self> { Box::new(Self::default()) }
}
impl NodeRule for BanMod {
    fn name(&self) -> &str { "Module files (mod.rs) are banned." }
    fn apply_rule(&mut self, _: &SyntaxNode) {}

    fn match_node(&self, _: &SyntaxNode) -> bool { false }

    fn validate(&self, path: &str, emitter: &mut Emitter) -> Result<(), crate::EzError> {
        if path.contains("mod.rs") {
            let mut p = PathBuf::from(path);
            p.pop();
            let folder = p.clone();

            p.set_extension("rs");
            let file = p;

            emitter.simple_sugg(
                self.name(),
                &format!(
                    "create a `{}` file and `{}` folder and remove `{}`",
                    file.display(),
                    folder.display(),
                    path
                ),
                path,
            );
        }
        Ok(())
    }
}
