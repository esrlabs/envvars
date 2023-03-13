#[macro_use]
extern crate lazy_static;

use std::io::Error;

mod cleaner;
mod extractor;
mod injector;
mod paths;

#[macro_export]
macro_rules! cargo_log {
    ($($tokens: tt)*) => {
        let level = if let Some(l) = std::env::var_os("ENVVARS_CARGO_LOG_LEVEL") {
            l.to_string_lossy().to_string()
        } else {
            String::from("debug")
        };
        println!("cargo:{level}={}", format!($($tokens)*))
    }
}

fn main() -> Result<(), Error> {
    extractor::copy_sources()?;
    extractor::build()?;
    injector::inject()?;
    cleaner::clear()?;
    Ok(())
}
