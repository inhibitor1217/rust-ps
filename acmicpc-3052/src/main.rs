use std::iter::FromIterator;
use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let nums = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let unique = HashSet::<u32>::from_iter(nums.iter().map(|x| x % 42));
    println!("{}", unique.len());
}
