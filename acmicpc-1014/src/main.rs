use std::{
    collections::HashMap,
    io::{self, Stdin},
};

struct Input {
    stream: Stdin,
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

        if let Ok(_) = self.stream.read_line(&mut s) {
            Some(s)
        } else {
            None
        }
    }
}

fn main() {
    let mut input = Input::new();
    let t: u32 = input.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let map = MapState::build(&mut input).unwrap();
        let mut cache: HashMap<CacheKey, u32> = HashMap::new();
        println!("{}", solve(map, 0, 0, &mut cache));
    }
}

fn solve(state: MapState, x: i32, y: i32, cache: &mut HashMap<CacheKey, u32>) -> u32 {
    if let Some(cached_state) = cache.get(&state.cache_key((x, y))) {
        *cached_state
    } else if let Some((next_x, next_y)) = state.next_coord((x, y)) {
        let one = solve(state.clone(), next_x, next_y, cache);
        let ret = if state.can_place((x, y)) {
            let two = solve(state.place((x, y)), next_x, next_y, cache) + 1;
            if one > two {
                one
            } else {
                two
            }
        } else {
            one
        };

        cache.insert(state.cache_key((x, y)), ret.clone());
        ret
    } else {
        0
    }
}

#[derive(Debug, Clone)]
struct MapState {
    width: i32,
    height: i32,
    filled: u128,
}

impl MapState {
    fn build<T: Iterator<Item = String>>(lines: &mut T) -> Result<MapState, MapParseError> {
        let dim = lines.next().unwrap();
        if let [height, width] = &dim
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()[..]
        {
            let mut filled = 0u128;

            for i in 0..*height {
                let line = lines.next().unwrap();
                let mut mask = 0u128;
                for (i, ch) in line.trim().chars().enumerate() {
                    match ch {
                        'x' => {
                            mask |= 1 << i;
                        }
                        _ => {}
                    }
                }

                filled |= mask << (i * width);
            }

            return Ok(MapState {
                width: *width,
                height: *height,
                filled,
            });
        }

        Err(MapParseError)
    }

    fn mask(&self, (x, y): (i32, i32)) -> u128 {
        assert!(x >= 0 && x < self.height && y >= 0 && y < self.width);
        1 << (self.width * x + y)
    }

    fn mask_or_zero(&self, (x, y): (i32, i32)) -> u128 {
        if x < 0 || x >= self.height || y < 0 || y >= self.width {
            0
        } else {
            self.mask((x, y))
        }
    }

    fn can_place(&self, (x, y): (i32, i32)) -> bool {
        self.filled & self.mask((x, y)) == 0
    }

    fn place(&self, (x, y): (i32, i32)) -> MapState {
        let r = self.mask_or_zero((x + 1, y - 1))
            | self.mask_or_zero((x + 1, y + 1))
            | self.mask_or_zero((x, y - 1))
            | self.mask_or_zero((x, y + 1))
            | self.mask_or_zero((x, y));

        MapState {
            filled: self.filled | r,
            ..*self
        }
    }

    fn next_coord(&self, (x, y): (i32, i32)) -> Option<(i32, i32)> {
        if x >= self.height {
            None
        } else if y >= self.width - 1 {
            Some((x + 1, 0))
        } else {
            Some((x, y + 1))
        }
    }

    fn cache_key(&self, (x, y): (i32, i32)) -> CacheKey {
        let cache_mask = (self.filled >> (self.width * x + y)) & ((1 << (self.width + 2)) - 1);
        CacheKey(cache_mask, x, y)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CacheKey(u128, i32, i32);

#[derive(Debug)]
struct MapParseError;
