use std::env;
use std::fs;
use std::process;

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


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Raw arguments: {:?}", args);

    let config = Config::build(&args).expect("Problem parsing arguments");
    
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }    
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines()  {
        if line.contains(query) {
            results.push(line);
        }
    }   

    results
}
