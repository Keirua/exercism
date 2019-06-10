/*
// a first version, using loops only
pub fn abbreviate(phrase: &str) -> String {
    let mut prev:Option<char> = None;
    let mut s = String::new();
    for c in phrase.chars() {
        match prev {
            Some(prev_char) => {
                if (prev_char.is_lowercase() && c.is_uppercase()) || prev_char == ' ' || prev_char == '-' {
                    s.push(c.to_ascii_uppercase())
                }
            },
            None => s.push(c.to_ascii_uppercase())
        }
        prev = Some(c)
    }
    s
}
*/

// a second version, using a functional approach
pub fn abbreviate(phrase: &str) -> String {
    let phrase = String::from(" ") + phrase;
    phrase
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .filter(|pair:&&[char]| {
            (pair[0].is_whitespace() || pair[0] == '-')
            || (pair[0].is_lowercase() && pair[1].is_uppercase())
        })
        .map(|pair:&[char]| pair[1])
        .collect::<String>()
        .to_uppercase()
}