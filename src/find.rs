use std::path::PathBuf;
use std::{env, fs};

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
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        match fs::metadata(&path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    println!("It is a file.{}", path.display());
                    // let name = path.file_name();
                    if path.file_name() == Some(c.target_file.as_os_str()) {
                        print!("---yes it is--------------------------------");
                        return Ok(());
                    }
                } else if metadata.is_dir() {
                    return read_dir(&path, c);
                } else {
                    println!("It is neither a file nor a directory.");
                }
            }
            Err(err) => {
                print!("err{}", err)
            }
        }
    }
    Ok(())
}
