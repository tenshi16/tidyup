use std::env;
use std::{fs, io, path::PathBuf};
use chrono::{DateTime, Local};
use lazy_static::lazy_static;

fn get_files_in_folder(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;
    let all: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();
    Ok(all)
}

lazy_static! {
    static ref ORIGINAL_PATH: PathBuf = PathBuf::from("/home/tenshi/Documents/Test/"); // env::current_dir().unwrap();
}


fn select_and_copy(path: &str) -> () {
    let current_dir = path;
    let files = get_files_in_folder(&current_dir).unwrap_or_default();
    for file in files {
        let metadata = file.metadata().unwrap();
        if metadata.is_dir() { // Keep going recursevely to each new directory
            select_and_copy(file.to_str().unwrap_or_default())
        }
        else { // Main logic to copy values to new folder
            let creation_date = metadata.modified().unwrap();
            let local_creation_date: DateTime<Local> = creation_date.into();
            let folder_name = local_creation_date.format("%d-%m-%y").to_string();
            let mut folder_path = ORIGINAL_PATH.clone();
            folder_path.push(folder_name);
            match fs::create_dir(&folder_path) {
                Ok(()) => {
                    folder_path.push(file.file_name().unwrap());
                    println!("{:?}, {:?}", &file.to_str(), &folder_path.to_str());
                    fs::copy(file, folder_path).unwrap();
                }
                Err(err) => {
                    if err.kind() == io::ErrorKind::AlreadyExists {
                        folder_path.push(file.file_name().unwrap());
                        println!("{:?}, {:?}", &file.to_str(), &folder_path.to_str());
                        fs::copy(file, folder_path).unwrap();
                    } else {
                        println!("{:?}, folder path = {:?}", err, folder_path);
                    }
                }}
        }
    }
}


fn main() {
    let current_dir = String::from("/run/media/tenshi/HDD/photos/2021/"); // env::current_dir().unwrap().to_string_lossy().to_string();
    select_and_copy(&current_dir);
}
