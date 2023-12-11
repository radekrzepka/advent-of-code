use std::fs;
use std::io::{self};

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize
}

const ADDITIONAL_EMPTY_SPACE: i64 = 1_000_000;

pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt")?;
    let lines: Vec<&str> = file.split("\r\n").collect();
    let mut distance = 0;

    let mut empty_rows: Vec<i64> = Vec::new();
    let mut empty_cols: Vec<i64> = Vec::new();
    let mut galaxies: Vec<Galaxy> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if line.chars().all(|ch| ch == '.') {
            empty_rows.push(i as i64);
        }
    }

    let row_len = lines[0].len();

    for col_index in 0..row_len {
        let mut is_all_dots = true;

        for line in &lines {
            if line.chars().nth(col_index) != Some('.') {
                is_all_dots = false;
                break;
            }
        }

        if is_all_dots {
            empty_cols.push(col_index as i64);
        }
    }

    for (row_num, &line) in lines.iter().enumerate() {
        for (col_num, ch) in line.chars().enumerate() {
            if ch == '#' {
                let galaxy = Galaxy { x: row_num, y: col_num };
                galaxies.push(galaxy);
            }
        }
    }

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let additional_rows = empty_rows.iter().filter(|&&v| v >= galaxies[i].x as i64 && v <= galaxies[j].x as i64).count();

            let (start_col, end_col) = if galaxies[i].y > galaxies[j].y {
                (galaxies[j].y, galaxies[i].y)
            } else {
                (galaxies[i].y, galaxies[j].y)
            };

            let additional_cols = empty_cols.iter().filter(|&&v| v >= start_col as i64 && v <= end_col as i64).count();

            let new_distance = (galaxies[i].x as i64 - galaxies[j].x as i64).abs() + additional_rows as i64 * (ADDITIONAL_EMPTY_SPACE - 1) + (galaxies[i].y as i64 - galaxies[j].y as i64).abs() + additional_cols as i64 * (ADDITIONAL_EMPTY_SPACE - 1);
            distance += new_distance;

            // println!("({:?},{:?}) {:?} {:?} {:?} {:?} {:?}", i + 1, j + 1 ,galaxies[i], galaxies[j], new_distance, additional_rows, additional_cols);
        }
    }    

    println!("{distance}");

    Ok(())
}