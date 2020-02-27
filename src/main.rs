use std::io::{Read, stdin, stdout, Write};
use std::process::{Command, Stdio};

fn main() {
    loop {
        print!("rsh> ");
        stdout().flush().unwrap();

        let mut command = String::new();
        stdin().read_line(&mut command)
            .expect("Failed to read user input");

        match command.trim() {
            "cd" => {}
            "exit" => return,
            command => {
                let output = Command::new(command)
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("An error occurred while executing process");
                let mut message = String::new();
                match output.stdout.unwrap().read_to_string(&mut message) {
                    Ok(_) => { println!("{}", message) }
                    Err(_why) => panic!("couldn't execute {} command", command)
                };
            }
        }
    }
}
