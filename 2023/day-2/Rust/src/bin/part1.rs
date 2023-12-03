use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn main() -> io::Result<()> {
    let mut id_sum = 0;
    let file = File::open("./src/inputs/input.txt")?;
    let reader = BufReader::new(file);

    let max_values = Game{
        red: 12,
        green: 13,
        blue: 14
    };
    

    for line in reader.lines() {
        let line = line?;
        let mut split = line.split(':');

        let mut game_possible = true; 
        let id_str = split.next().unwrap().split(" ").last().unwrap();
        let id = id_str.parse::<i32>().unwrap_or(0); 
        let sets = split.next().unwrap().split(";");


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

            if set_points.red > max_values.red || set_points.blue > max_values.blue || set_points.green > max_values.green {
                game_possible = false;
                break;
            }
        }

        if game_possible {
            id_sum += id;
        }
    }
    println!("{}", id_sum);

    Ok(())
}