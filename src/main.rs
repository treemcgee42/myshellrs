
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
                // let arg_string = args.fold(String::new(), |acc,x| acc+x) 
                //     + " --default-dir "+&working_dir.clone().into_os_string().into_string()
                //     .expect("non-unicode os-string");
                Command::new("./ls-util")
                    .args(args).arg("--default-dir").arg(working_dir.clone().into_os_string().into_string().unwrap())
                    .status()
                    .expect("`ls` failed to run");
            }
            Some("cd") => {
                match args.next() {
                    None => (),
                    Some("..") => {
                        working_dir.pop();
                        ()
                    }
                    Some(s) => working_dir.push(s),
                }
            }
            Some("clear") => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            }
            Some(_) => {
                println!("Unknown command");
            }
        }
    }
}
