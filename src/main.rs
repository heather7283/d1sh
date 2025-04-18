mod tokenizer;
mod interpreter;

use std::{env, io};
use std::io::Write;
use gethostname::gethostname;

fn main() {
    loop {
        print!("[{}@{}] {} d1sh> ",
               env::var("USER").unwrap(),
               gethostname().to_str().unwrap(),
               env::current_dir().unwrap().display());

        io::stdout().flush().unwrap();
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input.");
            continue;
        }

        let input = input.trim(); // remove \n char

        interpreter::interpret(tokenizer::tokenize(input));
    }
}
