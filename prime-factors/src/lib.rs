pub fn factors(number: i32) -> Vec<i32> {
	let mut primes:Vec<i32> = Vec::new();
	let mut prime_factors:Vec<i32> = Vec::new();

	let sqrt_n = (number as f32).sqrt() as i32;
	// List the prime numbers bet
	for i in 2..(sqrt_n as i32 +1) {
		primes[i as usize] = i;
	}
/*
	for i in 2..(sqrt_n+1) {
		if number % i != 0 {
			primes[i as usize] = 0;
		}
	}
	// Find the prime factors
	for p in primes.into_iter().rev() {
		if number % p == 0 {
			prime_factors.push(p);
		}
	}*/

	prime_factors
}