use std::{print, println};
use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("You are in a dark room");
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        print!("You typed {}\n", input.trim().to_lowercase());
    }
}
