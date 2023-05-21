use plotters::prelude::*;
use std::io;
use std::ops::Div;

fn main() {
    let mut n: String = String::new();
    let stdin = io::stdin();

    println!("How many n-numbers do you want to see?"); // El valor máximo sin desbordamiento es de n = 186

    stdin.read_line(&mut n).expect("Failed to read line");

    let n: u128 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    for i in 0..=n {
        match fibonacci(i) {
            Some(value) => println!("{}", value),
            None => {
                println!("Fibonacci's value for n = {} is too big😥.", i);
                break;
            }
        }
    }
}
fn fibonacci(n: u128) -> Option<u128> {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp.checked_add(b)?;
    }
    Some(a)
}
