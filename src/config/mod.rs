use serde::{Deserialize, Serialize};
use std::fs;
use crate::MacroType;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    pub macros: Vec<MacroEntry>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct MacroEntry {
    pub key: String,
    pub text: Option<String>,
    pub macro_type: MacroType,
    pub args: Option<Vec<String>>,
    pub fn_name: Option<String>,
}

pub fn load_config() -> Config {
    println!("{:?}", std::env::current_dir());

    let data = fs::read_to_string("./macros.json").expect("Unable to read config file...");

    let config: Config = serde_json::from_str(&data).unwrap();

    println!("{:?}", config);

    config
}