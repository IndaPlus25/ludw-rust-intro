use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let length: usize = lines.next()
        .expect("Missing first line")
        .expect("Error reading first line")
        .parse()
        .expect("First line is not a valid number");

    if length == 0 {
        println!("0");
        return;
    }

    let mut forenames = Vec::with_capacity(length);
    for _ in 0..length {
        let line = lines.next()
            .expect("Missing line")
            .expect("Error reading line");
        forenames.push(line);
    }

    let mut names = Vec::with_capacity(length);
    for i in 0..length {
        let surname = lines.next()
            .expect("Missing line")
            .expect("Error reading line");
        names.push(format!("{} {}", forenames[i], surname));
    }

    names.sort();

    names.dedup();

    println!("{}", names.len());
}

