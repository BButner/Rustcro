use std::env;
use enigo::{Enigo, Key, KeyboardControllable};
use chrono::Utc;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vec: Vec<i64> = Vec::new();

    test_key_sequence(&args, &mut vec);

    let avg = average(&vec);

    println!("Execution Average: {}ms", avg);
}

fn average(vec: &Vec<i64>) -> f32 {
    return vec.iter().sum::<i64>() as f32 / vec.len() as f32;
}

fn test_key_sequence(args: &Vec<String>, vec: &mut Vec<i64>) {
    let mut enigo = Enigo::new();

    for i in 1..11 {
        let start = Utc::now().time();

        for s in &args[1..] {
            // enigo.key_sequence(s);

            for char in s.chars() {
                // enigo.key_click(get_key(char));
                enigo.key_down(get_key(char));
                enigo.key_up(get_key(char));
            }
        }

        let end = Utc::now().time();

        let diff = end - start;

        vec.push(diff.num_milliseconds());
    }
}

fn get_key(char: char) -> Key {
    return Key::Layout(char);
}