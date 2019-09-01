use std::error::Error;
use std::fs::{File};
use std::io::Write;
use std::path::{PathBuf,Path};

pub struct Config
{
    pub directory : PathBuf,
    pub recursive : bool,
}

impl Config
{
    pub fn new(dir: &str, rec: bool) -> Result<Config, Box<dyn Error>> {
        Ok(Config {
            directory: PathBuf::from(dir),
            recursive: rec,
        })
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let dest = conf.directory.canonicalize()?;
    if conf.recursive {
        recursive(&dest, conf.directory)?;
    }
    else{
        make_playlist(&dest, conf.directory)?;
    }

    Ok(())
}

fn recursive(dest: &PathBuf, dir: PathBuf) -> Result<(), Box<dyn Error>> {
    for dir in dir.read_dir()? {
        let p = dir?.path();
        if p.is_dir() {
            recursive(dest, p)?;
        }
    }
    make_playlist(&dest, dir)
}

fn make_playlist (dest: &PathBuf, dir: PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(buf) = gen_playlist(&dir)?{

        let mut filename = dir.canonicalize()?
            .file_name().unwrap()
            .to_str().unwrap()
            .to_owned();
        filename.push_str(".m3u");

        let mut filepath = dest.to_owned();
        filepath.push(filename);

        let mut file = File::create(filepath)?;
        file.write(buf.as_bytes())?;
    }
    Ok(())
}

fn gen_playlist(dir: &Path) -> Result<Option<String>, Box<dyn Error>> {
    let mut buf = String::from("#EXTM3U\n");

    for song in dir.read_dir()? {
        let path : PathBuf = song?.path();

        if !path.is_dir() && path.extension().unwrap_or_default() == "mp3" {
            let songname = path.file_stem()
                .unwrap_or_default();
            buf.push_str(format!("#EXTINF:{0:?}\n{1}\n", songname, path.display()).as_str());
        }
    }
    
    if buf.len() > "#EXTM3U\n".len() {
        Ok(Some(buf))
    }
    else {
        Ok(None)
    }
}
