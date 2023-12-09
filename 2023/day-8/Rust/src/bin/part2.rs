use std::fs;
use std::io::{self};
use std::time::Instant;

#[derive(Debug, Clone)]
struct Node {
    start: String,
    left: String,
    right: String,
}


fn find_node<'a>(nodes: &'a [Node], node_start_to_find: &str) -> &'a Node {
    return nodes.iter().find(|node| node.start == node_start_to_find).expect("Node not found")
}

fn count_node(nodes: &[Node], current_node: &Node, steps_list: &str) -> i64 {
    let mut step_number = 0;
    let mut current_node = current_node;

    while !current_node.start.ends_with("Z") {
        for step in steps_list.chars() {
            let node_to_find = if step == 'L' { 
                &current_node.left 
            } else { 
                &current_node.right 
            };
            current_node = find_node(nodes, node_to_find);
            step_number += 1;
            if current_node.start.ends_with("Z") {return step_number;}
        }
    }

    step_number
}

fn nwd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        nwd(b, a % b)
    }
}

fn nww(a: i64, b: i64) -> i64 {
    a * b / nwd(a, b)
}

fn vec_nww(numbers: &Vec<i64>) -> i64 {
    numbers.iter().fold(1, |acc, &num| nww(acc, num))
}


pub fn main() -> io::Result<()> {
    let start = Instant::now();
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
    
    let mut current_nodes: Vec<Node> = Vec::new();

    for node in &nodes {
        if node.start.ends_with("A") {current_nodes.push(node.clone())}
    }

    let mut results_node: Vec<i64> = Vec::new();

    for node in current_nodes {
        let counted_node = count_node(&nodes, &node, &steps_list);
        results_node.push(counted_node);
    }

    let duration = start.elapsed();
    println!("{:?}", vec_nww(&results_node));
    println!("Time taken: {:?}", duration);
    Ok(())
}