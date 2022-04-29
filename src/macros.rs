use std::env;
use enigo::{Enigo, Key, KeyboardControllable};

enum TypingMode {
    // Windows
    KeyDownUp,
    // macOS
    KeySequence,
}

pub fn run_macro() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Args must be > 1");
    }

    let mode: TypingMode =
        if env::consts::OS == "macos" { TypingMode::KeySequence } else { TypingMode::KeyDownUp };

    let string_to_type = &args[1..].join(" ");

    let mut enigo = Enigo::new();

    match mode {
        TypingMode::KeySequence => {
            enigo.key_sequence(string_to_type);
        }
        TypingMode::KeyDownUp => {
            for c in string_to_type.chars() {
                enigo.key_down(Key::Layout(c));
                enigo.key_up(Key::Layout(c));
            }
        }
    }
}