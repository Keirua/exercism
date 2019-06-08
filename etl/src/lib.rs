use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output: BTreeMap<char, i32> = BTreeMap::new();
    for (score, letters) in h.iter() {
        for letter in letters.iter() {
            let lc_letter = letter.to_ascii_lowercase();
            output.insert(lc_letter, *score);
        }
    }
    output
}