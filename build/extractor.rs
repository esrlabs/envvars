use super::paths;
use fs_extra::dir;
use std::{
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
        .args(["build", "--release"])
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
        Ok(())
    }
}
