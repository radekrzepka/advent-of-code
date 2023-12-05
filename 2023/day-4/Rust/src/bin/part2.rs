use std::collections::{HashSet, HashMap};
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

    let mut copies: HashMap<i32,i32> = HashMap::new();

    for (i ,line) in reader.lines().enumerate() {
        let line = line?;
        let mut values  = line.split(":").last().unwrap().split("|");
        let input = values.next().unwrap();

        let winning_numbers = parse_numbers(input);
        let winning_numbers_set: HashSet<i32> = HashSet::from_iter(winning_numbers.iter().cloned());
    
        let guesses_input = values.next().unwrap();
        let guesses = parse_numbers(guesses_input);

        let mut correct_guesses = 0;

        for guess in guesses {
            if winning_numbers_set.contains(&guess) {correct_guesses += 1};
        }

        let number_of_copies = *copies.get(&(i as i32)).unwrap_or(&1);

        for k in i + 1..i + 1 + correct_guesses {
            let new_cards = *copies.get(&(k as i32)).unwrap_or(&1) + number_of_copies;
            copies.insert(k as i32, new_cards);
        }

        copies.insert(i as i32, number_of_copies);
    }

    println!("{}", copies.values().map(|&val| val).sum::<i32>());
    Ok(())
}