use std::fs;
use std::io::{self};

#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize
}


pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let lines = file.split("\r\n");

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut distance = 0;

    for line in lines {
        if line.chars().into_iter().all(|ch| ch == '.') {map.push(line.chars().collect())}
        map.push(line.chars().collect());
    }

    let mut row_len = map[0].len();
    let mut col_index = 0;

    while col_index < row_len {
        let mut is_all_dots = true;

        for row in &map {
            if let Some(&ch) = row.get(col_index) {
                if ch != '.' {
                    is_all_dots = false;
                    break;
                }
            } else {
                is_all_dots = false;
                break;
            }
        }

        if is_all_dots {
            for row in map.iter_mut() {
                row.insert(col_index + 1, '.');
                row_len += 1;
            }
            col_index += 1;
        }

        col_index += 1;
    }

    for (row_num, line) in map.iter().enumerate() {
        for (col_num, &ch) in line.iter().enumerate() {
            if ch == '#' {
                let galaxy = Galaxy { x: row_num, y: col_num };
                galaxies.push(galaxy);
            }
        }
    }

    for i in 0..galaxies.len() - 1 {
        for j in i+1..galaxies.len() {
            let new_distance = (galaxies[i].x as i32 - galaxies[j].x as i32).abs() + (galaxies[i].y as i32 - galaxies[j].y as i32).abs();
            distance += new_distance;

            // println!("({:?},{:?}) {:?} {:?} {:?}", i + 1, j + 1 ,galaxies[i], galaxies[j], new_distance);
        }
    }

    println!("{distance}");

    Ok(())
}