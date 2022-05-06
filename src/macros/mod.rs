use crate::config::{MacroEntry};
use crate::macros::types::typing::type_macro;
use serde::{Deserialize, Serialize};
use crate::macros::types::vscode::code_newest_directory;

mod types;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MacroType {
    TYPING,
    FUNCTION,
}

pub fn run_macro(macro_entry: &MacroEntry) {
    match macro_entry.macro_type {
        MacroType::TYPING => {
            if macro_entry.text.is_some() {
                let text = &macro_entry.text.as_ref().unwrap();
                type_macro(&text);
            }
        }
        MacroType::FUNCTION => {
            if macro_entry.fn_name.is_none() {
                eprintln!("`fn_name` parameter was not populated!");
            }

            let fn_name = macro_entry.fn_name.as_ref().unwrap().as_str();

            match fn_name {
                "code_newest_directory" => {
                    if macro_entry.args.is_some() {
                        let args = macro_entry.args.as_ref().unwrap();

                        if args.len() != 1 {
                            eprintln!("Need only 1 argument for directory to search");
                            return;
                        }

                        code_newest_directory(&args[0]);
                    }
                }
                _ => eprintln!("Could not find fn...")
            }
        }
    }
}