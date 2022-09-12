use std::{collections::HashMap, convert::TryInto};

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m = HashMap::<i32, Vec<usize>>::new();

    for (i, n) in nums.iter().enumerate() {
        m.entry(*n)
            .and_modify(|l| {
                l.push(i);
            })
            .or_insert(vec![i]);
    }

    for (i, n) in nums.iter().enumerate() {
        if let Some(entries) = m.get(&(target - *n)) {
            if let Some(other) = entries.iter().filter(|&j| i != *j).take(1).next() {
                return vec![i.try_into().unwrap(), (*other).try_into().unwrap()];
            }
        }
    }

    vec![]
}

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
}
