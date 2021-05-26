use crate::models::AppParent;
use crate::utils;
use comfy_table::Table;
use console::{style, Emoji};

fn return_status(url: &str, b: bool) {
    if b {
        println!(
            "{} is {}",
            style(url).bright().bold(),
            Emoji("🟢", &*format!("{}", style("online").green()))
        );
    } else {
        println!(
            "{} is {}",
            style(url).bright().bold(),
            Emoji("🔴", &*format!("{}", style("offline").red()))
        );
    }
}

pub fn run() -> anyhow::Result<()> {
    println!("-------------------------------------");
    let ping_main = utils::ping("https://api.dagpi.xyz")?;
    return_status("https://api.dagpi.xyz", ping_main);
    println!("-------------------------------------");
    let ping_main = utils::ping("https://central.dagpi.xyz")?;
    return_status("https://central.dagpi.xyz", ping_main);
    println!("-------------------------------------");
    let ping_main = utils::ping("https://dagpi.xyz")?;
    return_status("https://dagpi.xyz", ping_main);
    println!("-------------------------------------");
    Ok(())
}
