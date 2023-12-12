use std::fs;
use rayon::prelude::*;
use num::integer::lcm;

fn load_file(file_name: &str) -> String {
    let file_parse = fs::read_to_string(file_name);

    if file_parse.is_ok() {
       return file_parse.unwrap(); 
    } else {
        panic!("Error reading file: {}", file_name);
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String
}

impl Node {
    fn new() -> Node {
        Node {
            name: "".to_string(),
            left: "".to_string(),
            right: "".to_string()
        }
    }

    fn next(&self, c: char) -> String {
        if c == 'L' {
            return self.left.to_string();
        }

        if c == 'R' {
            return self.right.to_string();
        }
        
        return "".to_string();
    }
}

fn parse_line(line: &str) -> Node {
    let mut split: Vec<&str> = line.split(" = ").collect();
    let name = split[0];
    let links = split[1].replace("(", "").replace(")", "").replace(",", "");

    split = links.split_ascii_whitespace().collect();
    
    return Node {
        name: name.to_string(),
        left: split[0].to_string(),
        right: split[1].to_string()
    }
}

fn is_x_node(node: &Node, x: char) -> bool {
    return node.name.chars().collect::<Vec<char>>()[2] == x;
}

fn get_x_nodes(nodes: &Vec<Node>, x: char) -> Vec<Node> {
    let filter = nodes.iter().filter(|&n| is_x_node(n, x));
    let mut x_nodes = Vec::<Node>::new();

    for node in filter {
        x_nodes.push(node.clone());
    }

    return x_nodes;
}

fn main() {
    let file = load_file("input.txt");
    let mut lines: Vec<&str> = file.split("\n").collect(); 
    let dirs: Vec<char> = lines[0].chars().collect();
    let mut nodes = Vec::<Node>::new();

    lines.remove(0);
    lines.remove(0);
    lines.remove(lines.len() - 1);

    for line in lines {
        nodes.push(parse_line(line)); 
    }

    let mut current = &Node::new();
    for node in &nodes {
        if node.name == "AAA" {
            current = node;
        }
    }

    let mut total_moves: i64  = 0;
    let mut d = 0;
    let mut next_str: String;
    
    while current.name != "ZZZ" {
        next_str = current.next(dirs[d]);

        for node in &nodes {
            if node.name == next_str {
                current = node;
            }
        }

        total_moves += 1;
        if d == dirs.len() - 1 {
            d = 0;
        } else {
            d += 1;
        }
    }

    println!("Part 1 - The total number of moves is {}", total_moves);

    total_moves = 0;
    d = 0;
    let a_nodes = get_x_nodes(&nodes, 'A');
    let mut current_nodes = a_nodes.clone();
    
    let mut min_moves = Vec::<i64>::new();

    while &min_moves.len() < &6 {
        current_nodes.par_iter_mut().for_each(|current_node| {
            let next_str = current_node.next(dirs[d]);
            
            for node in &nodes {
                if node.name == next_str {
                    *current_node = node.clone();
                }
            }

        });
        
        total_moves += 1;
        
        for current_node in &current_nodes {
            if is_x_node(&current_node, 'Z') {
                min_moves.push(total_moves);
            }
        }

        if d == dirs.len() - 1{ 
            d = 0;
        } else {
            d += 1;
        }
    }

    total_moves = min_moves[0]; 
    min_moves.remove(0);

    for n in min_moves {
        total_moves = lcm(total_moves, n)
    }

    println!("Part 2 - The total number of moves is {}", total_moves); 
}
