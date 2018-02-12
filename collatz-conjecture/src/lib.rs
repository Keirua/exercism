// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(mut n: u64) -> Result<u64, &'static str> {
    let mut nb_steps = 0;
    if n > 0 {
        while n != 1 {
            n = if n % 2 == 0 {
                n >> 1
            }
            else {
                3 * n +1
            };
            nb_steps += 1;
        }

        return Ok(nb_steps);
    }

    Err("")
}
