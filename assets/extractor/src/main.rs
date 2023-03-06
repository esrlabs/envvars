use std::env;

const KEY_VALUE_SEP: &str = include_str!("./keys/key_value_sep.txt");
const PAIR_SEP: &str = include_str!("./keys/pair_sep.txt");
const MARKER: &str = include_str!("./keys/marker.txt");

pub fn main() {
    let mut output: String = String::new();
    for (key, value) in env::vars() {
        output = format!(
            "{output}{:?}{KEY_VALUE_SEP}{:?}{PAIR_SEP}",
            key.as_bytes(),
            value.as_bytes()
        );
    }
    println!("{MARKER}{output}{MARKER}");
}
