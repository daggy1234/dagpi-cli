use crate::models::AppParent;
use crate::utils;
use anyhow::Result;
use comfy_table::Table;
use console::style;

pub fn run() -> Result<()> {
    let config = match utils::parse_config() {
        Ok(c) => c,
        Err(e) => {
            anyhow::bail!(format!(
                "{} {}\nPlease try using `dagpi login` if you haven't.",
                style("Unable to read config due to an error: ").red(),
                style(e.to_string()).red()
            ))
        }
    };
    let app = match utils::get::<AppParent>(&*utils::build_url("app".parse()?, config.token)) {
        Ok(t) => t,
        Err(e) => {
            println!(
                "{}",
                style(format!("Error Getting App: {}", e.to_string())).red()
            );
            println!();
            return Ok(());
        }
    };
    let mut table = Table::new();
    table
        .set_header(vec!["name", "value"])
        .add_row(vec!["id", &app.app.uuid])
        .add_row(vec!["name", &app.app.name])
        .add_row(vec!["url", &app.app.url])
        .add_row(vec!["approved", &app.app.approved.to_string()])
        .add_row(vec!["premium", &app.app.premium.to_string()])
        .add_row(vec!["user_id", &app.app.user_id])
        .add_row(vec!["created", &app.app.created_at])
        .add_row(vec!["updated", &app.app.updated_at]);
    println!("{}", table);

    Ok(())
}
