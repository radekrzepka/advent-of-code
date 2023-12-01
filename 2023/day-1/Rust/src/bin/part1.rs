use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn main() -> io::Result<()> {
    let mut sum = 0;
    let file = File::open("./src/inputs/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let numbers: String = line?.chars().filter(|char| char.is_numeric()).collect();
        
        if numbers.len() >= 2 {
            let first_char = numbers.chars().next().unwrap();
            let last_char = numbers.chars().last().unwrap();
            let concatenated = format!("{}{}", first_char, last_char);

            if let Ok(number) = concatenated.parse::<i32>() {
                sum += number;
            }
        } else if numbers.len() == 1 {
            let duplicated = format!("{}{}", numbers, numbers);

            if let Ok(number) = duplicated.parse::<i32>() {
                sum += number;
            }
        }
    }

    println!("{}",sum);

    Ok(())
}