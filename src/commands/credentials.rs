use crate::utils;
use comfy_table::Table;
use console::style;

pub fn run() -> anyhow::Result<()> {
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
    let mut table = Table::new();
    table
        .set_header(vec!["name", "value"])
        .add_row(vec!["id", &config.id.to_string()])
        .add_row(vec!["name", &config.name])
        .add_row(vec!["client", &config.client_id])
        .add_row(vec!["created", &config.created_at]);
    println!(
        "{}\n\n{}",
        style("Dagpi Cli Credentials").bright().bold(),
        table
    );
    Ok(())
}
