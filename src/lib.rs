use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Config {
    pub directory: PathBuf,
    pub recursive: bool,
}

impl Config {
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
    } else {
        make_playlist(&dest, conf.directory)?;
    }

    Ok(())
}

fn recursive(dest: &PathBuf, dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let dirs = dir
        .read_dir()?
        .map(|f| f.unwrap().path())
        .take_while(|p| p.is_dir());

    for dir in dirs {
        recursive(dest, dir)?;
    }
    make_playlist(&dest, dir)
}

fn make_playlist(dest: &PathBuf, dir: PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(buf) = gen_playlist(&dir)? {
        let mut filename = dir.canonicalize()?.file_name().unwrap().to_owned();
        filename.push(".m3u");
        let mut filepath = dest.to_owned();
        filepath.push(filename);

        let mut file = File::create(filepath)?;
        file.write(buf.as_bytes())?;
    }
    Ok(())
}

fn gen_playlist(dir: &Path) -> Result<Option<String>, Box<dyn Error>> {
    let mut buf = String::from("#EXTM3U\n");

    let songs = dir
        .read_dir()?
        .map(|file| file.unwrap().path())
        .take_while(|file| file.is_file() && file.extension().unwrap_or_default() == "mp3");

    for song in songs {
        let songname = song.file_stem().unwrap_or_default();
        buf.push_str(format!("#EXTINF:{0:?}\n{1}\n", songname, song.display()).as_str());
    }

    if buf.len() > "#EXTM3U\n".len() {
        Ok(Some(buf))
    } else {
        Ok(None)
    }
}
