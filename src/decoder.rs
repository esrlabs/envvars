use crate::Error;
use std::{collections::HashMap, str::from_utf8};

const KEY_VALUE_SEP: &str = include_str!("../assets/extractor/src/keys/key_value_sep.txt");
const PAIR_SEP: &str = include_str!("../assets/extractor/src/keys/pair_sep.txt");
const MARKER: &str = include_str!("../assets/extractor/src/keys/marker.txt");

fn decode_value(str: &str) -> Option<String> {
    if let Ok(bytes) = serde_json::from_str::<Vec<u8>>(str) {
        if let Ok(value) = from_utf8(&bytes) {
            return Some(value.to_string());
        }
    }
    None
}

fn decode_pair(pair: &str) -> Option<(String, String)> {
    let pairs: Vec<&str> = pair.split(KEY_VALUE_SEP).collect();
    if pairs.len() != 2 {
        return None;
    }
    let key = decode_value(pairs[0]);
    let value = decode_value(pairs[1]);
    if let (Some(key), Some(value)) = (key, value) {
        Some((key, value))
    } else {
        None
    }
}

pub(crate) fn decode(stdout: &str) -> Result<HashMap<String, String>, Error> {
    let mut map: HashMap<String, String> = HashMap::new();
    let splitted: Vec<&str> = stdout.split(MARKER).collect();
    if splitted.len() != 3 {
        return Err(Error::NoExtractorOutput);
    }
    let pairs: Vec<&str> = splitted
        .get(1)
        .ok_or(Error::NoExtractorOutput)?
        .split(PAIR_SEP)
        .collect();
    pairs.iter().for_each(|pair| {
        if let Some((key, value)) = decode_pair(pair) {
            map.insert(key, value);
        }
    });
    Ok(map)
}

#[test]
fn test() {
    let pair_01 = ("key_01", "value_01");
    let pair_02 = ("key_02", "value_02");
    let map = decode(&format!(
        "{MARKER}{:?}{KEY_VALUE_SEP}{:?}{PAIR_SEP}{:?}{KEY_VALUE_SEP}{:?}{PAIR_SEP}{MARKER}",
        pair_01.0.as_bytes(),
        pair_01.1.as_bytes(),
        pair_02.0.as_bytes(),
        pair_02.1.as_bytes(),
    ))
    .expect("stdout should be decoded");
    assert_eq!(map.len(), 2);
    assert_eq!(
        map.get(pair_01.0).expect("Should have defined key"),
        pair_01.1
    );
    assert_eq!(
        map.get(pair_02.0).expect("Should have defined key"),
        pair_02.1
    );
}
