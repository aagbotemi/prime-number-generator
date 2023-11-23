fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(n as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let primes: Vec<usize> = (2..=n).filter(|&x| is_prime[x]).collect();
    primes
}

fn main() {
    println!("You want to generate prime numbers between 2 and n limit!");
    println!("Please enter limit number!");
    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("failed to read input.");
    let n: usize = number.trim().parse().expect("invalid input");
    let primes = sieve_of_eratosthenes(n);

    println!("Prime numbers between 1 and {}: {:?}", n, primes);
    println!(
        "The number of Prime numbers between 1 and {}: {:?}",
        n,
        primes.len()
    );
}
