pub fn raindrops(n: usize) -> String {
    if (n % 3 == 0) || n % 5 == 0 || n % 7 == 0 {
        let mut out = String::new();
        if n % 3 == 0 {
            out += "Pling";
        }
        if n % 5 == 0 {
            out += "Plang";
        }
        if n % 7 == 0 {
            out += "Plong";
        }
        out
    }
    else {
        n.to_string()
    }
}
