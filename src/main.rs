fn sieve_of_eratosthenes(start_num: usize, limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(limit as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    // Handle upper bound cases
    for i in (limit as f64).sqrt() as usize + 1..=limit {
        if is_prime[i] {
            let mut j = 2 * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let primes: Vec<usize> = (start_num..=limit).filter(|&x| is_prime[x]).collect();
    primes
}

fn main() {
    println!("You want to generate prime numbers between 2 and n limit!");
    println!("Please enter starting number!");
    let mut start_num = String::new();
    std::io::stdin()
        .read_line(&mut start_num)
        .expect("failed to read input.");
    let start_num: usize = start_num.trim().parse().expect("invalid input");
    println!("Please enter limit number!");
    let mut limit = String::new();
    std::io::stdin()
        .read_line(&mut limit)
        .expect("failed to read input.");
    let limit: usize = limit.trim().parse().expect("invalid input");
    let primes = sieve_of_eratosthenes(start_num, limit);

    println!(
        "Prime numbers between {} and {}: {:?}",
        start_num, limit, primes
    );
    println!(
        "The number of Prime numbers between {} and {}: {:?}",
        start_num,
        limit,
        primes.len()
    );
}
