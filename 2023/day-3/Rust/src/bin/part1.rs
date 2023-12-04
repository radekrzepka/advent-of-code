use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_symbol (symbol: char) -> bool {
    if !symbol.is_digit(10) && symbol != '.' {return true;}
    return false;
}

fn has_symbol(schema: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let row_start = if i > 0 { i - 1 } else { 0 };
    let row_end = if i < schema.len() - 1 { i + 2 } else { schema.len() };
    let col_start = if j > 0 { j - 1 } else { 0 };
    let col_end = if j < schema[0].len() - 1 { j + 2 } else { schema[0].len() };

    for k in row_start..row_end {
        for l in col_start..col_end {
            if is_symbol(schema[k][l]) {return true;}
        }
    }

    return false;
}
 
pub fn main() -> io::Result<()> {
    let file = File::open("./src/inputs/day3input.txt")?;
    let reader = BufReader::new(file);
    let mut schema: Vec<Vec<char>> = Vec::new();

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
        let mut has_number_symbol = false;
        let mut str_num = String::new();
    
        for (j, &char) in row.iter().enumerate() {
            if char.is_digit(10) {
                str_num.push(char);
                if !has_number_symbol {has_number_symbol = has_symbol(&schema, i, j);}
            } else if !str_num.is_empty() {
                if has_number_symbol {
                    sum += str_num.parse::<i32>().unwrap();
                }
                has_number_symbol = false;
                str_num.clear();
            }
        }

        if has_number_symbol && !str_num.is_empty() {
            sum += str_num.parse::<i32>().unwrap();
            str_num.clear();
        }
    }
    
    println!("{}", sum);
    Ok(())
}