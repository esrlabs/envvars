use crate::{profiles::Profile, Error};
use is_terminal::IsTerminal;
use std::{fs::read_to_string, path::Path};

const SHELLS_FILE_PATH: &str = "/etc/shells";

pub(crate) fn get() -> Result<Vec<Profile>, Error> {
    let shells_file_path = Path::new(SHELLS_FILE_PATH);
    if !shells_file_path.exists() {
        return Err(Error::NotFound(shells_file_path.to_path_buf()));
    }
    let mut profiles: Vec<Profile> = vec![];
    for shell in read_to_string(shells_file_path)
        .map_err(Error::Io)?
        .split('\n')
        .filter(|s| !s.starts_with('#') && !s.is_empty())
    {
        let path = Path::new(shell);
        let is_term = std::io::stdout().is_terminal();
        let profile = match Profile::new(
            &path.to_path_buf(),
            if path.ends_with("tcsh") || path.ends_with("csh") {
                if is_term {
                    vec!["-c"]
                } else {
                    vec!["-ic"]
                }
            } else {
                if is_term {
                    log::info!("TTY detected");
                    vec!["-l", "-c"]
                } else {
                    log::info!("no TTY");
                    vec!["-i", "-l", "-c"]
                }
            },
            None,
        ) {
            Ok(profile) => profile,
            Err(err) => {
                log::warn!("Cannot get envvars for {shell}: {err}");
                continue;
            }
        };
        profiles.push(profile);
    }
    Ok(profiles)
}
