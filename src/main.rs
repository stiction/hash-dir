use std::env;
use std::process;

use hash_dir::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|_err| {
        eprint_usage();
        process::exit(1);
    });

    println!("{:?}", config);
}

fn eprint_usage() {
    eprintln!("hash-dir dir output_file");
}
