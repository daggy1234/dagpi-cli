use clap::{App, AppSettings, SubCommand};
mod commands;
mod models;
mod utils;

extern crate serde;

use anyhow::Result;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let version: String = "0.3.1".to_string();
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
            SubCommand::with_name("app").about("view your dagpi application and it's status"),
        )
        .subcommand(
            SubCommand::with_name("status").about("view information on all of dagpi's apis"),
        )
        .subcommand(SubCommand::with_name("tokens").about("view your main dagpi api key"))
        .subcommand(SubCommand::with_name("update").about("Try to have the CLI update itself"))
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
    } else if matches.subcommand_matches("app").is_some() {
        commands::app::run()?;
    } else if matches.subcommand_matches("status").is_some() {
        commands::status::run()?;
    } else if matches.subcommand_matches("tokens").is_some() {
        commands::token::run()?;
    } else if matches.subcommand_matches("update").is_some() {
        commands::update::run()?
    }
    Ok(())
}
