use std::env;
use std::io::{self, stdout, Write};
use std::process::{Command, Stdio, Child};
use std::path::Path;
fn main() {
    loop{
        eprint!("{} ~> ", env::current_dir().unwrap().display());
        stdout().flush();
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).unwrap();
        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_commands = None;
        

        while let Some(command) = commands.next(){
            
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command{
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    
                    if let Err(e) = env::set_current_dir(&root){
                        eprintln!("{}",e);
                    }
                    previous_commands= None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_commands
                        .map_or(
                            Stdio::inherit(),
                            |output: Child| Stdio::from(output.stdout.unwrap())
                        );
                    let stdout = if commands.peek().is_some(){
                        Stdio::piped()
                    }else{
                        Stdio::inherit()
                    };
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    
                    match output {
                        Ok(output) => {previous_commands = Some(output);},
                        Err(e) => {
                            previous_commands = None;
                            eprintln!("{}",e);
                        },
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_commands{
            final_command.wait();
        }
    }
}

