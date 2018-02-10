pub fn build_proverb(list: Vec<&str>) -> String {
    let mut out = String::new();
    if list.len() > 0 {
        for i in 0..list.len()-1 {
            out += &format!("For want of a {} the {} was lost.\n", list[i], list[i+1]);
        }
        out += &format!("And all for the want of a {}.", list[0]);
    }
    out
}
