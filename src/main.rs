mod ls;

use std::env;
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
        .get_matches();

    let mut working_dir = env::current_dir().unwrap();
    let new_path = matches.value_of("path").unwrap_or("");
    working_dir.push(new_path);

    if !working_dir.is_dir() {
        println!("{} is not a directory", &new_path);
    } else {
        ls::list_contents(&working_dir).unwrap();
    }
}