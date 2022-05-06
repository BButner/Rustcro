mod macros;
mod config;
use chrono::Utc;
use std::env;
use crate::gui::launch_gui;
use crate::macros::MacroType;

mod gui;

fn main() {
    let start = Utc::now().time();
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Raw type a Macro, don't load the config
        if args[1] == "raw" {
            if args.len() < 3 {
                eprintln!("Must supply a 3rd argument to type for a raw macro!");
            }

            // macros::type_macro(&args[2..].join(" "));
            return;
        }
    }

    let config = config::load_config();

    launch_gui(config);

    // let m = config.typing_macros.iter().position(|m| m.key == macro_to_type);
    //
    // if m.is_some() {
    //     macros::type_macro(&config.typing_macros.get(m.unwrap()).clone().unwrap().text);
    // } else {
    //     eprintln!("Could not find a Macro by the name of \"{}\"...", macro_to_type);
    // }

    let end = Utc::now().time();
    println!("Execution Time: {}ms", (end - start).num_milliseconds());
}