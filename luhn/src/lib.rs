pub fn is_valid(s: &str) -> bool {
    let s = s.replace(" ", "");
    let mut sum = 0;
    let parity = s.len()% 2;

    for (index, c) in s.chars().enumerate() {
        if !c.is_numeric() {
            return false;
        }

        let curr_value = c as i32 - '0' as i32;
        sum += if (index) % 2 == parity {
            curr_value
        } else {
            let mut new_value = 2 * curr_value;
            if new_value > 10 {
                new_value -= 9
            }
            new_value
        };
    }
    return sum % 10 == 0;
}
