use std::fs;
use std::io::{self};

#[derive(Debug)]
struct Race {
    time: i32,
    record: i32
}

pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let lines = file.split("\r\n");
    let mut races: Vec<Race> = Vec::new();
    let mut result = 1;
 
    for (i,line) in lines.into_iter().enumerate() {
        let values = line.split(":").last().unwrap().split_ascii_whitespace().filter_map(|v| v.parse::<i32>().ok());

        for (j,value) in values.into_iter().enumerate() {
            if i == 0 { races.push(Race{time: value,record: 0});}
            else {races[j].record = value};
        }
    }

    for race in races {
        let mut times: Vec<i32> = Vec::new();

        for i in 1..race.time {
            let left_time = race.time - i;
            times.push(left_time * i);
        }

        result *= times.into_iter().filter(|time| time > &race.record).count();
    }

    println!("{result}");

    Ok(())
}