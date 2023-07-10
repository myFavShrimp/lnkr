use std::path::{Path, PathBuf};

use directories::BaseDirs;

#[derive(thiserror::Error, Debug)]
pub enum HomeDirError {
    #[error("No home dir was found the system")]
    NotFound,
}

pub fn expand_tilde<P>(path: P) -> Result<PathBuf, HomeDirError>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let resulting_path = if let Ok(path_from_home) = path.strip_prefix("~") {
        BaseDirs::new()
            .ok_or(HomeDirError::NotFound)?
            .home_dir()
            .join(path_from_home)
    } else {
        PathBuf::from(path)
    };

    Ok(resulting_path)
}
