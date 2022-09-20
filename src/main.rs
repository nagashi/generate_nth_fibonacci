use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use ordinal::Ordinal;
use std::{io, mem::replace};

// Calculate large fibonacci numbers.
fn fibonacci(n: u128) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();

    for _ in 0..n {
        let f2 = f0 + &f1;
        // Value change: f0 <- f1 & f1 <- f2
        f0 = replace(&mut f1, f2);
    }
    f0
}

// generic for BigUint
fn formatted<T>(input: T) -> String
where
    T: ToBigUint,
    T: std::fmt::Debug,
{
    let format_string = format!("{:?}", input);
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

        let suffix = Ordinal(num) // get ordinal suffix, i.e., st, nd, rd, th
            .to_string()
            .chars()
            .rev()
            .take(2)
            .collect::<String>();

        println!(
            "The {fib} Fibonacci number is: \n{}",
            formatted(fibonacci(num)),
            fib = format!(
                "{}{}",
                formatted(num), // format if > 999
                suffix.chars().rev().collect::<String>()
            )
        );
    }
}
