use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};

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
        recursive(|d| make_playlist(&dest, d), &conf.directory)?;
    } else {
        make_playlist(&dest, &conf.directory)?;
    }

    Ok(())
}

/// Traverse all directories contained in `dir` recursively and call `make_playlist` on each one
/// 
/// # Arguments
/// + `dest` destination path
/// + `dir` directory to 
/// 
fn recursive<F> (closure: F, dir: &PathBuf) -> Result<(), Box<dyn Error>> 
    where F : Fn(&PathBuf) -> Result<(), Box<dyn Error>> + Copy {
    let dirs = dir
        .read_dir()?
        .filter_map(|e| match e {
            Ok(entry) => Some(entry.path()),
            Err(_) => None,
        })
        .filter(|p| p.is_dir());

    for dir in dirs {
        recursive(closure, &dir)?;
    }
    (&closure)(&dir)
}

/// Call `gen_playlist` and write it to file
fn make_playlist(dest: &PathBuf, dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let songs = dir
        .read_dir()?
        .filter_map(|e| match e {
            Ok(entry) => Some(entry.path()),
            Err(_) => None,
        })
        .filter(|p| p.is_file() && p.extension().unwrap_or_default() == "mp3");

    if let Some(buf) = gen_playlist(songs)? {
        let mut filename = dir.canonicalize()?.file_name().unwrap().to_owned();
        filename.push(".m3u");
        let mut filepath = dest.to_owned();
        filepath.push(filename);

        let mut file = File::create(filepath)?;
        file.write_all(buf.as_bytes())?;
    }
    Ok(())
}

const M3U_HEAD: &str = "#EXTM3U\n";

/// Iterate over `mp3` and create m3u format String
fn gen_playlist<I : Iterator<Item=PathBuf>>(songs: I) -> Result<Option<String>, Box<dyn Error>> {
    let mut buf = String::from(M3U_HEAD);

    for song in songs {
        let songname = song.file_stem().unwrap_or_default();
        buf.push_str(format!("#EXTINF:{0:?}\n{1}\n", songname, song.display()).as_str());
    }
    if buf.len() > M3U_HEAD.len() {
        Ok(Some(buf))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gen_playlist() {
        let testpaths = vec![
            PathBuf::from("./dir1/dir2/songname1.mp3"),
            PathBuf::from("./dir1/dir3/songname2.mp3"),
            PathBuf::from("./songname3.mp3"),
            PathBuf::from("./dir1/dir3/dir2/songname4.mp3"),
        ].into_iter();

        let generated_lines = super::gen_playlist(testpaths).unwrap().unwrap();

        let mut generated_iter = generated_lines.split('\n');

        assert_eq!(
            generated_iter.next(),
            Some(M3U_HEAD.trim_end())
        );

        assert_eq!(
            generated_iter.next().unwrap()[..8],
            "#EXTINF:"[..8]
        )

    }
}
