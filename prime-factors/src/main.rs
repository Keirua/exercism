extern crate prime_factors;

use prime_factors::factors;

fn main() {
    let f = factors(901255);
    println!("{:?}", f);
}