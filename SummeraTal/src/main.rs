
use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut lines = input.lock().lines();
    
    let rawLength: usize = lines
        .next()
        .expect("Missing first line")
        .expect("Error reading first line")
        .trim()
        .parse()
        .expect("First line is not a valid number");
    let halfLength: usize = (rawLength+1)/2;

    let numbersLine = lines
        .next()
        .expect("Missing second line")
        .expect("Error reading second line");

    let mut numbers = numbersLine
        .split_whitespace()
        .map(|component| component.parse::<i64>().expect("Invalid number"))
        .collect::<Vec<i64>>();

    numbers.sort();
    
    let ans: i64 = numbers[rawLength - halfLength..].iter().sum();
    
    println!("{}", ans);
}

