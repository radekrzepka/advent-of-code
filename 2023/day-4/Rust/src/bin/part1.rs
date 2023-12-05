use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn parse_numbers(input: &str) -> Vec<i32> {
    input.split(" ")
         .filter_map(|val| val.parse::<i32>().ok())
         .collect()
}

pub fn main() -> io::Result<()> {
    let file = File::open("./src/inputs/input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mut values  = line.split(":").last().unwrap().split("|");
        let input = values.next().unwrap();

        let winning_numbers = parse_numbers(input);
        let winning_numbers_set: HashSet<i32> = HashSet::from_iter(winning_numbers.iter().cloned());
    
        let guesses_input = values.next().unwrap();
        let guesses = parse_numbers(guesses_input);

        let mut pow = -1;

        for guess in guesses {
            if winning_numbers_set.contains(&guess) {pow += 1};
        }

        sum += if pow != -1 { i32::pow(2, pow as u32) } else { 0 };
    }

    println!("{}", sum);
    Ok(())
}