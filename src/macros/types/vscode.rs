use std::fs;
use std::path::Path;
use std::process::Command;

pub fn code_newest_directory(path: &String) {
    if !Path::new(path).exists() {
        eprintln!("Could not find path \"{}\"", path);
        return;
    }

    // Get all the sub-directories in the path argument
    let sub_dirs = fs::read_dir(path).unwrap();

    // Attempts to get the latest created sub-directory
    let latest_dir =
        sub_dirs
            .into_iter()
            .max_by_key(|d| d.as_ref().unwrap().metadata().unwrap().modified().unwrap());

    if latest_dir.is_some() {
        // Launch VSCode to the most recent sub-directory
        let arg = latest_dir.unwrap().unwrap().path().to_str().unwrap().to_owned();

        let command =
            if cfg!(target_os = "windows")
            { Command::new("cmd").args(["/C", "code", &arg]).spawn() }
            else { Command::new("code").arg(&arg).spawn() };

        if command.is_err() {
            println!("Error on attempting to launch Code...");
            println!("Error: {}", command.err().unwrap());
        }
    }
}