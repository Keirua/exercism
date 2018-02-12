use std::collections::HashMap;

pub fn word_count(s: &str) ->  HashMap<String, u32> {
    let mut pairs:HashMap<String, u32> = HashMap::new();
    for w in s.split(" ") {
        let punctuation: &[_] = &[',','.',':','!','&','@','$','%','^','&'];
        let trimmed_word = w.trim_matches(punctuation).to_string().to_lowercase();
        if trimmed_word != "" {
            let entry = pairs.entry(trimmed_word).or_insert(0);
            *entry += 1;
        }
    }

    pairs
}