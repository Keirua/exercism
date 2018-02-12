# Readme

## accumulate


2 ways to do this:

 - by iterating through all the values, applying f on the way
 - by calling map directly

It's worth noting that in order to do map, the data needs to be Copy.
Also note the 2 different ways to annotate the traits that have types T and F.


```rust
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
```

## Collatz

Computation of the collatz formula. Note the

```rust
pub fn collatz(mut n: u64) -> Result<u64, &'static str> {
    let mut nb_steps = 0;
    if n > 0 {
        while n != 1 {
            n = if n % 2 == 0 {
                n >> 1
            }
            else {
                3 * n +1
            };
            nb_steps += 1;
        }

        return Ok(nb_steps);
    }

    Err("")
}
```

## Sum of squares, square of sum

Computing the sum of squares and the square of the sum of the numbers between 0 and n was a good opportunity to play with fold

```rust
pub fn square_of_sum(n: usize) -> usize {
    let sum = (0..n+1).fold(0, |acc, val| { acc+val });
    sum * sum
}

pub fn sum_of_squares(n: usize) -> usize {
	(0..n+1).fold(0, |acc, val| { acc+(val*val) })
}
```

## String reversal

My initial naive implementation was pretty simple: looping through all the chars in reverse. Something was wrong, but I couldn't see how to do better.

```rust
pub fn reverse(s: &str) -> String {
    let mut s2:String = String::new();
    for c in s.chars().rev() {
        s2.push(c);
    }
    
    s2
}
```

... I forgot `collect` would do that right away...

```rust
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
```


## Factors

### First draft

use std::cmp::Ordering::{Less, Equal, Greater};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

fn factors(n:u64) -> Vec<u64> {
    let mut f:Vec<u64> = Vec::new();
    for i in 1..n/2+1 {
        if n % i == 0 {
            f.push(i);
        }
    }
    f
}

pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num == 0 {
        return Err("Number must be positive");
    }

    let f = factors(num);
    let sum = f.into_iter().fold(0, |x, acc| acc + x);

    match sum.cmp(&num) {
        Equal => Ok(Classification::Perfect),
        Greater => Ok(Classification::Abundant),
        Less => Ok(Classification::Deficient)
    }
}

### Second draft

We can write the factors method using for_each:

fn factors(n:u64) -> Vec<u64> {
    let mut f:Vec<u64> = Vec::new();
    (1..n/2+1).for_each(|x| { if n % x == 0 {
            f.push(x);
        }
    });
    f
}

### Third draft

No need to compute the factors separately: we can map and sum right away

pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num == 0 {
        return Err("Number must be positive");
    }

    let sum:u64 = (1..num/2+1)
        .map(|x| if num%x == 0 { x } else {0})
        .sum();

    match sum.cmp(&num) {
        Equal => Ok(Classification::Perfect),
        Greater => Ok(Classification::Abundant),
        Less => Ok(Classification::Deficient)
    }
}

### Fourth draft

Let's compute the sum right away using for_each:

pub fn classify(num: u64) -> Result<Classification, & 'static str> {
    if num == 0 {
        return Err("Number must be positive");
    }

    let mut sum:u64 = 0;
    (1..num/2+1)
        .for_each(|x| if num%x == 0 { sum += x });

    match sum.cmp(&num) {
        Equal => Ok(Classification::Perfect),
        Greater => Ok(Classification::Abundant),
        Less => Ok(Classification::Deficient)
    }
}


# Word count

Finding all the words in a text (case insensitively, without punctuation) is a great opportunity to play with HashMaps:

```rust
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
```