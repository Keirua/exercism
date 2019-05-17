pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '('|'['|'{' => stack.push(c),
            ')'|']'|'}' => {
                match (stack.pop(), c) {
                    (Some('('), ')') => {},
                    (Some('['), ']') => {},
                    (Some('{'), '}') => {},
                    _ => return false
                };
            },
            _ => {}
        }
    }

    stack.len() == 0
}
