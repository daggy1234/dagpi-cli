use crate::models::{ClientToken, TomlDump};
use anyhow;
use console::style;
use dirs;
use std::io::Read;
use std::path::PathBuf;

fn gen_path() -> anyhow::Result<PathBuf> {
    let mut file_path = match dirs::config_dir() {
        Some(p) => p,
        None => {
            println!(
                "{}",
                style("Cannot find valid config dir for your OS. Cli will not work").red()
            );
            anyhow::bail!("Cannot find dir")
        }
    };
    file_path.push("dagpi");
    file_path.push("dagpi.toml");
    Ok(file_path)
}

pub fn parse_config() -> anyhow::Result<ClientToken> {
    let path = gen_path()?;
    let mut read = std::fs::File::open(path)?;
    let mut buffer = String::new();
    read.read_to_string(&mut buffer)?;
    let model = toml::from_str::<TomlDump>(&*buffer)?;
    Ok(model.cli_token)
}
