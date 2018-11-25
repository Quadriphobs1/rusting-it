extern crate primal;

use primal::Sieve;

fn main() {
    let seive = Sieve::new(10000);
    let suspect: usize = 5273;
    println!("{} is prime: {}", suspect, seive.is_prime(suspect));
    let not_a_prime = 1024;
    println!("{} is prime: {}", not_a_prime, seive.is_prime(not_a_prime));

    let n: usize = 1000;

    match seive.primes_from(0).nth(n - 1) {
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("I don't know anything about {}th prime.", n),
    }

    println!("{:?}", seive.factor(21));
}
