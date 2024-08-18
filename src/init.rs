use std::{fs, path::Path};

use crate::{
    apply_patch::try_apply_patch,
    messaging::{error_log, show_message_box},
    patches::Patch,
};
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

pub unsafe fn init() -> Result<(), InitError> {
    let patches_dir = Path::new("patches");
    if !patches_dir.is_dir() {
        error_log!("jitpatcher: patches directory not found");
        return Ok(());
    }
    let mut patch_files = fs::read_dir("patches")?.collect::<Result<Vec<_>, _>>()?;
    patch_files.sort_by_key(|it| it.file_name());
    let patch_files = patch_files
        .iter()
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "toml"));

    let mut has_errors: bool = false;

    for patch_file in patch_files {
        let filename = patch_file.file_name().into_string().unwrap();
        let patch = fs::read_to_string(patch_file.path())?;
        let patch: Patch = match toml::from_str(&patch) {
            Ok(patch) => patch,
            Err(err) => {
                error_log!(
                    "jitpatcher: error parsing patch file: {}\n{}",
                    filename,
                    err
                );
                has_errors = true;
                continue;
            }
        };

        if !patch.enabled {
            continue;
        }

        let result = try_apply_patch(&patch);
        if let Err(err) = result {
            error_log!(
                "jitpatcher: error applying patch: {} ({})\n{}",
                filename,
                patch.name,
                err.get_error_text(&patch)
            );
            has_errors = true;
        }
    }

    if has_errors {
        unsafe {
            show_message_box(
                "jitpatcher error",
                "Couldn't apply all patches; see console for details",
            )
        }
    }
    Ok(())
}
