use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::{self, Path, PathBuf};

#[derive(Debug)]
pub struct Config {
    pub dir: String,
    pub output_file: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let dir = match args.next() {
            Some(arg) => arg,
            None => return Err("no dir"),
        };

        let output_file = match args.next() {
            Some(arg) => arg,
            None => return Err("no output_file"),
        };

        Ok(Config { dir, output_file })
    }
}

#[derive(Debug)]
pub struct HashInfo {
    pub path: PathBuf,
    pub hash: String,
}

pub fn dir_files(dir: &Path) -> io::Result<HashMap<String, HashInfo>> {
    let mut files: HashMap<String, HashInfo> = HashMap::new();
    visit_dirs(dir, "", &mut |key, path: PathBuf| {
        files.insert(
            key,
            HashInfo {
                path,
                hash: String::new(),
            },
        );
    })?;
    Ok(files)
}

fn visit_dirs(dir: &Path, prefix: &str, cb: &mut dyn FnMut(String, PathBuf)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap();

            if path.is_dir() {
                let prefix = prefix.to_string() + &file_name + path::MAIN_SEPARATOR_STR;
                visit_dirs(&path, &prefix, cb)?;
            } else {
                let key = format!("{}{}", prefix, &file_name);
                cb(key, path);
            }
        }
    }
    Ok(())
}
