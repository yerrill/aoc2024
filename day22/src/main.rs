use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

fn calculate_secret(seed: usize, iterations: usize) -> usize {
    let mut result: usize = seed;

    for _ in 0..iterations {
        result = ((result * 64) ^ result) % 16777216;
        result = ((result / 32) ^ result) % 16777216;
        result = ((result * 2048) ^ result) % 16777216;
    }

    result
}

fn price_sequence(seed: usize, iterations: usize) -> Vec<(usize, isize)> {
    let mut sequence: Vec<(usize, isize)> = Vec::new();
    let mut secret: usize = seed;

    sequence.push((secret % 10, 0));

    let mut last_price: usize = secret % 10;
    let mut new_price: usize;

    for _ in 1..iterations {
        secret = ((secret * 64) ^ secret) % 16777216;
        secret = ((secret / 32) ^ secret) % 16777216;
        secret = ((secret * 2048) ^ secret) % 16777216;

        new_price = secret % 10;

        sequence.push((new_price, new_price as isize - last_price as isize));
        last_price = new_price;
    }

    sequence
}

fn mk_banana_map(
    arr: &Vec<Vec<(usize, isize)>>,
) -> Vec<HashMap<(isize, isize, isize, isize), usize>> {
    let mut maps: Vec<HashMap<(isize, isize, isize, isize), usize>> =
        vec![HashMap::new(); arr.len()];

    let mut current_subsequence: (isize, isize, isize, isize);
    let mut current_bananas: usize;

    for row in 0..arr.len() {
        for col in 4..arr[row].len() {
            current_subsequence = (
                arr[row][col - 3].1,
                arr[row][col - 2].1,
                arr[row][col - 1].1,
                arr[row][col].1,
            );
            current_bananas = arr[row][col].0;

            // Will sell at first instance of sequence, do not add for subsequent occurrances
            if let None = maps[row].get(&current_subsequence) {
                maps[row].insert(current_subsequence, current_bananas);
            }
        }
    }

    maps
}

fn banana_map_reduce(
    banana_map: &Vec<HashMap<(isize, isize, isize, isize), usize>>,
) -> ((isize, isize, isize, isize), usize) {
    let mut reduced: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();

    for row in 0..banana_map.len() {
        for (k, v) in banana_map[row].iter() {
            *reduced.entry(*k).or_insert(0) += v;
        }
    }

    let (&seq, &val) = reduced.iter().max_by(|(_, &a), (_, b)| a.cmp(b)).unwrap();

    (seq, val)
}

fn main() {
    let nums = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        contents
            .split("\n")
            .filter(|l| l.len() > 0)
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    };

    let mut acc: usize = 0;

    for num in nums.iter() {
        acc += calculate_secret(*num, 2000);
    }

    println!("Part 1 - Sum: {:?}", acc);

    let all_squences: Vec<Vec<(usize, isize)>> =
        nums.iter().map(|n| price_sequence(*n, 2000)).collect();

    let banana_map = mk_banana_map(&all_squences);
    let sequence = banana_map_reduce(&banana_map);

    println!(
        "Part 2 - Subsequence: {:?}, Max Bananas: {:?}",
        sequence.0, sequence.1
    );
}
