use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

fn parse_input(text: String) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let rules_re = Regex::new(r"([0-9]*)\|([0-9]*)").unwrap();
    let pages_re = Regex::new(r"((?:[0-9]*,)+[0-9]*)").unwrap();
    let hay = text.as_str();

    let mut rules: Vec<(u8, u8)> = Vec::new();
    let mut page_orders: Vec<Vec<u8>> = Vec::new();

    for (_, [num1, num2]) in rules_re.captures_iter(hay).map(|c| c.extract()) {
        rules.push((num1.parse().unwrap(), num2.parse().unwrap()));
    }

    for (_, [st]) in pages_re.captures_iter(hay).map(|c| c.extract()) {
        page_orders.push(st.split(",").map(|s| s.parse::<u8>().unwrap()).collect());
    }

    assert!(
        page_orders.iter().all(|e| e.len() % 2 == 1),
        "Even lists are present..."
    );

    (rules, page_orders)
}

fn rule_holds(rule: &(u8, u8), pages: &Vec<u8>) -> Option<bool> {
    let mut left: Option<usize> = None;
    let mut right: Option<usize> = None;

    for (index, &num) in pages.iter().enumerate() {
        if num == rule.0 {
            left = Some(index);
        }

        if num == rule.1 {
            right = Some(index);
        }
    }

    if let (Some(l), Some(r)) = (left, right) {
        // If rule holds
        Some(l < r)
    } else {
        // Rule doesn't apply
        None
    }
}

fn all_rules_hold(rules: &Vec<(u8, u8)>, pages: &Vec<u8>) -> bool {
    rules.iter().all(|r| {
        if let Some(b) = rule_holds(r, pages) {
            b
        } else {
            true
        }
    })
}

fn compose_good_page(rules: &Vec<(u8, u8)>, pages: &Vec<u8>) -> Vec<u8> {
    let mut new_pages: Vec<u8> = Vec::new();
    let mut test_order;

    for val in pages {
        
        for i in 0..pages.len()+1 {
            test_order = new_pages.clone();
            test_order.insert(i, *val);

            if all_rules_hold(rules, &test_order) {
                new_pages = test_order;
                break;
            }
        }
    }

    new_pages
}

fn main() {
    let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents of INPUT file");

    let (rules, pages) = parse_input(contents);

    println!("Rules {:?}, Pages {:?}", rules.len(), pages.len());

    let valid_pages = pages
        .iter()
        .filter(|&p| all_rules_hold(&rules, p))
        .collect::<Vec<_>>();

    let accumulator: usize = valid_pages
        .iter()
        .fold(0, |acc, &e| acc + usize::from(e[e.len() / 2]));

    println!(
        "Part 1 - Valid Pages {:?}, Accumulator {:?}",
        valid_pages.len(),
        accumulator
    );

    let invalid_pages = pages
        .iter()
        .filter(|&p| !all_rules_hold(&rules, p))
        .collect::<Vec<_>>();

    let new_good_pages = invalid_pages.iter()
        .map(|p| compose_good_page(&rules, p))
        .collect::<Vec<_>>();

    let accumulator = new_good_pages.iter().fold(0, |acc, e| acc + usize::from(e[e.len() / 2]));

    println!(
        "Part 2 - Invalid Pages {:?}, Accumulator {:?}",
        invalid_pages.len(),
        accumulator
    );
}
