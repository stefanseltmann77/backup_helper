use std::collections::HashSet;
use std::fs::{DirEntry, read_dir};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use env_logger::Env;
use log::info;

fn list_files(dir_path: &Path) -> HashSet<PathBuf> {
    let files: Vec<Result<DirEntry, std::io::Error>> = read_dir(dir_path).unwrap().collect();
    let mut files_path: Vec<PathBuf> = vec![];
    for element in files {
        files_path.push(element.unwrap().path());
    }
    HashSet::from_iter(files_path.clone())
}


fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    info!("Starting Backup-Helper");
    let source_path = Path::new("M:/gesichert/docstash");
    let target_path = Path::new("I:/Dropbox/data/docstash");

    let files_source = list_files(source_path);
    let files_target = list_files(target_path);

    println!("Hello, world!");
    println!("Hello, world! {}", files_source.len());
    println!("Hello, world! {}", files_target.len());

    let files_difference = files_source.difference(&files_target);
    for element in files_difference {
        println!("{:?}", element)
    }
    println!("Check");
    let files_difference = files_source.intersection(&files_target);
    for element in files_difference {
        println!("{:?}", element)
    }
}
