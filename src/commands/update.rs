#[cfg(feature = "auto_updates")]
pub fn run() -> anyhow::Result<()> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("daggy1234")
        .repo_name("dagpi-cli")
        .build()?
        .fetch()?;
    println!("found releases:");
    println!("{:#?}\n", releases);
    // let status = self_update::backends::github::Update::configure()
    //     .repo_owner("daggy1234")
    //     .repo_name("dagpi-cli")
    //     .bin_name("dagpi")
    //     .show_download_progress(true)
    //     .current_version(cargo_crate_version!())
    //     .build()?
    //     .update()?;
    // println!("Update status: `{}`!", status.version());
    println!("Yes updates will be automatic uwu");
    Ok(())
}

#[cfg(not(feature = "auto_updates"))]
pub fn run() -> anyhow::Result<()> {
    println!("Installed via package manager. Please update using it");
    Ok(())
}

// #[cfg(not(feature = "auto_updates"))]
// pub fn run() -> anyhow::Result<()> {
//     println!("Installed via package manager. Please update using it");
//     Ok(())
// }
