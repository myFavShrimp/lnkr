use std::{
    fs::{canonicalize, create_dir_all, read_link, remove_file},
    path::PathBuf,
};

use crate::{config::Config, home_dir::expand_tilde};

pub struct LinkToCreate {
    from: std::path::PathBuf,
    to: std::path::PathBuf,
    force: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum LinkError {
    #[error("An item already exists at the link location.")]
    AlreadyExists,
    #[error("Could not create the destination directory - {0}")]
    DestinationCreationError(std::io::Error),
    #[error("{0}")]
    IoError(std::io::Error),
    #[error("The 'from' path has an error - {0}")]
    FromPathError(std::io::Error),
}

pub enum LinkingSuccessState {
    Linked,
    AlreadyLinked,
}

impl LinkingSuccessState {
    pub fn message(&self) -> &'static str {
        match self {
            LinkingSuccessState::Linked => "Successfully linked.",
            LinkingSuccessState::AlreadyLinked => "Already linked.",
        }
    }
}

pub enum LinkResult {
    Success {
        item: LinkToCreate,
        state: LinkingSuccessState,
    },
    Failure {
        item: LinkToCreate,
        error: LinkError,
    },
}

impl ToString for LinkResult {
    fn to_string(&self) -> String {
        match self {
            LinkResult::Success { item, state } => {
                format!(
                    "✅ {} -> {}: {}",
                    item.from.display(),
                    item.to.display(),
                    state.message()
                )
            }
            LinkResult::Failure { item, error } => {
                format!(
                    "❌ {} -> {}: {}",
                    item.from.display(),
                    item.to.display(),
                    error
                )
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LinkPreparationError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    HomeDirError(#[from] crate::home_dir::HomeDirError),
}

pub fn symlink_by_config(config: Config) -> Result<Vec<LinkResult>, LinkPreparationError> {
    let links_to_create = retrieve_links_to_create(config)?;

    Ok(links_to_create.into_iter().map(symlink).collect())
}

fn symlink(item: LinkToCreate) -> LinkResult {
    let from_path = match canonicalize(item.from.clone()) {
        Ok(path) => path,
        Err(e) => {
            return LinkResult::Failure {
                item,
                error: LinkError::FromPathError(e),
            }
        }
    };
    if item.to.exists() {
        if item.to.is_symlink() && read_link(&item.to).unwrap_or(PathBuf::new()) == from_path {
            return LinkResult::Success {
                item,
                state: LinkingSuccessState::AlreadyLinked,
            };
        }
        if !item.force {
            return LinkResult::Failure {
                item,
                error: LinkError::AlreadyExists,
            };
        };
    }
    if let Some(parent_path) = item.to.parent() {
        if !parent_path.exists() {
            if let Err(e) = create_dir_all(parent_path) {
                return LinkResult::Failure {
                    item,
                    error: LinkError::DestinationCreationError(e),
                };
            }
        }
    }

    if item.force && item.to.exists() {
        if let Err(e) = remove_file(&item.to) {
            return LinkResult::Failure {
                item,
                error: LinkError::IoError(e),
            };
        }
    }

    match std::os::unix::fs::symlink(&from_path, &item.to) {
        Ok(_) => LinkResult::Success {
            item,
            state: LinkingSuccessState::Linked,
        },
        Err(e) => LinkResult::Failure {
            item,
            error: LinkError::IoError(e),
        },
    }
}

fn retrieve_links_to_create(config: Config) -> Result<Vec<LinkToCreate>, LinkPreparationError> {
    let current_os = String::from(std::env::consts::OS);

    let mut links_to_create = Vec::new();
    for link_group in config.links {
        if link_group
            .os
            .clone()
            .unwrap_or(Vec::new())
            .contains(&current_os)
        {
            let links: Vec<LinkToCreate> = link_group.try_into()?;
            links_to_create.extend(links);
        }
    }

    Ok(links_to_create)
}

impl From<(std::path::PathBuf, crate::config::Link)> for LinkToCreate {
    fn from(value: (std::path::PathBuf, crate::config::Link)) -> Self {
        let mut to = value.0;

        to.push(value.1.name);

        Self {
            from: value.1.path,
            to,
            force: value.1.force,
        }
    }
}

impl TryFrom<crate::config::LinkGroup> for Vec<LinkToCreate> {
    type Error = LinkPreparationError;

    fn try_from(value: crate::config::LinkGroup) -> Result<Self, Self::Error> {
        let mut result = Vec::new();
        let destination_path = expand_tilde(value.destination)?;
        for mut item in value.items {
            item.path = expand_tilde(item.path)?;
            result.push((destination_path.clone(), item).into())
        }

        Ok(result)
    }
}
