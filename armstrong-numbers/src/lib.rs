
pub fn extract_digits(num: u32) -> Vec<u32> {
    if num == 0 { return vec!(0); }
    let mut n = num;
    let mut digits = Vec::new();
    while n > 0 {
        let current_digit = n % 10;
        digits.push(current_digit);
        n /= 10;
    }
    digits
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = extract_digits(num);

    /*let mut sum = 0;
    for i in 0..digits.len() {
        sum += digits[i].pow((digits.len()) as u32);
    }*/
    let number_of_digits = digits.len();
    let sum:u32 = digits
        .iter()
        .fold(0, |sum:u32, x:&u32| { sum + (x.pow(number_of_digits as u32) )});

    sum == num
}
