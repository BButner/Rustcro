use std::fs;
use std::path::Path;
use std::process::Command;

pub fn code_newest_directory(args: &Vec<String>) {
    if args.len() < 2 {
        eprintln!("Must supply a path to search...");
        return;
    }

    let path = &args[1];

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
            .max_by_key(|d| d.as_ref().unwrap().metadata().unwrap().created().unwrap());

    if latest_dir.is_some() {
        // Launch VSCode to the most recent sub-directory
        Command::new("code").arg(latest_dir.unwrap().unwrap().path().to_str().unwrap()).spawn();
    }
}