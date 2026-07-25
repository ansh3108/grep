use std::env;
use std::fs;
use std::println;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Raw arguments: {:?}", args);

    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for: {}", query);
    println!("In file: {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should've been able to read the file!");

    println!("With text:\n{}", contents);
}
