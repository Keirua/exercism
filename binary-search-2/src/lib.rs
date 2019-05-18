use std::cmp::Ordering::*;
use std::cmp::Ord;

pub fn find<T>(list:&[T], v:T) -> Option<usize>
    where T: Ord
{
    if list.len() != 0 {
        let mut left = 0;
        if v >= list[left] {
            let mut right = list.len() -1;
            while left <= right {
                let middle = (right + left)/2;

                match list[middle].cmp(&v) {
                    Less => left = middle+1,
                    Greater => right = middle-1,
                    Equal => return Some(middle)
                };
            }
        }
    }

    return None;
}