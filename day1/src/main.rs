use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

const INPUT: &str = "input.txt";

fn parse_input(input_contents: String) -> Result<(Vec<usize>, Vec<usize>), ()> {
    // Split input String into lines and remove blanks
    let input_lines: Vec<&str> = input_contents
        .split("\n")
        .filter(|&s| s.len() > 0)
        .collect::<Vec<_>>();

    // Outputted left and right list of distances
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    for &line in input_lines.iter() {
        let line_pair = line.split_whitespace().collect::<Vec<_>>();

        // Lines should only have 2 numbers. One from each list.
        if line_pair.len() != 2 {
            return Err(());
        }

        let left_num = line_pair[0].parse::<usize>();
        let right_num = line_pair[1].parse::<usize>();

        if let (Ok(left), Ok(right)) = (left_num, right_num) {
            left_list.push(left);
            right_list.push(right);
        } else {
            return Err(());
        }
    }

    // Lists should be equal length and the same as input
    assert!(left_list.len() == right_list.len());
    assert!(right_list.len() == input_lines.len());

    Ok((left_list, right_list))
}

fn calculate_distance(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let mut left_sorted = left.clone();
    left_sorted.sort();
    let mut right_sorted = right.clone();
    right_sorted.sort();

    let distance: usize = zip(&left_sorted, &right_sorted)
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    distance
}

fn calculate_similarity(left: &Vec<usize>, right: &Vec<usize>) -> usize {
    let score: usize = left
        .iter()
        .map(|l| right.iter().filter(|&r| *r == *l).count() * l)
        .sum();

    score
}

fn main() -> std::io::Result<()> {
    let mut file: File = File::open(INPUT).expect("Failed to open file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Failed to read contents");

    let (left_list, right_list) = parse_input(contents).expect("Error in parsing lists");

    let distance: usize = calculate_distance(&left_list, &right_list);
    let similarity: usize = calculate_similarity(&left_list, &right_list);

    println!("Part 1 - List Distance: {}", distance.to_string().as_str());
    println!(
        "Part 2 - List Similarity: {}",
        similarity.to_string().as_str()
    );

    Ok(())
}
