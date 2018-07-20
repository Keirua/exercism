pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut numbers:Vec<u64> = (0..2.max(upper_bound+1))
        .map(|i|{ i })
        .collect();
    numbers[0] = 0;
    numbers[1] = 0;
            
    for i in 2..((upper_bound as f64).sqrt().floor() as u64 + 1) {
        let mut j = 2;
        while i*j <= upper_bound {
            numbers[(i*j) as usize] = 0;
            j+=1
        }
    }
    numbers
        .into_iter()
        .filter(|v| *v != 0)
        .collect()
}
