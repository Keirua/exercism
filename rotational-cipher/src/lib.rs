pub fn rotate(input: &str, key: i8) -> String {
    let mut output: String = String::new();

    for c in input.chars() {
        if c >= 'a' && c <= 'z' {
            let c2 = (((c as u8 + key as u8 - 'a' as u8) % 26) + 'a' as u8) as char;
            output.push_str(&c2.to_string());
        } else if c >= 'A' && c <= 'Z' {
            let c2 = (((c as u8 + key as u8 - 'A' as u8) % 26) + 'A' as u8) as char;
            output.push_str(&c2.to_string());
        } else {
            output.push_str(&c.to_string());
        }
    }

    output
}
