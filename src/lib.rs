use std::error::Error;

pub struct Config
{
    pub directory : String,
    pub recursive : bool,
}

impl Config
{
    pub fn new(dir: &str, rec: bool) -> Config {
        Config {
            directory: String::from(dir),
            recursive: rec,
        }
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {

    Ok(())
}