use std::env;
mod types;

pub fn type_macro(string_to_type: &String) {
    types::typing::type_macro(string_to_type);

    // if args[1].contains("run_macro=") {
    //     match args[1].as_str() {
    //         "run_macro=code_newest_directory" => types::vscode::code_newest_directory(&args[1..].to_vec()),
    //         _ => eprintln!("Could not find match for {}", args[1])
    //     }
    // } else {
    //     types::typing::type_macro(&args[1..].to_vec());
    // }
}