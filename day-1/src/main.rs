use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split = contents.lines();

    let sum = split.fold(0.0, |sum, s| sum + (s.parse::<f64>().unwrap()/3.0).floor()-2.0);

    println!("number {}", sum);
}
