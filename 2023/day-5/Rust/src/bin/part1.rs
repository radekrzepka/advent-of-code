use std::fs;
use std::io::{self};

#[derive(Debug, Clone, Copy)]
struct Seed {
    number: i64,
    has_changed: bool,
}

pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let mut input = file.split("\r\n\r\n");
    let mut seeds: Vec<Seed> = input.next().unwrap().split(":").last().unwrap().split(" ").filter_map(|v| v.parse::<i64>().ok()).map(|v| Seed{number: v, has_changed: false}).collect();

    for map_str in input.by_ref() {
        let seed_data: std::str::Split<'_, &str> = map_str.split(":").last().unwrap().split("\r\n");
        for line in seed_data {
            if line == "" {
                seeds = seeds.iter().map(|seed| Seed{number: seed.number, has_changed: false}).collect();
                continue;
            }
            
            let mut record = line.split(" ");

            let destination_start = record.next().unwrap().parse::<i64>().unwrap();
            let source_start = record.next().unwrap().parse::<i64>().unwrap();
            let range_length = record.next().unwrap().parse::<i64>().unwrap();

            seeds = seeds.iter().map(|seed| {
                if seed.number >= source_start && seed.number < source_start + range_length && !seed.has_changed {
                    let difference = seed.number - source_start;
                    Seed{
                        number: destination_start + difference,
                        has_changed: true,
                    }
                } else {
                    *seed
                }
            }).collect();
        }
    }
    println!("{:?}", seeds.iter().map(|seed| seed.number).min().unwrap());
    Ok(())
}