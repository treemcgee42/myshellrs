mod ls;
mod table;

use std::io;
use std::io::Write;
use std::env;

fn main() {
    loop {
        let mut current_dir = env::current_dir().unwrap();
        print!("{}> ", current_dir.display());
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read command.");

        if command=="ls\n" {
            ls::ls(&current_dir).unwrap();
        }
    }
}
