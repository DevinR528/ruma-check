use std::{
    convert::TryInto,
    env, fs,
    path::{Path, PathBuf},
};

use project_model::PackageData;
use thing::thing_test;

mod project;
mod rules;

use project::CargoInfo;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    match args.as_slice() {
        [_] => {
            let loc = env::current_dir().expect("No current directory found.");
            let root = CargoInfo::build_crate_root(&loc).expect("Failed to parse Cargo.toml");
            if let Err(errors) = check_workspace(root) {
                for e in errors {
                    eprintln!("{}", e);
                }
            }
        }
        [] => panic!(),
        _ => panic!(),
    }
}

fn check_workspace(info: CargoInfo) -> Result<(), Vec<rules::ValidationError>> {
    let mut errors = vec![];
    for pack in info.work.packages() {
        let p = &info.work[pack];
        if p.is_member {
            if let Err(e) = check_files(p) {
                errors.extend(e);
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}

fn check_files(package: &PackageData) -> Result<(), Vec<rules::ValidationError>> {
    // Infallible
    let mut path: PathBuf = package.manifest.clone().try_into().unwrap();
    path.pop();
    path.push("src");

    let mut errors = vec![];
    for file in walk_dirs(&path) {
        let text = fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("Failed to open file at {:?}", file));
        if let Err(e) = rules::validate_source(&text) {
            errors.extend(e);
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    Ok(())
}

fn walk_dirs(dir: &Path) -> impl Iterator<Item = PathBuf> {
    Walker {
        dir_stack: vec![dir.to_owned()],
        files: vec![],
    }
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
