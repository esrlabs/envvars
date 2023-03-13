use super::paths;
use fs_extra::dir;
use std::io::{Error, ErrorKind};

/// If ENVVARS_CRATE_EXTRACTOR_TEMP_DIR is used, we would not clear it after build
pub fn clear() -> Result<(), Error> {
    if paths::is_predefined_location_used() {
        Ok(())
    } else {
        dir::remove(paths::extractor_dest_dir()?)
            .map_err(|e| Error::new(ErrorKind::Other, format!("{e:?}")))
    }
}
