use std::fs;
use std::io::{self};

#[derive(Debug, Clone)]
struct Node {
    start: String,
    left: String,
    right: String,
}


pub fn main() -> io::Result<()> {
    let file = fs::read_to_string("./src/inputs/input.txt").unwrap();
    let mut lines = file.split("\r\n");
    let mut nodes: Vec<Node> = Vec::new();

    let steps_list = lines.next().unwrap();

    for node in lines.skip(1) {
        let mut split = node.split("=");
        let start = split.next().unwrap().trim().to_string();
        let paths_str = split.next().unwrap().trim().replace("(", "").replace(")", "");
        let mut paths = paths_str.split(",");
        let left = paths.next().unwrap().trim().to_string();
        let right = paths.next().unwrap().trim().to_string();
        
        nodes.push(Node{start, left, right});
    }
    
    let mut actual_path = "AAA";
    let mut steps = 0;
    
    while actual_path != "ZZZ" {        
        for step in steps_list.chars() {
            let founded_path = nodes.iter().find(|node| node.start == actual_path).unwrap();

            if step == 'L' {
                actual_path = &founded_path.left;
            } else {
                actual_path = &founded_path.right;
            }

            steps += 1;
        }
    }

    println!("{steps}");

    Ok(())
}