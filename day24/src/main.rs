use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    out: &'a str,
    op: Op,
}

fn main() {
    let result = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut wire_list: HashSet<String> = HashSet::new();
        let mut wire_value: HashMap<&str, bool> = HashMap::new();
        let mut gates: Vec<Gate> = Vec::new();

        let (starting_values, str_gates) = contents.split_once("\n\n").unwrap();

        for val in starting_values.split("\n") {
            let (val_str, _) = val.split_once(": ").unwrap();
            wire_list.insert(val_str.to_string());
        }

        for val in starting_values.split("\n") {
            let (val_str, b) = val.split_once(": ").unwrap();
            wire_value.insert(wire_list.get(val_str).unwrap(), b.parse().unwrap());
        }

        for val in str_gates.split("\n").filter(|l| l.len() > 0) {
            let iter = val.split(" ").filter(|t| *t != "->").collect::<Vec<_>>();
            wire_list.insert(iter[0].to_string());
            wire_list.insert(iter[2].to_string());
            wire_list.insert(iter[3].to_string());
        }

        for val in str_gates.split("\n").filter(|l| l.len() > 0) {
            let iter = val.split(" ").filter(|t| *t != "->").collect::<Vec<_>>();
            gates.push(Gate {
                left: wire_list.get(iter[0]).unwrap(),
                right: wire_list.get(iter[2]).unwrap(),
                out: wire_list.get(iter[3]).unwrap(),
                op: match iter[1] {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    _ => panic!(),
                }
            });
        }

        (wire_list, wire_value, gates)
    };

    
}
