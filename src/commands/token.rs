use crate::models::TokenParent;
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
    let token = match utils::get::<TokenParent>(&*utils::build_url("token".parse()?, config.token))
    {
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
        .add_row(vec!["user_id", &token.token.user_id])
        .add_row(vec!["ratelimit", &token.token.ratelimit.to_string()])
        .add_row(vec!["key", &token.token.api_key])
        .add_row(vec!["created", &token.token.created_at])
        .add_row(vec!["updated", &token.token.updated_at]);
    println!("{}", table);

    Ok(())
}
