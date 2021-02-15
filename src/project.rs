use std::{
    convert::TryInto,
    path::{Path, PathBuf},
};

use paths::{AbsPath, AbsPathBuf};
use project_model::{CargoConfig, CargoWorkspace, ProjectManifest};

#[derive(Clone, Debug)]
pub struct CargoInfo {
    pub work: CargoWorkspace,
}

impl CargoInfo {
    pub fn build_crate_root<P: AsRef<Path>>(p: P) -> Result<Self, String> {
        let mut path = p.as_ref().to_owned();
        if !path.ends_with("Cargo.toml") {
            path.push("Cargo.toml");
        }
        let path: AbsPathBuf = path
            .try_into()
            .map_err(|_| format!("Failed to find {:?}", p.as_ref()))?;

        let config = CargoConfig::default();

        Ok(Self {
            work: CargoWorkspace::from_cargo_metadata(&path, &config, &|_| {})
                .map_err(|e| e.to_string())?,
        })
    }
}
