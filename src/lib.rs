use std::fs;
use std::fs::{DirEntry};
use std::path::Path;

pub fn get_newest_directory(path: &String) -> Option<DirEntry> {
    if !Path::new(path).exists() {
        eprintln!("Could not find path \"{}\"", path);
        return None;
    }

    // Get all the sub-directories in the path argument
    let sub_dirs = fs::read_dir(path).unwrap();

    // Attempts to get the latest created sub-directory
    Option::from(
        sub_dirs
            .into_iter()
            .filter(|dir| dir.as_ref().unwrap().file_type().unwrap().is_dir())
            .max_by_key(|d| d.as_ref().unwrap().metadata().unwrap().modified().unwrap())
            .unwrap().unwrap()
    )
}

pub fn get_newest_file_in_dir(path: &String, file_ext: Option<String>) -> Option<DirEntry> {
    if !Path::new(path).exists() {
        eprintln!("Could not find path \"{}\"", path);
        return None;
    }

    let sub_files = fs::read_dir(path)
        .unwrap()
        .filter(|f| f.as_ref().unwrap().file_type().unwrap().is_file());

    if file_ext.is_some() {
        return Option::from(
            sub_files
                .filter(|f| {
                    let path = f.as_ref().unwrap().path();
                    let ext = Path::new(path.to_str().unwrap()).extension();

                    if ext.is_some() {
                        let result = String::from(ext.unwrap().to_str().unwrap()) == String::from(file_ext.as_ref().unwrap());
                        println!("{}", result);
                        return result;
                    }

                    false
                })
                .max_by_key(|f| f.as_ref().unwrap().metadata().unwrap().modified().unwrap())
                .unwrap().unwrap()
        );
    }

    Option::from(
        sub_files
            .max_by_key(|f| f.as_ref().unwrap().metadata().unwrap().modified().unwrap())
            .unwrap().unwrap()
    )
}