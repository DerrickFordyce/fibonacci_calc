#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use std::io;
use std::io::Write;

fn main() {
    println!("Please enter the nth Fibonacci to calculate:");
    println!("The nth Fibonacci number is: {}", calc_fib(get_user_int()));
}

fn calc_fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut index = 2;

    if n == 0 {
        return 0;
    }

    while index <= n {
        let c = a + b;
        a = b;
        b = c;
        index += 1;
    }
    b
}

fn get_user_int() -> i32 {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let _input: i32 = if let Ok(num) = input.trim().parse() {
            return num;
        } else {
            println!("Please enter a number!");
            continue;
        };
    }
}
