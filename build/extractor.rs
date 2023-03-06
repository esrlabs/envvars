use super::paths;
use fs_extra::dir;
use std::{
    env,
    fs::rename,
    io::{Error, ErrorKind},
    process::Command,
    str::from_utf8,
};

pub fn copy_sources() -> Result<(), Error> {
    let src = paths::extractor_src_dir()?;
    let dest = paths::extractor_dest_dir()?;
    let extractor = dest.join("extractor");
    dir::copy(&src, &dest, &dir::CopyOptions::new().overwrite(true)).map_err(|_| {
        Error::new(
            ErrorKind::Other,
            format!("Fail to copy sources from {src:?} to {dest:?}"),
        )
    })?;
    rename(
        extractor.join("Cargo.toml.hidden"),
        extractor.join("Cargo.toml"),
    )?;
    Ok(())
}

pub fn build() -> Result<(), Error> {
    let dest = paths::extractor_dest_dir()?.join("extractor");
    let output = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--target-dir",
            &dest.join("target").to_string_lossy(),
        ])
        .current_dir(&dest)
        .output()?;
    if !output.status.success() {
        Err(Error::new(
            ErrorKind::Other,
            format!(
                "Fail to build: {}",
                from_utf8(&output.stderr)
                    .expect("Fail to decode output of \"cargo build\" command")
            ),
        ))
    } else {
        println!("Build {dest:?} is done");
        println!("Extractor folder: {}", paths::ls(&dest));
        println!(
            "Extractor target folder: {}",
            paths::ls(&dest.join("target"))
        );
        println!(
            "Extractor release folder: {}",
            paths::ls(&dest.join("target").join("release"))
        );
        println!(
            "Extractor debug folder: {}",
            paths::ls(&dest.join("target").join("debug"))
        );
        if let Ok(output) = paths::cargo_output_dir() {
            println!("Cargo OUTPUT folder {output:?}: {}", paths::ls(&output));
        }
        println!(
            "CARGO_BUILD_TARGET_DIR: {:?}",
            env::var_os("CARGO_BUILD_TARGET_DIR")
        );
        println!("CARGO_TARGET_DIR: {:?}", env::var_os("CARGO_TARGET_DIR"));
        Ok(())
    }
}
