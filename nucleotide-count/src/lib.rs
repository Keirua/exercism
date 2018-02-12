use std::collections::HashMap;

pub fn count(c:char, s:&str) -> Result<usize, &str> {
    match c {
        'G' | 'C' | 'T' | 'A' => match nucleotide_counts(s) {
            Ok(mut map) => Ok(*map.entry(c).or_insert(0)),
            Err(e) => Err(e)
        },
        _ => Err("")
    }
}

pub fn nucleotide_counts(s:&str) -> Result<HashMap<char, usize>, &str> {
    let mut map = "ACGT".chars()
                    .map(|c| (c, 0 as usize))
                    .collect::<HashMap<_, _>>();
    
    for c in s.chars() {
        if c == 'G' || c == 'C'  || c == 'T' || c == 'A' {
            let mut entry = map.entry(c).or_insert(0);
            *entry += 1;
        }
        else {
            return Err("");
        }
    }
    Ok(map)
}

