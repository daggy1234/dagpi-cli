use clap::{App, AppSettings, SubCommand};
mod commands;
mod models;
mod utils;

extern crate serde;

use anyhow::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let version: String = "0.2.0".to_string();
    let app = App::new("dagpi-cli")
        .version(&*version)
        .author("Daggy1234 <daggy@daggy.tech>")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .about("A cli for Managing you dagpi app. Still very beta")
        .setting(AppSettings::VersionlessSubcommands)
        .bin_name("dagpi")
        .subcommand(SubCommand::with_name("login").about("login to dagpi"))
        .subcommand(
            SubCommand::with_name("credentials")
                .alias("c")
                .about("view configured dagpi credentials"),
        );
    let matches = app.get_matches();
    if matches.subcommand_matches("login").is_some() {
        commands::login::run()?;
    } else if matches.subcommand_matches("credentials").is_some() {
        commands::credentials::run()?;
    }
    Ok(())
}
