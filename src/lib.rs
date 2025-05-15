use std::collections::HashSet;
use std::fs;
use std::fs::{create_dir_all, read_dir, DirEntry};
use std::iter::FromIterator;
use std::path::PathBuf;

use clap::Parser;
use log::{debug, info};

#[derive(Parser)]
pub struct Cli {
    /// Path where to search for new files
    pub path_source: PathBuf,
    /// Path to where files will be copied
    pub path_target: PathBuf,

    /// Only list files and do not copy yet
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
}

impl Cli {
    pub fn len_path_source(&self) -> usize {
        self.path_source.components().count()
    }
    pub fn len_path_target(&self) -> usize {
        self.path_target.components().count()
    }
}

pub fn remove_path_root(root_length: usize, files_source: &HashSet<PathBuf>) -> Vec<PathBuf> {
    let mut files_rel: Vec<PathBuf> = vec![];
    for chunk in files_source {
        files_rel.push(chunk.components().skip(root_length).collect())
    }
    files_rel
}

pub fn list_files(dir_path: &PathBuf) -> HashSet<PathBuf> {
    let files: Vec<Result<DirEntry, std::io::Error>> = read_dir(dir_path).unwrap().collect();

    let mut files_path: Vec<PathBuf> = vec![];
    for element in files {
        let element_path: PathBuf = element.unwrap().path();
        if element_path.is_file() {
            files_path.push(element_path);
        } else {
            let files_dir: HashSet<PathBuf> = list_files(&element_path);
            for path in files_dir {
                files_path.push(path);
            }
        }
    }
    HashSet::from_iter(files_path)
}

pub fn sync_files(input: &Cli) {
    let files_source: HashSet<PathBuf> = list_files(&input.path_source);
    let files_target: HashSet<PathBuf> = list_files(&input.path_target);

    info!("Processing Files for {}", input.path_source.display());
    
    info!(
        "Files from Source {}, Files from Target {}",
        files_source.len(),
        files_target.len()
    );

    // remove path root from all paths for comparison
    let files_source_rel: Vec<PathBuf> = remove_path_root(input.len_path_source(), &files_source);
    let files_target_rel: Vec<PathBuf> = remove_path_root(input.len_path_target(), &files_target);

    let files_source_set: HashSet<PathBuf> = HashSet::from_iter(files_source_rel);
    let files_target_set: &HashSet<PathBuf> = &HashSet::from_iter(files_target_rel);

    let files_source_new = files_source_set.difference(files_target_set);

    for element in files_source_new {
        let dummy: &PathBuf = element;
        let full_path_source = &mut input.path_source.clone();
        let full_path_target = &mut input.path_target.clone();

        full_path_source.push(dummy);
        full_path_target.push(dummy);

        debug!(
            "COPY {:?} | {:?}  >>> {:?} ",
            element.file_name().unwrap(),
            full_path_source,
            full_path_target
        );

        if !input.dry_run {
            if !full_path_target.parent().unwrap().exists() {
                let create_res = create_dir_all(full_path_target.parent().unwrap());
                info!("{:?}", create_res)
            }

            let copy_result = fs::copy(full_path_source, full_path_target).unwrap();
            info!("{:?}", copy_result)
        }
    }
}
