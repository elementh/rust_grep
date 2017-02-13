use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
//    println!("With text:\n{}", content);
    let results = if config.case_sensitive {
        grep(&config.search, &content)
    } else {
        grep_case_insensitive(&config.search, &content)
    };
    for line in results{
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub search: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let search = args[1].clone();
        let filename = args[2].clone();
        
        let mut case_sensitive = true;
        
        for (name, _) in env::vars() {
            if name == "CASE_INSENSITIVE" {
                case_sensitive = false;
            }
        }

        Ok(Config {
            search: search,
            filename: filename,
            case_sensitive: case_sensitive
        })
    }
}

fn grep<'a>(search: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }

    results
}

fn grep_case_insensitive<'a>(search: &str, content: &'a str) -> Vec<&'a str> {
    let search = search.to_lowercase();
    let mut results = Vec::new();
    
    for line in content.lines() {
        if line.to_lowercase().contains(&search) {
            results.push(line);
        }
    }
    
    results
}

#[cfg(test)]
mod test {
    use grep;
    use grep_case_insensitive;

    #[test]
    fn case_sensitive() {
        let search = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
        vec!["safe, fast, productive."],
        grep(search, content)
        )
    }
    #[test]
    fn case_insensitive() {
        let search = "rust";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            grep_case_insensitive(search, content)
        )
    }
}
