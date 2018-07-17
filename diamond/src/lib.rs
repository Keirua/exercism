fn single_char_line(_c:char, nb_spaces:i8) -> String {
    " ".repeat(nb_spaces as usize).to_owned()
        + &_c.to_string().to_owned()
        + &" ".repeat(nb_spaces as usize).to_owned()
}

fn n_spaces(n:i8) -> String {
    if n > 0 {
        " ".repeat(n as usize)
    }
    else {
        String::from("")
    }
}

fn double_char_line(_c:char, offset:i8, nb_chars:i8) -> String {
    let nb_spaces_before = offset;
    let nb_spaces_middle = nb_chars - 2*(offset+1);

    let spaces_before = n_spaces(nb_spaces_before);
    let spaces_middle = n_spaces(nb_spaces_middle);

    format!("{}{}{}{}{}", 
        spaces_before,
        _c,
        spaces_middle,
        _c,
        spaces_before
        )
}

pub fn get_diamond(_c: char) -> Vec<String> {
    let mut lines:Vec<String> = Vec::new();
    if _c == 'A' {
        lines.push(String::from("A"));
    }
    else {
        let nb_spaces = _c as i8 - 'A' as i8;
        let nb_chars_per_line = 2*nb_spaces+1;
        
        lines.push(String::from(double_char_line(_c, 0, nb_chars_per_line)));
        for i in 1..nb_spaces {
            let curr_char = (_c as u8 - i as  u8) as char;
            let l = double_char_line(curr_char, i, nb_chars_per_line);
            lines.insert(0, l.clone());
            lines.push(l.clone());
        }
        lines.push(String::from(single_char_line('A', nb_spaces)));
        lines.insert(0, String::from(single_char_line('A', nb_spaces)));

    }

    lines
}
