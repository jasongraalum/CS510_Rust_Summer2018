/// Copyright (c) 2018 Jason Graalum
///
/// CS510 Rust Programming
/// Summer 2018
///
/// HW #3
/// Generate primes using Sieve of Eratosthenes
///
use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // Use a HashSet to track eliminated numbers - is_not_prime
    let mut is_not_prime: HashSet<u64> = HashSet::new();

    // Store primes in a Vec for function return value
    let mut result: Vec<u64> = Vec::new();

    // Starting at 2, next is the next value not in is_not_prime
    let mut next: u64 = 2;

    // While loop to add values to the is_not_prime HashSet
    while next <= upper_bound {
        result.push(next);
        (next..upper_bound)
            .take_while(|n| n * next <= upper_bound)
            .for_each(|n| {
                let _ = is_not_prime.insert(n * next);
            });
        next += 1;

        // Increment up next until a value is NOT found in is_not_prime
        while is_not_prime.contains(&next) {
            next += 1;
        }
    }

    // Report result
    result
}
