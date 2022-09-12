use std::io;

fn main() {
    let input = Input::new();
    let lines = input
        .map(|line| String::from(line.trim()))
        .collect::<Vec<_>>();

    let map = Map::build(&lines[1..]);

    let mut answer = map.width * map.height;
    for i in 0..=map.height - Map::CUT_SIZE {
        for j in 0..=map.width - Map::CUT_SIZE {
            answer = answer.min(map.count(i, j, Cell::Black, Cell::White));
            answer = answer.min(map.count(i, j, Cell::White, Cell::Black));
        }
    }

    println!("{}", answer);
}

struct Input {
    stream: io::Stdin,
}

impl Input {
    fn new() -> Input {
        Input {
            stream: io::stdin(),
        }
    }
}

impl Iterator for Input {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = String::new();

        if let Ok(bytes) = self.stream.read_line(&mut s) {
            match bytes {
                0 => None,
                _ => Some(s),
            }
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
enum Cell {
    Black,
    White,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    board: Vec<Vec<Cell>>,
}

impl Map {
    const CUT_SIZE: usize = 8;

    fn build_row(s: &str) -> Vec<Cell> {
        s.chars()
            .map(|c| match c {
                'W' => Cell::White,
                'B' => Cell::Black,
                _ => {
                    panic!("Invalid character");
                }
            })
            .collect()
    }

    fn build(rows: &[String]) -> Map {
        Map {
            width: rows[0].len(),
            height: rows.len(),
            board: rows.iter().map(|row| Self::build_row(row)).collect(),
        }
    }

    fn count(&self, top: usize, left: usize, target: Cell, other: Cell) -> usize {
        let mut count = 0;
        for i in top..top + Self::CUT_SIZE {
            for j in left..left + Self::CUT_SIZE {
                if (i + j) % 2 == 0 && self.board[i][j] != target {
                    count += 1;
                } else if (i + j) % 2 == 1 && self.board[i][j] != other {
                    count += 1;
                }
            }
        }

        count
    }
}
