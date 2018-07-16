fn is_multiple_of_any(i : u32, factors: &[u32]) -> bool {
    for f in factors.iter() {
        if i % f == 0 {
            return true;
        }
    }
    return false;
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit).fold(0, |total, value|{
        if is_multiple_of_any(value, factors) {
            total + value
        } else {
            total
        }
    })
}
