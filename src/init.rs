use std::{fs, path::Path};

use crate::{messaging::error_log, patches::Patch};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{message}")]
pub struct InitError {
    message: String,
}
impl InitError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
impl From<std::io::Error> for InitError {
    fn from(e: std::io::Error) -> Self {
        Self::new(e.to_string())
    }
}

pub fn init() -> Result<(), InitError> {
    let patches_dir = Path::new("patches");
    if !patches_dir.is_dir() {
        error_log!("jitpatcher: patches directory not found");
        return Ok(());
    }
    let patch_files = fs::read_dir("patches")?.collect::<Result<Vec<_>, _>>()?;
    let patch_files = patch_files
        .iter()
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "toml"));

    for patch_file in patch_files {
        let patch = fs::read_to_string(patch_file.path())?;
        let patch: Patch = match toml::from_str(&patch) {
            Ok(patch) => patch,
            Err(err) => {
                error_log!(
                    "jitpatcher: error parsing patch file: {}\n{}",
                    patch_file.file_name().into_string().unwrap(),
                    err
                );
                continue;
            }
        };
    }
    Ok(())
}
