use num_bigint::BigUint;
use num_traits::{One, Zero};
use ordinal::Ordinal;
use std::io;
use std::mem::replace;

// Calculate large fibonacci numbers.
fn fib(n: u128) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..n {
        let f2 = f0 + &f1;
        // Value change: f0 <- f1 & f1 <- f2
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn format_biguint(n: BigUint) -> String {
    let format_string = format!("{}", n);
    let mut x = String::new();
    let mut z = format_string.chars().rev().peekable();

    while z.peek().is_some() {
        let chunk: String = z.by_ref().take(3).collect();
        x.push_str(&chunk);
        if z.peek().is_some() {
            x.push(',');
        }
    }
    x.chars().rev().collect()
}

fn main() {
    loop {
        let mut str_input = String::new();

        println!("\nType \"quit\" to end the program or");

        println!("Enter a positive integer >= 0");

        io::stdin()
            .read_line(&mut str_input)
            .expect("Failed to reade line");

        if str_input.trim().to_string().to_lowercase() == "quit" {
            break;
        }

        let num: u128 = match str_input.trim().parse::<u128>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!(
            "The {fib} Fibonacci number is: \n{}",
            format_biguint(fib(num)),
            fib = Ordinal(num).to_string()
        );
    }
}
