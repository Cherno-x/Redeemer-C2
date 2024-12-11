use github::GithubConfig;
use std::{error::Error, fs::{read_dir, File}, io::Read, path::{Path, PathBuf}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub github: GithubConfig,
}

pub fn load_config(path:&PathBuf) -> Result<Config, Box<dyn Error>>{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

pub fn find_bof_files_in_dir(directory: &Path) -> Vec<PathBuf> {
    let mut o_files = Vec::new();

    // 读取目录内容
    if let Ok(entries) = read_dir(directory) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "o") {
                    o_files.push(path);
                }
            }
        }
    }

    o_files
}