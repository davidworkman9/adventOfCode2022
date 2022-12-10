use std::{hash::Hash, collections::HashSet};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}


fn main() {
    let str = include_str!("../input.txt");

    let mut pt_one_idx = str.len();
    let mut pt_two_idx = str.len();
    for (i, _c) in str.chars().enumerate() {
        if i < 3 {
            continue;
        }
        {
            let slice = &str[i-3..i+1];
            let has = has_unique_elements(slice.chars());
            if has && pt_one_idx == str.len() {
                pt_one_idx = i;
            }
        }

        if i < 13 {
            continue;
        }

        {
            let slice = &str[i-13..i+1];
            let has = has_unique_elements(slice.chars());
            if has {
                pt_two_idx = i;
                break;
            }
        }
    }

    println!("Part 1 {}", pt_one_idx+1);
    println!("Part 2 {}", pt_two_idx+1);
}
