use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};

pub struct Config {
    pub directory: PathBuf,
    pub recursive: bool,
}

impl Config {
    pub fn new(dir: &str, rec: bool) -> Config {
        Config {
            directory: PathBuf::from(dir),
            recursive: rec,
        }
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

/// Traverse all directories contained in `dir` and call `closure` on each directory
/// 
/// # Arguments
/// + `closure` closure that takes a directory as argument
/// + `dir` root directory of the recursion
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

/// Create and write `m3u` file (if valid)
/// 
/// # Arguments
/// + `destination` where you want the file to be written to
/// + `dir` directory structure to parse
fn make_playlist(destination: &PathBuf, dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    // Iterate over valid mp3 files
    let songs = dir
        .read_dir()?
        .filter_map(|e| match e {
            Ok(entry) => Some(entry.path()),
            Err(_) => None,
        })
        .filter(|p| p.is_file() && p.extension().unwrap_or_default() == "mp3");

    if let Some(buf) = gen_playlist(songs) {
        // Build playlist path
        let mut filename = dir.canonicalize()?.file_name().unwrap().to_owned();
        filename.push(".m3u");
        let mut filepath = destination.to_owned();
        filepath.push(filename);

        // Write m3u to file
        let mut file = File::create(filepath)?;
        file.write_all(buf.as_bytes())?;
    }
    Ok(())
}

const M3U_HEAD: &str = "#EXTM3U\n";

/// Create `m3u` content from an iterator over song paths
/// 
/// # Arguments
/// + `songs` Iterator containg the PathBuf of the song files
fn gen_playlist<I : Iterator<Item=PathBuf>>(songs: I) -> Option<String> {
    let mut buf = String::from(M3U_HEAD);

    for song in songs {
        let songname = song.file_stem().unwrap_or_default();
        buf.push_str(format!("#EXTINF:{0:?}\n{1}\n", songname, song.display()).as_str());
    }
    if buf.len() > M3U_HEAD.len() {
        Some(buf)
    } else {
        None
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
        ];

        let ret = super::gen_playlist(testpaths.clone().into_iter());

        assert!(ret.is_some());

        let generated_lines = ret.unwrap();

        let mut generated_iter = generated_lines.split('\n');

        assert_eq!(
            generated_iter.next(),
            Some(M3U_HEAD.trim_end())
        );

        for i in 0..testpaths.len() {
            assert_eq!(
                generated_iter.next().unwrap()[..8],
                "#EXTINF:"[..8]
            );
            assert_eq!(
                PathBuf::from(generated_iter.next().unwrap()),
                testpaths[i]
            );
        }
    }
}
