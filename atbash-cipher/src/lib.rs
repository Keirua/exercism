/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut cipher = decode(plain);

    let mut out:String = String::new();
    for (i,c) in cipher.chars().enumerate(){
        out.push_str(&c.to_string());
        if i != 0 && (i+1)%5 == 0 && i < cipher.len()-1 {
            out.push_str(&String::from(" "));
        }
    }
    return out;
}

pub fn decode(plain: &str) -> String {
    let mut cipher:String = plain
        .to_lowercase()
        .chars()
        .map(|c| {
            match c {
                'a'...'z' => {
                    (b'z' - c as u8 + b'a') as char
                },
                '0'...'9' => { c }
                _ => ' '
            }
        })
        .collect()
        ;
    cipher = cipher.replace(" ", "");
    return cipher;
}
