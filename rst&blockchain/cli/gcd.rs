// Function to calculate the Greatest Common Divisor (GCD) of two integers
fn gcd(mut m: i32, mut n: i32) -> i32 {
    // Ensure that neither `m` nor `n` is zero
    assert!(n != 0 && m != 0);

    // Loop until `m` becomes zero
    while m != 0 {
        // If `m` is smaller than `n`, swap their values
        if m < n {
            let t = m; // Temporary variable to hold the value of `m`
            m = n; // Assign `n` to `m`
            n = t; // Assign the original value of `m` (stored in `t`) to `n`
        }

        // Update `m` to be the remainder of `m / n`
        m %= n;
    }

    // When `m` becomes zero, `n` is the GCD
    n
}

// Test function to verify the correctness of the `gcd` function
#[test]
fn test_gcd() {
    // Test case: GCD of 14 and 15 should be 1
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 19 * 23), 3 * 11);
}

use std::env;
use std::str::FromStr;

// Main function to demonstrate the usage of the `gcd` function
fn main() {
    // dynamic array
    let mut numbers = Vec::new();

    let args = env::args().skip(1);

    for arg in args {
        let result = i32::from_str(&arg);
        match result {
            Ok(num) => {
                numbers.push(num);
            }
            Err(e) => {
                eprintln!("error parsing argument: {}", e);
                std::process::exit(1);
            }
        }
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        // skip the first element
        // reference and dereference
        // take every element in the array and make it as mutable reference
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
