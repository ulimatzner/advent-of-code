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

    let mut increments = 0;
    let mut oldest = 999;
    let mut old = 999;
    let mut current = 999;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let new: i32 = line.parse().unwrap();
        if old + current + new > oldest + old + current {
            increments += 1;
        }
        oldest = old;
        old = current;
        current = new;
    }

    return increments;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_path = Path::new("tests/input.txt");

        let increments = run(test_path);
        assert_eq!(5, increments)
    }
}
