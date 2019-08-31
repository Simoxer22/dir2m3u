use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{PathBuf,Path};
use std::ffi::OsString;

pub struct Config
{
    pub directory : PathBuf,
    pub recursive : bool,
}

impl Config
{
    pub fn new(dir: &str, rec: bool) -> Result<Config, Box<dyn Error>> {
        Ok(Config {
            directory: PathBuf::from(dir).canonicalize()?,
            recursive: rec,
        })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    //TODO Add recursion
    let mut filename = conf.directory.file_name().unwrap().to_str().unwrap().to_owned();
    filename.push_str(".m3u");
    make_playlist(&filename, &conf.directory)?;

    Ok(())
}

fn make_playlist(filename: &str, dir: &Path) -> Result<(),Box<dyn Error>> {
    let mut file = File::create(filename)?;
    file.write("#EXTM3U\n".as_bytes())?;
    for song in fs::read_dir(dir)? {
        let path = song?.path();
        if !path.is_dir() && path.extension().unwrap_or_default() == "mp3" {
            let songname = path.file_name().unwrap_or_default().to_str().unwrap_or_default();
            file.write(format!("#EXTINF:{0}\n{1}\n", &songname[..songname.len()-4] ,path.display()).as_bytes())?;
        }
    }

    Ok(())
}