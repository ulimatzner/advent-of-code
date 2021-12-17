use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
fn main() {
    let path = Path::new("data/input.txt");
    println!("Solution: {}", run(path));
}

fn run(path: &Path) -> i32 {

    let (sequence, mut bingos) = read_input(path);

    for number in sequence {
        let total_bingos = bingos.len();
        for i in 0..total_bingos {
            for j in 0..25 {
                if bingos[i][j] == number {
                    bingos[i][j] = -1;
                }
            }
        }
        if total_bingos == 1 && is_winner(&bingos[0]) {
            return calc_score(&bingos[0], number);
        }
        bingos.retain(|bingo| !is_winner(bingo));
    }

    panic!("End of bingos reached without winner.");
}

fn read_input(path: &Path) -> (Vec<i32>, Vec<[i32; 25]>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut lines_res = reader.lines();
    let first_line = lines_res.next().unwrap().unwrap();
    let sequence = first_line.split(',').map(
        | str | -> i32 {
            str.parse::<i32>().unwrap()
        }
    ).collect::<Vec<i32>>();

    lines_res.next();

    let mut bingos: Vec<[i32; 25]> = Vec::new();
    let mut single_bingo = [0; 25];
    let mut counter = 0;

    for line_res in lines_res {
        let line = line_res.unwrap();
        if counter == 5 {
            bingos.push(single_bingo.clone());
            single_bingo = [0; 25];
            counter = 0;
            continue;
        }
        let mut j = 0;
        for number_as_str in line.split_whitespace()  {
            single_bingo[5*counter + j] = number_as_str.parse::<i32>().unwrap();
            j += 1;
        }
        counter += 1;
    }
    bingos.push(single_bingo.clone());

    return (sequence, bingos);
}

fn is_winner(bingo: &[i32; 25]) -> bool {
    for i in 0..5 {
        let mut horizontal = true;
        let mut vertical = true;
        for j in 0..5 {
            if bingo[5*i+j] >= 0 {
                horizontal = false;
            }
            if bingo[i+5*j] >= 0 {
                vertical = false;
            }
        }
        if horizontal || vertical {
            return true;
        };
    }
    return false;
}

fn calc_score(bingo: &[i32; 25], number: i32) -> i32 {
    let mut sum = 0;
    for i in 0..25 {
        if bingo[i] >= 0 {
            sum += bingo[i]
        }
    }
    return sum * number;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_path = Path::new("tests/input.txt");

        let solution = run(test_path);
        assert_eq!(1924, solution)
    }
}
