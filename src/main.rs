fn main() {
    let file_name= "../data.input.txt";
    println!("Number of increments: {}", run(file_name));
}

fn run(file_name: &str) -> i32  {
    println!("File name is {}", file_name);
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_example() {
        let test_file_name = "../tests/input.txt";
        
        let increments = run(test_file_name);
        assert_eq!(7, increments)
    }
}

