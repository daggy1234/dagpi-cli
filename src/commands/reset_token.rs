use crate::models::Token;
use crate::utils::{self, confirm};
use console::style;

pub fn run() -> anyhow::Result<()> {
    let out = confirm("Are you sure you want dagpi to reset your api key? Existing key will be deactivated and will stop working.")?;
    if !out {
        println!("Does not want to reset token. Aborting...");
        return Ok(());
    }
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
    let okay = match utils::get::<Token>(&*utils::build_url("reset_token".parse()?, config.token)) {
        Ok(out) => out,
        Err(e) => {
            println!(
                "{}",
                style(format!("Error Resetting Token: {}", e.to_string())).red()
            );
            println!();
            return Ok(());
        }
    };
    println!(
        "{}: Token was successfully reset for user_id {} . Visit the dashboard or use {} to view token",
        style("success").green(),
        okay.user_id,
        style("dagpi token").bold().bright()
    );
    Ok(())
}
