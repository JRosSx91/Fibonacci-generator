use std::io;

fn main() {
    let mut n: String = String::new();
    let stdin = io::stdin();

    println!("How many n-numbers do you want to see?");

    stdin.read_line(&mut n).expect("Failed to read line");

    let n: u128 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    for i in 0..n {
        println!("{}", fibonacci(i));
    }
}
fn fibonacci(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
