use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num == 0 {
        return Err("Number must be positive");
    }

    let mut sum:u64 = 0;
    (1..num/2+1)
        .for_each(|x| if num%x == 0 { sum += x });

    match sum.cmp(&num) {
        Equal => Ok(Classification::Perfect),
        Greater => Ok(Classification::Abundant),
        Less => Ok(Classification::Deficient)
    }
}
