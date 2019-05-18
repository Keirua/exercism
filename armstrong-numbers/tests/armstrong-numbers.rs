use armstrong_numbers::*;

#[test]
fn test_extract_digits() {
    assert_eq!(extract_digits(0), vec![0]);
    assert_eq!(extract_digits(9), vec![9]);
    assert_eq!(extract_digits(10), vec![0,1]);
    assert_eq!(extract_digits(123), vec![3,2,1]);
    assert_eq!(extract_digits(1234), vec![4,3,2,1]);
}


#[test]
fn test_single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
#[ignore]
fn test_there_are_no_2_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
#[ignore]
fn test_three_digit_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
#[ignore]
fn test_three_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
#[ignore]
fn test_four_digit_armstrong_number() {
    assert!(is_armstrong_number(9474))
}

#[test]
#[ignore]
fn test_four_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9475))
}

#[test]
#[ignore]
fn test_seven_digit_armstrong_number() {
    assert!(is_armstrong_number(9926315))
}

#[test]
#[ignore]
fn test_seven_digit_non_armstrong_number() {
    assert!(!is_armstrong_number(9926316))
}
