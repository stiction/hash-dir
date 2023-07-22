use std::env;

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
