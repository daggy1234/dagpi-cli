use crate::utils;
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
            r#"{} is {}"#,
            style(url).bright().bold(),
            Emoji("🔴", &*format!("{}", style("offline").red()))
        );
    }
}

pub fn run() -> anyhow::Result<()> {
    println!("-------------------------------------");

    return_status(
        "https://api.dagpi.xyz",
        utils::ping("https://api.dagpi.xyz")?,
    );
    println!("-------------------------------------");
    return_status(
        "https://central.dagpi.xyz",
        utils::ping("https://central.dagpi.xyz")?,
    );
    println!("-------------------------------------");
    return_status("https://dagpi.xyz", utils::ping("https://dagpi.xyz")?);
    println!("-------------------------------------");
    Ok(())
}
