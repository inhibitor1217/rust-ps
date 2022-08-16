use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let [a, b] = &input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()[..]
    {
        println!(
            "{}",
            if a < b {
                "<"
            } else if a > b {
                ">"
            } else {
                "=="
            },
        );
    }
}
