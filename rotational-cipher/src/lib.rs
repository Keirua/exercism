pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| {
        match c {
            'a'...'z' => (((c as u8 + key as u8 - b'a') % 26) + b'a') as char,
            'A'...'Z' => (((c as u8 + key as u8 - b'A') % 26) + b'A') as char,
            _ => c
        }
    })
    .collect()
}
