use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    pub typing_macros: Vec<TypingMacro>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct TypingMacro {
    pub key: String,
    pub text: String,
}

pub fn load_config() -> Config {
    let data = fs::read_to_string("/Users/bbutner/Projects/rustcro/src/config/macros.json").expect("Unable to read config file...");

    let config: Config = serde_json::from_str(&data).unwrap();

    println!("{:?}", config);

    config
}