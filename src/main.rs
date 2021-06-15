
use std::io;
use std::io::Write;
use std::env;
use std::process::Command;

fn main() {
    let mut working_dir = env::current_dir().unwrap();
    loop {
        print!("{}> ", working_dir.display());
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read command.");
        command = command.trim().to_string();

        let mut args = command.split(' ');

        match args.next() {
            None => (),
            Some("") => (),
            Some("ls") => {
                let arg_string = args.fold(String::new(), |acc,x| acc+x);
                Command::new("./ls-util").arg(arg_string).status()
                    .expect("`ls` failed to run");
            }
            Some(_) => {
                println!("Unknown command");
            }
        }
    }
}
