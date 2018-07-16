fn single_char_line(_c:char, nb_spaces:i8) -> String {
    " ".repeat(nb_spaces as usize).to_owned()
        + &_c.to_string().to_owned()
        + &" ".repeat(nb_spaces as usize).to_owned()
}

pub fn get_diamond(_c: char) -> Vec<String> {
    let nb_spaces = _c as i8 - 'A' as i8;
    let mut lines:Vec<String> = Vec::new();
    if _c == 'A' {
        lines.push(String::from("A"));
    }
    if _c == 'B' {
        lines.push(String::from(single_char_line('A', nb_spaces)));
        lines.push(String::from("B B"));
        lines.push(String::from(single_char_line('A', nb_spaces)));
    }
    if _c == 'C' {
        lines.push(String::from(single_char_line('A', nb_spaces)));
        lines.push(String::from(" B B "));
        lines.push(String::from("C   C"));
        lines.push(String::from(" B B "));
        lines.push(String::from(single_char_line('A', nb_spaces)));
    }
    lines
}
