use std::io::{self, Read};

mod time {
    pub struct Time(u32);

    impl Time {
        pub fn new(hour: u32, minute: u32) -> Self {
            Time(hour * 60 + minute)
        }

        pub fn subtract(&mut self, minute: u32) {
            self.0 = (self.0 + 1440 - minute) % 1440;
        }

        pub fn hour(&self) -> u32 {
            self.0 / 60
        }

        pub fn minute(&self) -> u32 {
            self.0 % 60
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    if let [hour, minute] = &input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>()[..]
    {
        let mut time = time::Time::new(*hour, *minute);
        time.subtract(45);
        println!("{} {}", time.hour(), time.minute());
    }
}
