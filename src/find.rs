use std::fs;
use std::path::PathBuf;

// use anyhow::{Error, Ok};

pub struct Config {
    target_file: PathBuf,
    dir: PathBuf,
}

impl Config {
    pub fn new(target_file: PathBuf, dir: PathBuf) -> Config {
        Config { target_file, dir }
    }
}

pub fn read_dir(dir: &PathBuf, c: &Config) -> anyhow::Result<()> {
    let dir = PathBuf::from(dir);
    match get_dir(&dir) {
        Ok(paths) => {
            for path in paths {
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("It is a file.{}", path.display());
                            if path.file_name() == Some(c.target_file.as_os_str()) {
                                print!("---yes it is--------------------------------");
                                return Ok(());
                            }
                        } else if metadata.is_dir() {
                            read_dir(&path, c)?;
                        } else {
                            println!("It is neither a file nor a directory.");
                        }
                    }
                    Err(err) => {
                        print!("err{}", err)
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}

pub fn get_dir(dir: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    let result = fs::read_dir(dir)?;
    let paths: Result<Vec<_>, _> = result.map(|entry| entry.map(|e| e.path())).collect();
    paths
}
