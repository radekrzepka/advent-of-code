use std::fs;
use std::io::{self};

fn generate_values(values: &mut Vec<Vec<i32>>) {
    let current_values = values.last().unwrap();
    let mut new_values: Vec<i32> = Vec::new();

    for i in 0..current_values.len() - 1 {
        new_values.push(current_values[i + 1] - current_values[i]);
    }

    values.push(new_values.clone()); 

    if new_values.iter().all(|&v| v == 0) { 
        return; 
    }
    
    generate_values(values);
}

fn add_values(values: &mut Vec<Vec<i32>>) {
    values.last_mut().unwrap().push(0);
    
    for i in (0..values.len() - 2).rev() {
        let next_last_value = values[i + 1].last().copied().unwrap();
        let last_value = values[i].last().copied().unwrap();
        values[i].push(next_last_value + last_value);
    }

}

pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let lines = file.split("\r\n");
    let mut sum = 0;

    for line in lines {
        let first_values: Vec<i32> = line.split(" ").into_iter().filter_map(|v| v.parse::<i32>().ok()).collect();
        let mut values: Vec<Vec<i32>> = Vec::new();

        values.push(first_values);
        generate_values(&mut values);
        add_values(&mut values);

        sum += values.first().unwrap().last().unwrap();
    }

    println!("{}", sum);

    Ok(())
}