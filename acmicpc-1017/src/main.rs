use std::io::{self, Read};

use input::TestCase;
use solution::solve;

mod input {
    #[derive(Debug)]
    pub struct TestCase {
        pub nums: Vec<u32>,
    }

    impl TestCase {
        pub fn build(input: &String) -> TestCase {
            let mut input = input.lines();
            input.next();
            let nums = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            TestCase { nums }
        }
    }
}

mod solution {
    use std::{
        collections::{HashMap, HashSet},
        fmt::Display,
    };

    use crate::input::TestCase;

    #[derive(Debug, PartialEq)]
    pub struct Solution(pub Option<Vec<u32>>);

    impl Display for Solution {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match &self.0 {
                Some(v) => write!(
                    f,
                    "{}",
                    v.iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
                None => write!(f, "-1"),
            }
        }
    }

    #[derive(Debug)]
    pub struct Graph {
        pub adj: Vec<Vec<bool>>,
    }

    impl Graph {
        fn build(nums: &[u32], primes: &HashSet<u32>) -> Option<Graph> {
            let evens = nums.iter().filter(|n| *n % 2 == 0).collect::<Vec<_>>();
            let odds = nums.iter().filter(|n| *n % 2 == 1).collect::<Vec<_>>();

            if evens.len() != odds.len() {
                return None;
            }

            let mut adj = vec![];

            for e in evens.iter() {
                let mut adj_row = vec![];
                for o in odds.iter() {
                    adj_row.push(primes.contains(&(*e + *o)));
                }
                adj.push(adj_row);
            }

            Some(Graph { adj })
        }

        fn size(&self) -> usize {
            self.adj.len()
        }

        fn has_edge(&self, i: usize, j: usize) -> bool {
            self.adj[i][j]
        }

        fn bipartite_match(&self) -> usize {
            let mut match_count = 0;
            let mut matches = HashMap::new();
            for i in 0..self.size() {
                if self.bipartite_match_rec(i, &mut HashSet::new(), &mut matches) {
                    match_count += 1;
                }
            }
            match_count
        }

        fn bipartite_match_rec(
            &self,
            cur: usize,
            visited: &mut HashSet<usize>,
            matches: &mut HashMap<usize, usize>,
        ) -> bool {
            for v in 0..self.size() {
                if self.has_edge(cur, v) && !visited.contains(&v) {
                    visited.insert(v);
                    if !matches.contains_key(&v)
                        || self.bipartite_match_rec(*matches.get(&v).unwrap(), visited, matches)
                    {
                        matches.insert(v, cur);
                        return true;
                    }
                }
            }

            false
        }
    }

    pub fn solve(case: &TestCase) -> Solution {
        let primes = primes();
        let TestCase { nums } = case;

        let mut candidates = vec![];
        for (i, num) in nums[1..].iter().enumerate() {
            if !primes.contains(&(nums[0] + num)) {
                continue;
            }
            let rest = [&nums[1..i + 1], &nums[i + 2..]].concat();
            if let Some(graph) = Graph::build(&rest, &primes) {
                if graph.bipartite_match() == graph.size() {
                    candidates.push(*num);
                }
            }
        }

        candidates.sort();
        if candidates.len() == 0 {
            Solution(None)
        } else {
            Solution(Some(candidates))
        }
    }

    fn primes() -> HashSet<u32> {
        let mut primes = HashSet::new();

        'outer: for i in 2..2000 {
            for p in &primes {
                if i % p == 0 {
                    continue 'outer;
                }
            }
            primes.insert(i);
        }

        primes
    }
}

#[cfg(test)]
mod test {
    use crate::{
        input::TestCase,
        solution::{solve, Solution},
    };

    #[test]
    fn case_0() {
        assert_eq!(
            solve(&TestCase {
                nums: vec![1, 4, 7, 10, 11, 12],
            }),
            Solution(Some(vec![4, 10]))
        )
    }

    #[test]
    fn case_1() {
        assert_eq!(
            solve(&TestCase {
                nums: vec![11, 1, 4, 7, 10, 12]
            }),
            Solution(Some(vec![12]))
        )
    }

    #[test]
    fn case_2() {
        assert_eq!(
            solve(&TestCase {
                nums: vec![8, 9, 1, 14]
            }),
            Solution(None)
        )
    }

    #[test]
    fn case_3() {
        assert_eq!(
            solve(&TestCase {
                nums: vec![34, 39, 32, 4, 9, 35, 14, 17]
            }),
            Solution(Some(vec![9, 39]))
        )
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let case = TestCase::build(&input);

    println!("{}", solve(&case));
}
