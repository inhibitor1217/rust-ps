use std::io::{self, Read};

use input::TestCase;
use solution::solve;

mod input {
    #[derive(Debug)]
    pub struct TestCase {
        pub min: u64,
        pub max: u64,
    }

    #[derive(Debug)]
    pub struct TestCaseParseError;

    impl TestCase {
        pub fn from(input: &String) -> Result<TestCase, TestCaseParseError> {
            if let [min, max] = input.trim().split_whitespace().collect::<Vec<_>>()[..] {
                let min: u64 = min.parse().map_err(|_| TestCaseParseError)?;
                let max: u64 = max.parse().map_err(|_| TestCaseParseError)?;
                Ok(TestCase { min, max })
            } else {
                Err(TestCaseParseError)
            }
        }
    }
}

mod solution {
    use crate::input::TestCase;

    pub fn solve(case: &TestCase) -> u64 {
        let TestCase { min, max } = case;
        let ps = primes(*max);
        let mut marks = [false; 1_000_001];

        for p in &ps {
            let p = p * p;
            let mut start = ((min - 1) / p) * p + p;
            while start <= *max {
                marks[(start - min) as usize] = true;
                start += p;
            }
        }

        let mut cnt = 0;
        for i in *min..=*max {
            if !marks[(i - min) as usize] {
                cnt += 1;
            }
        }

        cnt
    }

    pub fn primes(max: u64) -> Vec<u64> {
        let mut primes = vec![];
        let mut marks = [false; 1_001_000];

        for i in 2.. {
            if i * i > max {
                break;
            }
            if !marks[i as usize] {
                primes.push(i);
                let mut multiples = i;
                while multiples * multiples <= max {
                    marks[multiples as usize] = true;
                    multiples += i;
                }
            }
        }

        primes
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        input::TestCase,
        solution::{primes, solve},
    };

    #[test]
    fn case_0() {
        assert_eq!(solve(&TestCase { min: 1, max: 10 }), 7)
    }

    #[test]
    fn case_1() {
        assert_eq!(solve(&TestCase { min: 15, max: 15 }), 1)
    }

    #[test]
    fn case_2() {
        assert_eq!(solve(&TestCase { min: 1, max: 1000 }), 608)
    }

    #[test]
    fn primes_case_0() {
        assert_eq!(primes(100), vec![2, 3, 5, 7])
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let case = TestCase::from(&input).unwrap();
    println!("{}", solve(&case));
}
