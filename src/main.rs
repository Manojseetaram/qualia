use std::{env, fs, process::exit};

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    } /*    run(contents); */
}
fn run(contents: &str) -> Result<(), String> {
    return Err("Not implemented".to_string());
}
fn run_prompt() {}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage : jlox [script]");
        exit(64);
    } else if args.len() == 1 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error : \n{}", msg);
            }
        }
    } else {
        run_prompt();
    }

    dbg!(args);
}
