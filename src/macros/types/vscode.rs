use std::process::Command;
use rustcro::get_newest_directory;

pub fn code_newest_directory(path: &String) {
    let latest_dir = get_newest_directory(path);

    if latest_dir.is_some() {
        // Launch VSCode to the most recent sub-directory
        let arg = latest_dir.unwrap().path().to_str().unwrap().to_owned();

        let command =
            if cfg!(target_os = "windows")
            { Command::new("cmd").args(["/C", "code", &arg]).spawn() } else { Command::new("code").arg(&arg).spawn() };

        if command.is_err() {
            println!("Error on attempting to launch Code...");
            println!("Error: {}", command.err().unwrap());
        }
    }
}