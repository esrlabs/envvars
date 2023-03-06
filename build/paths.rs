use std::{
    env,
    fs::{create_dir, read_dir},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};
use uuid::Uuid;

lazy_static! {
    #[doc(hidden)]
    pub static ref TEMP_DIR: String = Uuid::new_v4().to_string();
}

pub fn ls_parent(path: &Path) -> String {
    if let Some(parent) = path.parent() {
        ls(parent)
    } else {
        "path doesn't have parent".to_string()
    }
}
pub fn ls(path: &Path) -> String {
    if path.exists() {
        if let Ok(inner) = read_dir(path) {
            format!(
                "\n{}",
                inner
                    .map(|entry| {
                        entry.map_or(String::new(), |entry| format!("{:?}", entry.path()))
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        } else {
            format!("Fail to read content of {path:?}")
        }
    } else {
        format!("{path:?} doesn't exist")
    }
}

fn if_exist(path: PathBuf) -> Result<PathBuf, Error> {
    if !path.exists() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("{path:?} doesn't exist. List: {}", ls_parent(&path)),
        ));
    }
    Ok(path)
}

pub fn manifest_dir() -> Result<PathBuf, Error> {
    if_exist(
        Path::new(
            &env::var("CARGO_MANIFEST_DIR").map_err(|_e| {
                Error::new(ErrorKind::NotFound, "CARGO_MANIFEST_DIR doesn't exist")
            })?,
        )
        .to_path_buf(),
    )
}

pub fn extractor_src_dir() -> Result<PathBuf, Error> {
    if_exist(assets_dir()?.join("extractor"))
}

pub fn extractor_dest_dir() -> Result<PathBuf, Error> {
    let path = env::temp_dir().join(TEMP_DIR.as_str());
    if !path.exists() {
        create_dir(&path)?;
    }
    if_exist(path)
}

pub fn assets_dir() -> Result<PathBuf, Error> {
    if_exist(manifest_dir()?.join("assets"))
}

pub fn extractor_executable() -> Result<PathBuf, Error> {
    if_exist(
        extractor_dest_dir()?
            .join("extractor")
            .join("target")
            .join("release")
            .join(executable_file_name()),
    )
}

pub fn executable_file_name() -> String {
    String::from(if cfg!(windows) {
        "extractor.exe"
    } else {
        "extractor"
    })
}

pub fn cargo_output_dir() -> Result<PathBuf, Error> {
    let out_dir = env::var_os("OUT_DIR").ok_or(Error::new(
        ErrorKind::NotFound,
        "Variable OUT_DIR doesn't exist".to_string(),
    ))?;
    if_exist(Path::new(&out_dir).to_path_buf())
}
