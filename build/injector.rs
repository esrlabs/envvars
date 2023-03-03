#[path = "../src/checksum.rs"]
mod checksum;
use super::paths;
use checksum::checksum;
use std::{
    fs::{remove_file, write, File},
    io::{Error, Read},
};
use uuid::Uuid;

pub fn inject() -> Result<(), Error> {
    let out_dir = paths::cargo_output_dir()?;
    let dest = out_dir.join("assets.rs");
    if dest.exists() {
        remove_file(&dest)?;
    }
    let mut extractor = File::open(paths::extractor_executable()?)?;
    let mut buffer = Vec::new();
    extractor.read_to_end(&mut buffer)?;
    write(
        &dest,
        format!(
            "
static BIN: &[u8] = &{buffer:?};
static CHECKSUM: &str = \"{}\";
static FILENAME: &str = \"{}\";
        ",
            checksum(&paths::extractor_executable()?)?,
            Uuid::new_v4(),
        ),
    )?;
    Ok(())
}
