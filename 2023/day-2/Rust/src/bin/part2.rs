use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn main() -> io::Result<()> {
    let mut sum = 0;
    let file = File::open("./src/inputs/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut min_values = Game{
            red: 0,
            green: 0,
            blue: 0, 
        };

        let line = line?;
        let split = line.split(':');

        let sets = split.last().unwrap().split(";");

        for set in sets {
            let mut set_points = Game{
                red: 0,
                green: 0,
                blue: 0, 
            };

            let choices = set.split(",");
            for choice in choices {
                let mut split = choice.split(" ");
                split.next();
                let cube_number_str = split.next().unwrap();
                let cube_number = cube_number_str.parse::<i32>().unwrap_or(0);
                let color = split.last().unwrap();

                match color {
                    "red" => set_points.red += cube_number,
                    "green" => set_points.green += cube_number,
                    "blue" => set_points.blue += cube_number,
                    _ => {}
                };
            }
            
            if min_values.red < set_points.red {min_values.red = set_points.red};
            if min_values.blue < set_points.blue {min_values.blue = set_points.blue};
            if min_values.green < set_points.green {min_values.green = set_points.green};
        }

        sum += min_values.red * min_values.green * min_values.blue;
    }

    println!("{}",sum);

    Ok(())
}