use std::env;
use chrono::Utc;
mod types;

pub fn run_macro() {
    let start = Utc::now().time();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Args must be > 1");
    }

    if args[1].contains("run_macro=") {
        match args[1].as_str() {
            "run_macro=code_newest_directory" => types::vscode::code_newest_directory(&args[1..].to_vec()),
            _ => eprintln!("Could not find match for {}", args[1])
        }
    } else {
        types::typing::type_macro(&args[1..].to_vec());
    }

    let end = Utc::now().time();
    println!("Execution Time: {}ms", (end - start).num_milliseconds());
}