use std::fs;
use std::io::{self};

#[derive(Debug, PartialEq)]
struct Offset {
    row: i32,
    col: i32,
}

#[derive(Debug,Copy, Clone, PartialEq)]
enum FieldType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition,
}

impl FieldType {
    fn to_char(&self) -> char {
        match self {
            FieldType::Vertical => '║',
            FieldType::Horizontal => '═',
            FieldType::NorthEast => '╚',
            FieldType::NorthWest => '╝',
            FieldType::SouthWest => '╗',
            FieldType::SouthEast => '╔',
            FieldType::Ground => '.',
            FieldType::StartingPosition => 'S',
        }
    }

    fn from_char(c: char) -> FieldType {
        match c {
            '|' => FieldType::Vertical,
            '-' => FieldType::Horizontal,
            'L' => FieldType::NorthEast,
            'J' => FieldType::NorthWest,
            '7' => FieldType::SouthWest,
            'F' => FieldType::SouthEast,
            '.' => FieldType::Ground,
            'S' => FieldType::StartingPosition,
            _ => panic!("Invalid character: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Field {
    value: FieldType,
    display_value: char,
    row: i32,
    col: i32
}


impl Field {
    fn new(value: char, row: i32, col: i32) -> Field {
        Field {
            value: FieldType::from_char(value),
            display_value: FieldType::to_char(&FieldType::from_char(value)),
            row,
            col,
        }
    }

    fn get_offsets(&self) -> [Offset; 2] {
        match self.value {
            FieldType::Vertical => [Offset { row: 1, col: 0 }, Offset { row: -1, col: 0 }],
            FieldType::Horizontal => [Offset { row: 0, col: 1 }, Offset { row: 0, col: -1 }],
            FieldType::NorthEast => [Offset { row: -1, col: 0 }, Offset { row: 0, col: 1 }],
            FieldType::NorthWest => [Offset { row: -1, col: 0 }, Offset { row: 0, col: -1 }],
            FieldType::SouthEast => [Offset { row: 1, col: 0 }, Offset { row: 0, col: 1 }],
            FieldType::SouthWest => [Offset { row: 1, col: 0 }, Offset { row: 0, col: -1 }],
            FieldType::Ground => [Offset { row: 0, col: 0 }, Offset { row: 0, col: 0 }],
            FieldType::StartingPosition => [Offset { row: 0, col: 0 }, Offset { row: 0, col: 0 }],
        }
    }
}

pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/mikolaj.txt").unwrap();
    let lines = file.split("\r\n");
    let mut map: Vec<Vec<Field>> = Vec::new();

    for (row_num ,line) in lines.enumerate() {
        let mut row: Vec<Field> = Vec::new();
        for (col_num, ch) in line.chars().enumerate() {
            let field = Field::new(ch, row_num as i32, col_num as i32);
            print!("{}", field.display_value);
            row.push(field);
        }
        print!("\n");
        map.push(row);
    }

    let mut map_loop: Vec<Field> = Vec::new();
    let mut start: Option<Field> = None;

    for line in &map {
        if let Some(field) = line.iter().find(|&field| field.value == FieldType::StartingPosition) {
            start = Some(*field);
            break;
        }
    }
    map_loop.push(start.unwrap());
    map_loop.push(map[(start.unwrap().row + 1) as usize][start.unwrap().col as usize]);

    while map_loop.last().unwrap().value != FieldType::StartingPosition {
        let previous_element = map_loop[map_loop.len() - 2];
        let current_element = map_loop[map_loop.len() - 1];
        let offset_from_previous_to_current = Offset{row: (previous_element.row as i32 - current_element.row as i32) as i32, col: (previous_element.col as i32 - current_element.col as i32) as i32};
        
        let offset_to_move = current_element.get_offsets().into_iter().find(|offset| *offset != offset_from_previous_to_current).unwrap();
        map_loop.push(map[(current_element.row + offset_to_move.row) as usize][(current_element.col + offset_to_move.col) as usize]);
    }

    println!("{:?}", map_loop.len() / 2);

    Ok(())
}