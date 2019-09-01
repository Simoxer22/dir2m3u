extern crate clap;
use clap::{App,Arg};
use dir2m3u::Config;

fn main() {
    let matches = App::new("dir2m3u")
        .about("Create m3u playlists from directories")
        .arg(Arg::with_name("DIRECTORY")
            .default_value(".")
            .required(true)
            .help("directory used to create the playlist"))
        .arg(Arg::with_name("recursive")
            .short("r")
            .help("recursive: turn subdirectories into playlists too"))
        .arg(Arg::with_name("depth")
            .short("d")
            .help("set maximum depth for recursion (NOT IMPLEMENTED)"))
        .get_matches();
    
    let dir = matches.value_of("DIRECTORY").unwrap();
    let rec = matches.is_present("recursive");
    
    let conf = match Config::new(dir, rec) {
        Ok(conf) => conf,
        Err(why) => {
            println!("Error generating configuration: {:?}", why);
            std::process::exit(1);
        }
    };

    if let Err(why) = dir2m3u::run(conf) {
        println!("Error: {:?}", why);
        std::process::exit(2);
    }
}
