use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

fn is_valid_design(patterns: &Vec<&str>, cache: &mut HashMap<String, bool>, design: &str) -> bool {
    if let Some(v) = cache.get(design) {
        return *v;
    }

    if design.len() <= 0 {
        return true;
    }

    for i in 0..design.len() + 1 {
        if patterns.contains(&&design[..i]) && is_valid_design(patterns, cache, &design[i..]) {
            cache.insert(design.to_owned(), true);
            return true;
        }
    }

    cache.insert(design.to_owned(), false);
    false
}

fn num_valid_designs(
    patterns: &Vec<&str>,
    cache: &mut HashMap<String, usize>,
    design: &str,
) -> usize {
    if let Some(v) = cache.get(design) {
        return *v;
    }

    if design.len() <= 0 {
        return 1;
    }

    let mut count = 0;

    for i in 0..design.len() + 1 {
        if patterns.contains(&&design[..i]) {
            count += num_valid_designs(patterns, cache, &design[i..]);
        }
    }

    cache.insert(design.to_owned(), count);
    count
}

fn main() {
    let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents of INPUT file");

    let (patterns, designs) = contents.split_once("\n\n").unwrap();

    let patterns: Vec<&str> = patterns
        .split(",")
        .map(|s| {
            if let Some(new_s) = s.strip_prefix(" ") {
                new_s
            } else {
                s
            }
        })
        .collect();

    let designs: Vec<&str> = designs.split("\n").filter(|l| l.len() > 0).collect();

    let mut cache1: HashMap<String, bool> = HashMap::new();
    let mut cache2: HashMap<String, usize> = HashMap::new();

    let part1_count: usize = designs
        .iter()
        .map(|d| is_valid_design(&patterns, &mut cache1, d))
        .filter(|b| *b)
        .count();
    let part2_count: usize = designs
        .iter()
        .map(|d| num_valid_designs(&patterns, &mut cache2, d))
        .sum();

    println!("Part 1 - Num Ok: {:?}", part1_count);
    println!("Part 2 - Variations: {:?}", part2_count);
}
