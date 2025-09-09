/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 * Edit: Benjamin Widman <bwidman@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // Get input lines as a vector of strings
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input.lines()
        .map(|_line| _line.ok().unwrap());
        //.collect::<Vec<String>>();          // Converts iterator into vector. Not necessary, see example solution.
    // The map acts on every element in the iterator, getting the value inside the returned Result, assuming the result is Ok(value) and not Err(error_message).
    // ok() returns an Option, so we unwrap it to get the value inside.

    /* add code here ... */
    
    let rawLength: usize = lines.next().unwrap().parse::<usize>().unwrap();
    let halfLength: usize = (rawLength+1)/2;

    let mut numbers = lines.next().unwrap().split(" ").map(|component| component.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    numbers.sort();

    
    let mut ans: i64 = 0;
    let mut iterator: usize = 1;
    while iterator <= halfLength{
        ans += numbers[rawLength-iterator];
        iterator+=1;
    }
    // while iterator < halfLength
    // sort lines[2].unwrap
    // for i in halfLength ans += numbers[-i]

    //eprintln!("Kattis skips this comment!");
    print!("length is: ");
    println!("{}", halfLength);
    print!("numbers are: ");
    println!("{:?}", numbers);
    print!("answer is: ");
    println!("{}", ans);
}

