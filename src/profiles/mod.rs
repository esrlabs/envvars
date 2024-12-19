use crate::{Error, EXTRACTOR};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

pub mod unix;
pub mod windows;

/// Definition of shell profile
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    /// Suggested name of shell. For unix based systems it will be name of executable file,
    /// like "bash", "fish" etc. For windows it will be names like "GitBash", "PowerShell"
    /// etc.
    pub name: String,
    /// Path to executable file of shell
    pub path: PathBuf,
    /// List of environment variables. Because extracting operation could take some time
    /// by default `envvars = None`. To load data should be used method `load`, which will
    /// make attempt to detect environment variables.
    pub envvars: Option<HashMap<String, String>>,
    /// true - if path to executable file of shell is symlink to another location.
    pub symlink: bool,
    /// Private field to store arguments needed to execute shell in right way to grab list
    /// of environment variables
    args: Vec<String>,
}

impl Profile {
    /// Creates shell's profile description
    /// * `shell` - path to shell's executable file
    /// * `args` - list of arguments needed to pass a command into shell. For example: "-c"
    ///    to have a full command like: "/etc/bin/bash -c cmd"
    /// * `name` - optional name for profile. For unix based systems it will be name of
    ///    executable file, like "bash", "fish" etc. For windows better to provide name to
    ///    have it like "GitBash", "PowerShell" etc.
    pub fn new(shell: &PathBuf, args: Vec<&str>, name: Option<&str>) -> Result<Self, Error> {
        let path = Path::new(shell);
        if !path.exists() {
            return Err(Error::NotFound(shell.clone()));
        }
        let symlink = fs::symlink_metadata(path)
            .map_err(Error::Io)?
            .file_type()
            .is_symlink();
        let name = if let Some(name) = name {
            name.to_string()
        } else {
            path.file_name()
                .ok_or(Error::Other(format!(
                    "Found {shell:?}, but cannot convert path"
                )))?
                .to_string_lossy()
                .to_string()
        };
        Ok(Profile {
            name,
            path: shell.clone(),
            envvars: None,
            symlink,
            args: args
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        })
    }
    /// Makes attempt to grab a list of environment variables for profile. It will
    /// spawn an instance of shell with extractor as command argument. If stdout will
    /// have suitable output, it will be parsed and list of environment variables will
    /// be saved in `self.envvars`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{path::PathBuf, str::FromStr};
    /// use envvars::Profile;
    ///
    /// let mut profile: Profile = if cfg!(windows) {
    ///     Profile::new(&PathBuf::from_str("C:\\Program Files\\Git\\bin\\bash.exe").unwrap(), vec!["-c"], None).unwrap()
    /// } else {
    ///     Profile::new(&PathBuf::from_str("/bin/bash").unwrap(), vec!["-c"], None).unwrap()
    /// };
    ///
    /// assert_eq!(profile.name, if cfg!(windows) {
    ///     "bash.exe"
    /// } else {
    ///     "bash"
    /// });
    ///
    /// profile.load().unwrap();
    ///
    /// assert!(profile.envvars.is_some());
    ///
    /// if let Some(vars) = profile.envvars.as_ref() {
    ///     assert!(vars.contains_key("PATH") || vars.contains_key("Path") || vars.contains_key("path"));
    /// }
    /// ```
    pub fn load(&mut self) -> Result<(), Error> {
        self.envvars = Some(
            EXTRACTOR
                .lock()
                .map_err(|e| Error::PoisonError(e.to_string()))?
                .get(Some(&self.path), &self.args)?,
        );
        Ok(())
    }
}

/// Returns all detected shell's profiles.
/// - Unix based systems: reads /etc/shells and creates Profile for each found shell
/// - Windows: checks most regulars shells like CMD, PowerShell, GitBash, Cygwin etc.
///
/// Because an operation of extracting of environment variables could take some time,
/// by default `Profile.envvars` is empty (None). To load data should be used method
/// `Profile.load`, which will make attempt to detect environment variables.
///
/// # Examples
///
/// ```
/// use envvars::{get_profiles, Profile};
///
/// let mut profiles: Vec<Profile> = get_profiles().unwrap();
///
/// profiles.iter_mut().for_each(|profile| {
///     // Attempt to load envvars
///     if let Err(err) = profile.load() {
///         eprintln!("Cannot load envvars for {}: {err}", profile.name);
///     }
/// });
/// ```
pub fn get() -> Result<Vec<Profile>, Error> {
    if cfg!(windows) {
        windows::get()
    } else if cfg!(unix) {
        unix::get()
    } else {
        Err(Error::NotSupportedPlatform)
    }
}
