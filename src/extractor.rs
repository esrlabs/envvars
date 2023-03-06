use crate::{assets, checksum::checksum, decoder, Error};
use std::{
    collections::HashMap,
    env::temp_dir,
    fs::{remove_file, File, OpenOptions},
    io,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output},
    str::from_utf8,
};

#[cfg(not(windows))]
use std::os::unix::fs::OpenOptionsExt;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

#[cfg(windows)]
fn get_extractor_path() -> PathBuf {
    temp_dir().join(Path::new(&format!("{}.exe", assets::filename())))
}

#[cfg(not(windows))]
fn get_extractor_path() -> PathBuf {
    temp_dir().join(Path::new(assets::filename()))
}

pub struct Extractor {
    location: PathBuf,
    /// Field is used only for testing to confirm status of hash checking
    pub(crate) invalid_hash: bool,
}

impl Extractor {
    pub fn new() -> Self {
        Extractor {
            location: get_extractor_path(),
            invalid_hash: false,
        }
    }

    #[cfg(not(windows))]
    fn create_file(&self) -> Result<File, io::Error> {
        OpenOptions::new()
            .mode(0o777)
            .read(true)
            .write(true)
            .create(true)
            .open(&self.location)
    }
    #[cfg(windows)]
    fn create_file(&self) -> Result<File, io::Error> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.location)
    }

    fn delivery(&mut self) -> Result<(), io::Error> {
        if self.location.exists() {
            log::warn!(
                "Extractor {:?} already exists. Checking checksum.",
                self.location
            );
            if !match checksum(&self.location) {
                Ok(checksum) => checksum == assets::checksum(),
                Err(err) => {
                    log::warn!(
                        "Fail to get checksum of extractor {:?}: {err}",
                        self.location
                    );
                    self.invalid_hash = true;
                    false
                }
            } {
                remove_file(&self.location)?;
            } else {
                return Ok(());
            }
        }
        let mut file = self.create_file()?;
        file.write_all(assets::bin())?;
        file.flush()?;
        log::debug!("File is written in: {:?}", self.location);
        Ok(())
    }

    #[cfg(not(windows))]
    fn output(&self, shell: Option<&PathBuf>, args: &[String]) -> Result<Output, io::Error> {
        if let Some(shell) = shell {
            Command::new(shell)
                .args(args.iter())
                .arg(&self.location)
                .output()
        } else {
            Command::new(&self.location).output()
        }
    }

    #[cfg(windows)]
    fn output(&self, shell: Option<&PathBuf>, args: &[String]) -> Result<Output, io::Error> {
        if let Some(shell) = shell {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            Command::new(shell)
                .args(args.iter())
                .arg(
                    &self
                        .location
                        .to_string_lossy()
                        .to_string()
                        .replace('\\', "\\\\"),
                )
                .creation_flags(CREATE_NO_WINDOW)
                .output()
        } else {
            Command::new(&self.location).output()
        }
    }

    pub fn get(
        &mut self,
        shell: Option<&PathBuf>,
        args: &[String],
    ) -> Result<HashMap<String, String>, Error> {
        self.delivery().map_err(Error::Create)?;
        let output = self.output(shell, args).map_err(Error::Executing)?;
        let stdout = from_utf8(&output.stdout).map_err(Error::Decoding)?;
        let stderr = from_utf8(&output.stderr).map_err(Error::Decoding)?;
        decoder::decode(stdout).map_err(|e| {
            Error::Parsing(
                e.to_string(),
                output.status.code(),
                stdout.to_owned(),
                stderr.to_owned(),
            )
        })
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Removes extractor file from OS temporary folder. `envvars` creates a small
/// executable file in OS temporary folder. This application drops a list of
/// available environment variables and does nothing else. As soon as the
/// extractor has been created, `envvars` uses it. But it still can be safely
/// removed for cleaning purposes for example before closing of an application.
///
/// If `envvars` doesn't detect an extractor, it will be created again.
///
/// Note, `envvars` doesn't remove an extractor application automatically.
pub fn cleanup() -> Result<(), io::Error> {
    let path = get_extractor_path();
    if !path.exists() {
        Ok(())
    } else {
        remove_file(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{profiles::get as get_profiles, Profile, EXTRACTOR};

    fn extract() -> Result<(), Error> {
        let mut profiles = get_profiles()?;
        let mut failed: Vec<(Profile, Error)> = vec![];
        println!("{}", "=".repeat(50));
        println!("Found shells with detected envvars:");
        println!("{}", "=".repeat(50));
        profiles.iter_mut().for_each(|p| {
            if let Err(err) = p.load() {
                failed.push((p.clone(), err));
            } else {
                println!(
                    "{}: {:?}; (detected variables: {}); symlink={}",
                    p.name,
                    p.path,
                    if let Some(envvars) = p.envvars.as_ref() {
                        envvars.len()
                    } else {
                        0
                    },
                    p.symlink
                );
            }
        });
        assert!(!profiles.is_empty());
        println!("{}", "=".repeat(50));
        println!("Found shells with failed detection of envvars:");
        println!("{}", "=".repeat(50));
        failed.iter().for_each(|(p, err)| match err {
            Error::Parsing(_err, code, stdout, stderr) => {
                println!("{}: {:?}; code: {code:?}", p.name, p.path,);
                println!("{}: {:?}; stdout:\n{stdout}", p.name, p.path,);
                println!("{}: {:?}; stderr:\n{stderr}", p.name, p.path,);
            }
            _ => {
                println!("{}: {:?}; fail to get envvars: {err}", p.name, p.path,);
            }
        });
        Ok(())
    }
    #[test]
    fn test() {
        // Extracting
        extract().expect("Envvars should be extracted");
        // Remove extractor
        let extractor_path = get_extractor_path();
        remove_file(&extractor_path).expect("Extractor should removed");
        // Extracting again
        extract().expect("Envvars should be extracted");
        // Damage extractor
        let mut file = OpenOptions::new()
            .write(true)
            .open(&extractor_path)
            .expect("Extractor file should be opened");
        file.write_all(&[0, 0, 0, 0, 0, 0, 0, 0, 0])
            .expect("Data should be written into extractor");
        file.flush().expect("Data should be flushed into extractor");
        drop(file);
        // Extracting again
        extract().expect("Envvars should be extracted");
        // Extractor should detect changes on executable file with invalid hash and rewrite
        // executable file again
        assert!(!EXTRACTOR.lock().expect("Access to extractor").invalid_hash);
    }
}
