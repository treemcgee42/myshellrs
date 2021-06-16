mod ls;

use std::env;
use std::path::PathBuf;
use clap::{App,Arg};

fn main() {
    let matches = App::new("ls-util")
        .version("0.1")
        .author("Rooney Malladi")
        .about("Rust implementation of `ls`")
        .arg(Arg::with_name("path")
            .required(false)
            .takes_value(true)
            .help("operate on directory other than current one"))
        .arg(Arg::with_name("show_dot")
            .required(false)
            .takes_value(false)
            .short("a")
            .long("all")
            .help("show files beginning with '.' (hidden by default)"))
        .arg(Arg::with_name("default_dir")
            .required(false)
            .takes_value(true)
            .long("default-dir"))
        .get_matches();

    let mut working_dir = match matches.value_of("default_dir") {
        None => env::current_dir().unwrap(),
        Some(s) => PathBuf::from(s),
    };

    let new_path = matches.value_of("path").unwrap_or("");
    working_dir.push(new_path);

    if !working_dir.is_dir() {
        println!("{} is not a directory", &new_path);
    } else {
        let show_dot: bool = matches.is_present("show_dot");

        ls::list_contents(&working_dir, show_dot).unwrap();
    }
}