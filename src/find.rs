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
    // println!("Reading{:?}{}", c.target_file, c.dir)
    // get_dir(c.dir, c);
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        match fs::metadata(path) {
            Ok(metadata) => {
                if metadata.is_file() {
                    println!("It is a file.");
                } else if metadata.is_dir() {
                    read_dir(&path, c);
                    println!("It is a directory.");
                } else {
                    println!("It is neither a file nor a directory.");
                }
            }
            Err(err) => {
                print!("err{}", err)
            }
        }
        // if entry.into().is_dir() {
        //     println!("dir is :{}", path.display());
        //     get_dir(path, c);
        // }

        let name = entry.file_name();
        if name == c.target_file {
            print!("---yes it is--------------------------------");
            return Ok(());
        }
    }
    Ok(())

    /* println!("file is :{}", c.target_file.display());
    println!("dir is :{}", c.dir.display()); */
}

// pub fn get_dir(dir: PathBuf, c: Config) -> anyhow::Result<()> {
//     Ok(())
// }
