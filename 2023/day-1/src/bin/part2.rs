use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn main() -> io::Result<()> {
    let mut sum = 0;
    let file = File::open("./src/inputs/input.txt")?;
    let reader = BufReader::new(file);
    let nums = ["one","two","three","four","five","six","seven","eight","nine"];

    for line in reader.lines() {
        let mut numbers: Vec<i32> = Vec::new();
        let mut str = String::new();
        
        for char in line?.chars() {
            if char.is_numeric() {
                if let Ok(number) = char.to_string().parse::<i32>() {
                    numbers.push(number);
                }
            } else {
                str.push(char);
            }

            if let Some(index) = nums.iter().position(|&element| str.ends_with(element)) {
                let adjusted_index = index as i32 + 1; 
                numbers.push(adjusted_index);
                str = str.chars().last().unwrap().to_string();
            }
        }

        if !numbers.is_empty() {
            let first_num = numbers[0];
            let last_num = *numbers.last().unwrap_or(&first_num);
        
            if numbers.len() >= 2 {
                sum += 10 * first_num + last_num;
            } else {
                sum += 11 * first_num;
            }
        }
    }

    println!("{}",sum);

    Ok(())
}