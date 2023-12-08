use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

fn read_file(filename: &str) -> String {
    let file_contents: String =
        fs::read_to_string(filename).expect("Should have been able to read file");

    return file_contents;
}

fn parse_node(line: &str) -> (String, String, String) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\S+)\s*=\s*\(\s*(\S+)\s*,\s*(\S+)\s*\)").expect("Regex should compile");
    }

    let caps: Captures = RE.captures(line).expect("Should be able to capture groups");
    let current_node: String = caps
        .get(1)
        .expect("Should be able to parse node")
        .as_str()
        .to_string();
    let left_node: String = caps
        .get(2)
        .expect("Should be able to parse node")
        .as_str()
        .to_string();
    let right_node: String = caps
        .get(3)
        .expect("Should be able to parse node")
        .as_str()
        .to_string();

    return (current_node, left_node, right_node);
}

fn gcd(a: u128, b: u128) -> u128 {
    if a == 0 {
        return a;
    }

    if b == 0 {
        return a;
    }

    return gcd(b, a % b);
}

fn lcm(a: u128, b: u128) -> u128 {
    return (a * b) / gcd(a, b);
}

fn main() {
    let filename: &str = "../network.txt";
    let file_contents: String = read_file(filename);
    let lines: Vec<&str> = file_contents.split::<char>('\n').collect::<Vec<&str>>();

    let mut instructions: Vec<char> = Vec::new();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        if i == 0 {
            instructions = line.chars().collect::<Vec<char>>();
        } else {
            let (current_node, left_node, right_node) = parse_node(line);
            network.insert(current_node, (left_node, right_node));
        }
    }

    let src: String = String::from_str("AAA").expect("Should be able to construct String");
    let dest: String = String::from_str("ZZZ").expect("Should be able to construct String");

    let mut current_node: String = src.clone();
    let mut steps: u32 = 0;
    let mut i: usize = 0;
    loop {
        if current_node == dest {
            break;
        }

        let children_nodes = network
            .get(&current_node)
            .expect("Should be able to find node");
        let direction = instructions[i];
        if direction == 'L' {
            current_node = children_nodes.0.clone();
        } else {
            current_node = children_nodes.1.clone();
        }

        steps += 1;
        i = (i + 1) % instructions.len();
    }

    println!("{:?}", steps);

    let current_nodes: Vec<String> = network
        .keys()
        .filter(|&n| n.ends_with("A"))
        .map(|n| n.clone())
        .collect::<Vec<String>>();
    let mut lcm_steps: u128 = 0;
    for src_node in current_nodes {
        let mut current_node: String = src_node.clone();
        let mut visited_nodes: HashSet<(String, usize)> = HashSet::new();
        let mut steps: u32 = 0;
        let mut i: usize = 0;
        loop {
            if let Some(_) = visited_nodes.get(&(current_node.clone(), i)) {
                break;
            }

            visited_nodes.insert((current_node.clone(), i));

            if current_node.ends_with("Z") {
                break;
            }

            let children_nodes = network
                .get(&current_node)
                .expect("Should be able to find node");
            let direction = instructions[i];
            if direction == 'L' {
                current_node = children_nodes.0.clone();
            } else {
                current_node = children_nodes.1.clone();
            }

            steps += 1;
            i = (i + 1) % instructions.len();
        }

        if lcm_steps == 0 {
            lcm_steps = steps as u128;
        } else {
            lcm_steps = lcm(lcm_steps, steps as u128);
        }
    }

    println!("{}", lcm_steps);
}
