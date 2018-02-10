pub fn map_function<T, F> (v:Vec<T>, f:F) -> Vec<T> 
    where F: (Fn(T) -> T)
{
    let mut out:Vec<T> = Vec::new();
    for value in v.into_iter() {
        out.push(f(value));
    }
    out
}

pub fn map_closure<T: Copy, F: Fn(T) -> T>(data: Vec<T>, f: F) -> Vec<T> {
    data.into_iter().map(f).collect()
}
