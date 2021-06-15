mod ls;

use std::env;

fn main() {
    let mut working_dir = env::current_dir().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len()>1 {
        working_dir.push(&args[1]);
        if !working_dir.is_dir() {
            println!("{} is not a directory", &args[1]);
        } else {
            ls::list_contents(&working_dir);
        }
    } else {
        ls::list_contents(&working_dir);
    }
}