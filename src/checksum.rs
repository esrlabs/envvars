use std::{fs::File, io, io::Read, path::PathBuf};

pub fn checksum(filename: &PathBuf) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0; 65536];
    loop {
        match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                hasher.update(&buffer[..n]);
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    Ok(hasher.finalize().to_string())
}
