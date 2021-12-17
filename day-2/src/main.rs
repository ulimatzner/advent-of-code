use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
fn main() {
    let path = Path::new("data/input.txt");
    println!("Number of increments: {}", run(path));
}

fn run(path: &Path) -> i32 {
    let file = File::open(path).unwrap();    
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let mut split = line.splitn(2,' ');

        let command = split.next().unwrap();
        let number: i32 = split.next().unwrap().parse().unwrap();

        match command {
            "forward" => {horizontal += number; depth = depth + (aim * number)},
            "up" => aim -= number,
            "down" => aim += number,
            _ => panic!("unknown command: {}", command)
        }
    }
    println!("hor = {}, depth = {}, aim = {}", horizontal, depth, aim);
    return horizontal * depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_path = Path::new("tests/input.txt");

        let increments = run(test_path);
        assert_eq!(900, increments)
    }
}
