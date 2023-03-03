use std::{collections::HashMap, env};

pub fn main() {
    let mut envvars: HashMap<String, String> = HashMap::new();
    for (key, value) in env::vars() {
        envvars.insert(key, value);
    }
    println!("{}", serde_json::to_string(&envvars).unwrap());
}
