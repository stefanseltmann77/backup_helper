use std::path::PathBuf;

use config::Config;
use env_logger::Env;
use serde_derive::Deserialize;

use backup_helper::{Cli, sync_files};

#[derive(Debug, Deserialize)]
struct PathMapping {
    source: String,
    target: String,
}

#[derive(Debug, Deserialize)]
struct AppConfig {
    dry_run: bool,
    path_mappings: Vec<PathMapping>,
}

impl AppConfig {
    fn new() -> AppConfig {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
        // let input = Cli::parse();

        let settings = Config::builder()
            .add_source(config::File::with_name("config_backups"))
            .build()
            .unwrap();
        let app_conf: AppConfig = settings.try_deserialize().unwrap();
        return app_conf;
    }
}

fn main() {
    let app_conf = AppConfig::new();
    let dry_run: bool = app_conf.dry_run;
    println!("Running in dry_run='{:?}'", dry_run);

    let mut inputs: Vec<Cli> = Vec::new();
    for path_mapping in app_conf.path_mappings.iter() {
        inputs.push(Cli {
            path_source: PathBuf::from(&path_mapping.source),
            path_target: PathBuf::from(&path_mapping.target),
            dry_run,
        });
    }
    for input in inputs {
        sync_files(&input)
    }
}
