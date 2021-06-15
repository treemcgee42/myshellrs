mod ls;

use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    
    ls::ls(&current_dir);
}
