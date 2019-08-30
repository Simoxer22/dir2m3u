extern crate clap;
use clap::{App,Arg};
use dir2m3u::Config;

fn main() {
    let matches = App::new("dir2m3u")
        .about("Create m3u playlists from directories")
        .arg(Arg::with_name("DIRECTORY")
            .default_value(".")
            .required(true))
        .arg(Arg::with_name("recursive")
            .short("r")
            .help("Turn subdirectories into playlists too"))
        .get_matches();
    
    let dir = matches.value_of("DIRECTORY").unwrap();
    let rec = matches.is_present("recursive");
    
    let conf = Config::new(dir, rec);

    if let Err(why) = dir2m3u::run(conf) {
        println!("Error: {:?}", why);
        std::process::exit(2);
    }
}
