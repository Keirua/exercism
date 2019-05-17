pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors:Vec<u64> = Vec::new();
    let mut primes = vec![]; // sieve of primes below n
    let mut n = n;

    // initialize sieve
    let n_sqrt = (n as f64).sqrt().floor() as u64;
    for i in 0..(n_sqrt+1) {
        primes.push(i);
    }

    let mut current_prime = 2u64;
    while n > 1 {
        if n % current_prime == 0 {
            prime_factors.push(current_prime);
            n = n / current_prime;
        }
        else {
            let mut k = 2;
            let mut index = k*current_prime;
            // discard non primes: all the multiples of the current prime (2)
            while index < n_sqrt{
                primes[index as usize] = 0;
                k += 1;
                index = k*current_prime;
            }
            // go to the next prime
            current_prime += 1;
            while current_prime < n_sqrt && primes[current_prime as usize] == 0 {
                current_prime += 1;
            }
        }
    }

    prime_factors
}
