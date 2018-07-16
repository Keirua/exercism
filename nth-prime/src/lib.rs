// check if n is prime using an improved erathostene"""
fn is_prime(n: u32) -> bool {
    if n == 1 {
        return false;
    }
    else if n < 4 {
        return true;
    }
    else if n % 2 == 0 {
        return false;
    }
    else if n < 9 {
        return true; // We have already excluded 4,6,8
    }
    else if n%3 == 0 {
        return false;
    }
    let r = (n as f32).sqrt().floor() as u32;
    let mut f = 5;
    while f <= r {
        if n % f == 0 || n % (f+2) == 0 {
            return false;
        }
        f += 6
    }

    return true;
}

pub fn nth(n: u32) -> Option<u32> {
    let mut n = n;
    let mut i = 0;
    if n == 0 {
        return None;
    }
    while n > 0 {
        i += 1;
        if is_prime(i) {
            n -= 1;
        }
    }
    Some(i)
}
