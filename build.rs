use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=resources/*");

    // desktop
    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let paths_to_copy = vec!["resources/"];
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    // web
    let out_dir = "./web";
    copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
