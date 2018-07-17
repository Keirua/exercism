fn get_letter_index(c: char) -> Option<u8> {
    if c >= 'a' && c <= 'z' {
        Some(c as u8 - 'a' as u8)
    } else if c >= 'A' && c <= 'Z' {
        Some(c as u8 - 'A' as  u8)
    } else {
        None
    }
}

pub fn check(candidate: &str) -> bool {
    let mut char_counts : [i32; 26] = [0;26];

    for c in candidate.chars() {
        let letter = get_letter_index(c);
        match letter {
            Some(index) => {
                char_counts[index as usize] += 1;
                if char_counts[index as usize] > 1 {
                    return false;
                }
            }
            None => {}
        }
    }

    true
}
