#[macro_use]
extern crate lazy_static;

use std::io::Error;

mod cleaner;
mod extractor;
mod injector;
mod paths;

fn main() -> Result<(), Error> {
    extractor::copy_sources()?;
    extractor::build()?;
    injector::inject()?;
    cleaner::clear()?;
    Ok(())
}
