use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    //    let search = args.get(1);
    //    let filename = args.get(2);
    let search = &args[1];
    let filename = &args[2];

    println!("Searching for {} in file {}.", search.trim(), filename.trim());

    let mut file = File::open(filename).expect("File not found!");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Something went wrong reading the file!");
    println!("With text:\n{}", content);

}
