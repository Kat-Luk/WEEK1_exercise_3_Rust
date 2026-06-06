use std::io;
use std::convert::Into;
fn main() {
    println!("By how much do you want to increment the number?");
    let mut integer: i32 = 0;
    while integer < i16::MAX.into() {
        let mut input = String::new();
        println!("Current: {}. Increment by: ", integer);
        io::stdin().read_line(&mut input).expect("Error reading input");
        let input: i32 = input.trim().parse().expect("Input is not a valid integer");
        integer += input;
    } println!("Enough incrementations.")
}
