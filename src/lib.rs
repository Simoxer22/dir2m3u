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
    let directories = dir.read_dir()?
        .take_while(|file| match file {
            Ok(file) => file.path().is_dir(),
            Err(_) => false,
        });

    for dir in directories {
        let p = dir?.path();
        eprintln!("{:?}", p.display());
        recursive(dest, p)?;
    }

    make_playlist(&dest, dir)
}

fn make_playlist (dest: &PathBuf, dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut filename = dir.canonicalize()?
        .file_name().unwrap()
        .to_str().unwrap()
        .to_owned();
    filename.push_str(".m3u");

    let mut filepath = dest.to_owned();
    filepath.push(filename);

    let mut file = File::create(filepath)?;
    write_playlist(&mut file, &dir)?;
    Ok(())
}

fn write_playlist(file: &mut File, dir: &Path) -> Result<(),Box<dyn Error>> {
    let songs = dir.read_dir()?
        .take_while(|file| match file {
            Ok(file) => match file.path().is_dir() {
                false => file.path().extension().unwrap_or_default() == "mp3",
                true => false,
            },
            Err(_) => false,
        });
    let mut writeheader = true;

    for song in songs {
        if writeheader {
            writeheader = false;
            file.write("#EXTM3U\n".as_bytes())?;
        }
        let path : PathBuf = song?.path();
        let songname = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
        file.write(format!("#EXTINF:{0}\n{1}\n", songname, path.display()).as_bytes())?;
    }

    Ok(())
}