use super::{cargo_log, paths};
use fs_extra::dir;
use std::{
    env,
    fs::rename,
    io::{Error, ErrorKind},
    path::PathBuf,
    process::Command,
    str::from_utf8,
};

fn skip() -> bool {
    paths::is_predefined_location_used() && paths::extractor_executable().is_ok()
}

fn report(dest: &PathBuf) {
    cargo_log!("Build {dest:?} is done");
    cargo_log!("Extractor folder: {}", paths::ls(dest));
    cargo_log!(
        "Extractor target folder: {}",
        paths::ls(&dest.join("target"))
    );
    cargo_log!(
        "Extractor release folder: {}",
        paths::ls(&dest.join("target").join("release"))
    );
    cargo_log!(
        "Extractor debug folder: {}",
        paths::ls(&dest.join("target").join("debug"))
    );
    if let Ok(output) = paths::cargo_output_dir() {
        cargo_log!("Cargo OUTPUT folder {output:?}: {}", paths::ls(&output));
    }
    cargo_log!(
        "CARGO_BUILD_TARGET_DIR: {:?}",
        env::var_os("CARGO_BUILD_TARGET_DIR")
    );
    cargo_log!("CARGO_TARGET_DIR: {:?}", env::var_os("CARGO_TARGET_DIR"));
}

pub fn copy_sources() -> Result<(), Error> {
    if skip() {
        cargo_log!("Copying of source is skipped. Sources already exist");
        return Ok(());
    }
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
    if skip() {
        cargo_log!("Building is skipped. Extractor already exist");
        report(&dest);
        return Ok(());
    }
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
        report(&dest);
        Ok(())
    }
}
