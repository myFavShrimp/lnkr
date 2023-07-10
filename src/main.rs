use linking::symlink_by_config;

mod config;
mod home_dir;
mod linking;

fn main() -> eyre::Result<()> {
    let mut config_path = std::env::current_dir()?;
    config_path.push(config::CONFIG_FILE_NAME);

    let used_config = config::read_config(config_path)?;
    let results = symlink_by_config(used_config)?;

    for result in results {
        println!("{}", result.to_string());
    }

    Ok(())
}
