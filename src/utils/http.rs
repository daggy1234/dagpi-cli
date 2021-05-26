use crate::models;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::time::Instant;
use ureq;

pub fn get<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T, ureq::Error> {
    let style_spinner = ProgressStyle::default_spinner().template("{prefix} {spinner}   {msg}");
    let spinner = ProgressBar::new_spinner().with_style(style_spinner);
    spinner.set_message("Waiting for api request");
    spinner.enable_steady_tick(20);
    let now = Instant::now();
    let body = ureq::get(url).call()?;
    let status = body.status();
    let json: T = body.into_json()?;
    let diff = now.duration_since(now);
    let color: String;
    if status >= 499 {
        color = format!("{}", style(status.to_string()).red());
        spinner.set_prefix(format!("{}", models::ERROR));
    } else if status >= 399 {
        color = format!("{}", style(status.to_string()).yellow());
        spinner.set_prefix(format!("{}", models::ERROR));
    } else {
        color = format!("{}", style(status.to_string()).green());
        spinner.set_prefix(format!("{}", models::SUCCESS));
    }

    spinner.finish();
    println!(
        "{} {} {} {}ms",
        style("GET").bold().bright(),
        url.replace("https://central.dagpi.xyz", ""),
        color,
        diff.as_micros()
    );
    Ok(json)
}
pub fn ping(url: &str) -> Result<bool, ureq::Error> {
    let style_spinner = ProgressStyle::default_spinner().template("{prefix} {spinner}   {msg}");
    let spinner = ProgressBar::new_spinner().with_style(style_spinner);
    spinner.set_message("Waiting for api request");
    spinner.enable_steady_tick(20);
    let now = Instant::now();
    let body = ureq::get(url).call()?;
    let status = body.status();
    let diff = now.duration_since(now);
    let color: String;
    let success: bool;
    if status >= 399 {
        color = format!("{}", style(status.to_string()).red());
        success = false;
        spinner.set_prefix(format!("{}", models::ERROR));
    } else {
        color = format!("{}", style(status.to_string()).green());
        success = true;
        spinner.set_prefix(format!("{}", models::SUCCESS));
    }
    spinner.finish();
    println!(
        "{} {} {} {}ms",
        style("GET").bold().bright(),
        url.replace("https://central.dagpi.xyz", ""),
        color,
        diff.as_micros()
    );
    Ok(success)
}
