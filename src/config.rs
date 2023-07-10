use std::fs::read_to_string;

use serde::Deserialize;

pub static CONFIG_FILE_NAME: &str = "lnkr.yaml";

#[derive(Deserialize)]
pub struct Config {
    pub links: Vec<LinkGroup>,
}

#[derive(Deserialize)]
pub struct LinkGroup {
    pub preset: Option<Vec<String>>,
    pub os: Option<Vec<String>>,
    pub destination: std::path::PathBuf,
    pub items: Vec<Link>,
}

#[derive(Deserialize)]
pub struct Link {
    pub name: String,
    pub path: std::path::PathBuf,
    #[serde(default)]
    pub force: bool,
}

pub fn read_config(config_path: std::path::PathBuf) -> eyre::Result<Config> {
    let config = serde_yaml::from_str(&read_to_string(config_path)?)?;
    Ok(config)
}
