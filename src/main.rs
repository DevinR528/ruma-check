use std::{
    convert::TryInto,
    env, fs,
    path::{Path, PathBuf},
};

use project_model::PackageData;

mod error;
mod macro_exp;
mod project;
mod rules;

use error::Emitter;
use project::CargoInfo;

pub type EzError = Box<dyn std::error::Error>;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let mut emitter = Emitter::default();
    match args.as_slice() {
        [_] => {
            let loc = env::current_dir().expect("No current directory found.");
            let root =
                CargoInfo::build_crate_root(&loc).expect("Failed to parse Cargo.toml");
            check_workspace(root, &mut emitter).unwrap_or_else(|e| {
                eprintln!("{}", e);
                std::process::exit(1);
            });
        }
        [] => panic!(),
        _ => panic!(),
    }

    if emitter.found_errors() {
        emitter.emit().unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
        std::process::exit(1);
    }
}

fn check_workspace(info: CargoInfo, emitter: &mut Emitter) -> Result<(), EzError> {
    for pack in info.work.packages() {
        let p = &info.work[pack];
        if p.is_member {
            check_files(p, emitter)?;
        }
    }

    Ok(())
}

fn check_files(package: &PackageData, emitter: &mut Emitter) -> Result<(), EzError> {
    // Infallible
    let mut path: PathBuf = package.manifest.clone().try_into()?;
    path.pop();
    path.push("src");

    for file in walk_dirs(&path) {
        let text = fs::read_to_string(&file)
            .map_err(|_| format!("Failed to open file at {:?}", file))?;

        // Here is where the magic happens.
        // We validate all files found for this crate!
        rules::validate_source(&file, &text, emitter)?;
    }

    Ok(())
}

fn walk_dirs(dir: &Path) -> impl Iterator<Item = PathBuf> {
    Walker { dir_stack: vec![dir.to_owned()], files: vec![] }
}

struct Walker {
    dir_stack: Vec<PathBuf>,
    files: Vec<PathBuf>,
}

impl Iterator for Walker {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir_stack.pop() {
            Some(dir) => {
                if dir.is_dir() {
                    for entry in fs::read_dir(dir).ok()? {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.is_dir() {
                            self.dir_stack.push(path);
                        } else {
                            self.files.push(path);
                        }
                    }
                    self.files.pop()
                } else {
                    None
                }
            }
            None => self.files.pop(),
        }
    }
}
