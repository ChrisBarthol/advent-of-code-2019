use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let split = contents.lines();

    let sum = split.fold(0.0, |sum, s| sum + recursive_fuel_needed_for(s.parse::<f64>().unwrap()));

    println!("number {}", sum);
}


fn fuel_needed_for(item: f64) -> f64 {
    (item/3.0).floor()-2.0
}

fn recursive_fuel_needed_for(item: f64) -> f64 {
    let mut v: Vec<f64> = Vec::new();
    let mut fuel = fuel_needed_for(item);
    v.push(fuel);
    while fuel >= 0.0 {
        fuel = fuel_needed_for(fuel);
        if fuel >= 0.0 {
            v.push(fuel);
        }
    }
    let sum: f64 = v.iter().sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2.0, fuel_needed_for(14.0));
    }
    #[test]
    fn recursive_fuel_for_14() {
        assert_eq!(2.0, recursive_fuel_needed_for(14.0));
    }
    #[test]
    fn recursive_fuel_for_1969() {
        assert_eq!(966.0, recursive_fuel_needed_for(1969.0));
    }
    #[test]
    fn recursive_fuel_for_100756() {
        assert_eq!(50346.0, recursive_fuel_needed_for(100756.0));
    }
}
