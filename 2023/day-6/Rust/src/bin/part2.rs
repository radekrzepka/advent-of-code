use std::fs;
use std::io::{self};

pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let mut lines = file.split("\r\n");

    let time = lines.next().unwrap().split(":").last().unwrap().split_ascii_whitespace().collect::<String>().parse::<i64>().unwrap();
    let record = lines.next().unwrap().split(":").last().unwrap().split_ascii_whitespace().collect::<String>().parse::<i64>().unwrap();

    let mut result = 0;

    for i in 1..time {
        if i * (time - i) > record {result +=1}
    }

    println!("{result}");

    Ok(())
}