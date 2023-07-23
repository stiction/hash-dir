use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use hash_dir::dir_files;
use hash_dir::md5_file;
use hash_dir::sorted_hash_result;
use hash_dir::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|_err| {
        eprint_usage();
        process::exit(1);
    });

    let mut file_output = File::options()
        .write(true)
        .create_new(true)
        .open(&config.output_file)
        .unwrap_or_else(|err| {
            eprintln!("cannot create file {} {}", &config.output_file, err);
            process::exit(1);
        });

    let mut files = dir_files(Path::new(&config.dir)).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let count = files.len();

    let mut index = 0usize;
    files.iter_mut().for_each(|(key, info)| {
        let path = &info.path;
        let hash = md5_file(path).unwrap();
        info.hash = hash;

        index += 1;
        eprintln!("{}/{} {}", index, count, key);
    });

    let text = sorted_hash_result(&files);
    file_output
        .write_all(text.as_bytes())
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });
}

fn eprint_usage() {
    eprintln!("hash-dir dir output_file");
}
