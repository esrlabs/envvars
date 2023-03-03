use super::paths;
use fs_extra::dir;
use std::io::{Error, ErrorKind};

pub fn clear() -> Result<(), Error> {
    dir::remove(paths::extractor_dest_dir()?)
        .map_err(|e| Error::new(ErrorKind::Other, format!("{e:?}")))
}
