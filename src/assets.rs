include!(concat!(env!("OUT_DIR"), "/assets.rs"));

pub(crate) fn checksum() -> &'static str {
    CHECKSUM
}

pub(crate) fn filename() -> &'static str {
    FILENAME
}

pub(crate) fn bin() -> &'static [u8] {
    BIN
}
