use linking::symlink_by_config;

mod config;
mod home_dir;
mod linking;

static CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
static CRATE_NAME: &'static str = env!("CARGO_PKG_NAME");
static CRATE_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
static HELP_TEXT: &'static str = "Usage:\n  Run 'lnkr' in a directory containing a 'lnkr.yaml'";

fn help_text() -> String {
    format!(
        "{} {} - {}\n\n{}",
        CRATE_NAME, CRATE_VERSION, CRATE_DESCRIPTION, HELP_TEXT
    )
}

fn main() -> eyre::Result<()> {
    if let Some(_) = std::env::args().position(|arg| arg == "--help" || arg == "-h") {
        println!("{}", help_text());
        return Ok(());
    }

    let mut config_path = std::env::current_dir()?;
    config_path.push(config::CONFIG_FILE_NAME);

    let used_config = config::read_config(config_path)?;
    let results = symlink_by_config(used_config)?;

    for result in results {
        println!("{}", result.to_string());
    }

    Ok(())
}
