use std::env;
use enigo::{Enigo, Key, KeyboardControllable};
use chrono::Utc;

enum TypingType {
    Click,
    Press,
    KeySequence,
    KeySequenceParse,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vec_press: Vec<i64> = Vec::new();
    let mut vec_click: Vec<i64> = Vec::new();
    let mut vec_sequence: Vec<i64> = Vec::new();
    let mut vec_sequence_parse: Vec<i64> = Vec::new();

    test_key_sequence(&args, &mut vec_press, TypingType::Press);
    test_key_sequence(&args, &mut vec_click, TypingType::Click);
    test_key_sequence(&args, &mut vec_sequence, TypingType::KeySequence);
    test_key_sequence(&args, &mut vec_sequence_parse, TypingType::KeySequenceParse);

    let avg = average(&vec_press);
    let avg_click = average(&vec_click);
    let avg_sequence = average(&vec_sequence);
    let avg_sequence_parse = average(&vec_sequence_parse);

    println!("======= Average Execution Times =======");
    println!("Test Text: {:?}", &args[1..]);
    println!("key_down/key_up: {}ms ({}ms per key)", avg.0, avg.1);
    println!("key_press: {}ms ({}ms per key)", avg_click.0, avg_click.1);
    println!("key_sequence: {}ms ({}ms per key)", avg_sequence.0, avg_sequence.1);
    println!("key_sequence_parse: {}ms ({}ms per key)", avg_sequence_parse.0, avg_sequence_parse.1);
}

fn average(vec: &Vec<i64>) -> (f32,f32) {
    let average = vec.iter().sum::<i64>() as f32 / vec.len() as f32;
    return (average, average / vec.len() as f32);
}

fn test_key_sequence(args: &Vec<String>, vec: &mut Vec<i64>, typing_type: TypingType) {
    let mut enigo = Enigo::new();

    for _i in 1..101 {
        let start = Utc::now().time();

        for s in &args[1..] {
            match typing_type {
                TypingType::Click => {
                    for char in s.chars() {
                        enigo.key_click(get_key(char));
                    }
                }
                TypingType::Press => {
                    for char in s.chars() {
                        enigo.key_down(get_key(char));
                        enigo.key_up(get_key(char));
                    }
                }
                TypingType::KeySequence => {
                    enigo.key_sequence(s);
                }
                TypingType::KeySequenceParse => {
                    enigo.key_sequence_parse(s);
                }
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