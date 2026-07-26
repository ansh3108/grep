use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Raw arguments: {:?}", args);

    let config = Config::build(&args).expect("Problem parsing arguments");
    
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should've been able to read the file!");

    println!("With text:\n{}", contents);

    
}


struct Config {
        query: String,
        file_path: String,
    }

    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            } else {
                Ok(Config {
                    query: args[1].clone(),
                    file_path: args[2].clone(),
                })
            }

        }
    }
