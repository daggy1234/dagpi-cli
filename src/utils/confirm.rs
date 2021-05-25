use anyhow::Result;
use console::style;
use std::io::stdin;

pub fn confirm(prompt_string: &str) -> Result<bool> {
    println!("{} {}:", prompt_string, style("[y/n]").green());
    let mut response = String::new();
    stdin().read_line(&mut response)?;
    response = response.split_whitespace().collect(); // remove whitespace
    response.make_ascii_lowercase(); // ensure response is all lowercase
    response.truncate(1);
    match response.as_ref() {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => anyhow::bail!(
            "{}",
            style("Response must either be \"y\" for yes or \"n\" for no").red()
        ),
    }
}
