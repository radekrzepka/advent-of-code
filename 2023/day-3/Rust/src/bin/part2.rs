use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq, Clone)]
struct Cords {
    i:i32,
    j:i32,
}

#[derive(Debug, PartialEq, Clone)]
struct Number {
    number: i32,
    cords: Vec<Cords>,
}

fn find_numbers(numbers: &Vec<Number>, schema: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<Number> {
    let row_start = if i > 0 { i - 1 } else { 0 };
    let row_end = if i < schema.len() - 1 { i + 2 } else { schema.len() };
    let col_start = if j > 0 { j - 1 } else { 0 };
    let col_end = if j < schema[0].len() - 1 { j + 2 } else { schema[0].len() };
    let mut founded_numbers: Vec<Number> = vec![];

    for k in row_start..row_end {
        for l in col_start..col_end {
            let cords = Cords{
                i: k as i32,
                j: l as i32
            };

            for number in numbers {
                if number.cords.contains(&cords) && !founded_numbers.contains(&number) {founded_numbers.push(number.clone())}
            }
        }
    }

    return founded_numbers;
}

pub fn main() -> io::Result<()> {
    let file = File::open("./src/inputs/input.txt")?;
    let reader = BufReader::new(file);
    let mut schema: Vec<Vec<char>> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap(); 
        let row_chars = line.chars();
        let mut row: Vec<char> = Vec::new();
        for char in row_chars {
            row.push(char);
        }
        schema.push(row);
    }

    for (i, row) in schema.iter().enumerate() {
        let mut number = Number {
            number: 0,
            cords: vec![],
        };
        let mut str_number = String::new();

        for (j, &char) in row.iter().enumerate() {
            if char.is_digit(10) {
                str_number.push(char);
                number.cords.push(Cords{
                    i:i as i32,
                    j:j as i32
                });
            } else  {
                if !str_number.is_empty() {
                    number.number = str_number.parse::<i32>().unwrap();
                    numbers.push(number);
                }

                number = Number {
                    number: 0,
                    cords: vec![],
                };
                str_number.clear();
            }
        }

        if !str_number.is_empty() {
            number.number = str_number.parse::<i32>().unwrap();
            numbers.push(number);
        }
    }

    for (i, row) in schema.iter().enumerate() {
        for (j, &char) in row.iter().enumerate() {
            if char != '*' {continue;}
            let founded_numbers = find_numbers(&numbers, &schema, i, j);

            if founded_numbers.len() == 2 {
                sum += founded_numbers[0].number * founded_numbers[1].number;
            }
        }
    }
    
    println!("{:?}", sum);
    Ok(())
}