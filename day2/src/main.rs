use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

const INPUT: &str = "input.txt";

fn report_good(report: &Vec<i32>) -> bool {
    // Create tuples of pairs [1, 2, 3] [(1,2), (2, 3)]
    let pairs = zip(report.iter(), report.iter().skip(1)).collect::<Vec<_>>();

    // Check increasing and decreasing cases
    let increasing = pairs.iter().map(|(&a,&b)| if (b - a >= 1) && (b - a <= 3) { true } else { false } ).all(|e| e == true);
    let decreasing = pairs.iter().map(|(&a,&b)| if (a - b >= 1) && (a - b <= 3) { true } else { false } ).all(|e| e == true);

    increasing || decreasing
}

fn report_dampener_good(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let modified_report = [&report[..i], &report[i+1..]].concat();
        if report_good(&modified_report) {
            return true;
        }
    }

    false
}

fn report_safety(reports: &Vec<Vec<i32>>) -> i32 {

    let score: i32 = reports
        .iter()
        .filter(|l| l.len() > 0)
        .map(|line| report_good(line))
        .fold(0i32, |acc, e| if e { acc + 1 } else { acc } );

    score
}

fn report_safety_dampener(reports: &Vec<Vec<i32>>) -> i32 {

    let score: i32 = reports
        .iter()
        .filter(|l| l.len() > 0)
        .map(|line| report_good(line) || report_dampener_good(line))
        .fold(0i32, |acc, e| if e { acc + 1 } else { acc } );

    score
}

fn main() {
    let mut file: File = File::open(INPUT).expect("Failed to open file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents");

    let input_lines: Vec<&str> = contents.split("\n").collect::<Vec<_>>();

    let reports: Vec<Vec<i32>> = input_lines
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let safety = report_safety(&reports);
    let safety_with_dampener = report_safety_dampener(&reports);

    println!("Part 1 - Report Safety: {}", safety);
    println!("Part 2 - Report Safety With Dampener: {}", safety_with_dampener);
}
