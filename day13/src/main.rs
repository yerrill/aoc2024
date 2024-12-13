use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";

type XY = (f64, f64);
type AB = (f64, f64);

#[derive(Debug)]
struct ClawMachine {
    button_a: XY,
    button_b: XY,
    prize: XY,
}

fn solve_system(machine: &ClawMachine) -> AB {
    let x = machine.prize.0;
    let y = machine.prize.1;
    let a_x = machine.button_a.0;
    let a_y = machine.button_a.1;
    let b_x = machine.button_b.0;
    let b_y = machine.button_b.1;

    let a = |b| (x - b_x * b) / a_x;

    let b = (a_x * y - a_y * x) / (-1.0 * a_y * b_x + a_x * b_y);

    (a(b), b)
}

fn is_solvable((a, b): AB) -> bool {
    a.fract() == 0.0 && b.fract() == 0.0
}

fn main() {
    let result: Vec<ClawMachine> = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut machines: Vec<ClawMachine> = Vec::new();

        let re_button_a =
            Regex::new(r"Button A: X\+([0-9]+), Y\+([0-9]+)").expect("re_button_a failed");
        let re_button_b =
            Regex::new(r"Button B: X\+([0-9]+), Y\+([0-9]+)").expect("re_button_b failed");
        let re_prize = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").expect("re_prize failed");

        for hay in contents.split("\n\n") {
            let button_a = re_button_a
                .captures(hay)
                .expect("Failed to capture button a");
            let button_b = re_button_b
                .captures(hay)
                .expect("Failed to capture button b");
            let prize = re_prize.captures(hay).expect("Failed to capture prize");

            machines.push(ClawMachine {
                button_a: (
                    button_a[1].parse().expect("failed to parse button a x"),
                    button_a[2].parse().expect("failed to parse button a y"),
                ),
                button_b: (
                    button_b[1].parse().expect("failed to parse button b x"),
                    button_b[2].parse().expect("failed to parse button b y"),
                ),
                prize: (
                    prize[1].parse().expect("failed to parse prize x"),
                    prize[2].parse().expect("failed to parse prize y"),
                ),
            });
        }

        machines
    };

    let mut part1_acc = 0.0;
    let mut part2_acc = 0.0;

    for c in result {
        let (sol_a, sol_b) = solve_system(&c);

        if is_solvable((sol_a, sol_b)) {
            part1_acc += sol_a * 3.0 + sol_b * 1.0;
        }

        let modified = ClawMachine {
            prize: (c.prize.0 + 10000000000000.0, c.prize.1 + 10000000000000.0),
            ..c
        };

        let (sol_a, sol_b) = solve_system(&modified);

        if is_solvable((sol_a, sol_b)) {
            part2_acc += sol_a * 3.0 + sol_b * 1.0;
        }
    }

    println!("Part 1 - {:?}", part1_acc);
    println!("Part 2 - {:?}", part2_acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(ClawMachine, bool); 8] = [
        (
            ClawMachine {
                button_a: (94.0, 34.0),
                button_b: (22.0, 67.0),
                prize: (8400.0, 5400.0),
            },
            true,
        ),
        (
            ClawMachine {
                button_a: (26.0, 66.0),
                button_b: (67.0, 21.0),
                prize: (12748.0, 12176.0),
            },
            false,
        ),
        (
            ClawMachine {
                button_a: (17.0, 86.0),
                button_b: (84.0, 37.0),
                prize: (7870.0, 6450.0),
            },
            true,
        ),
        (
            ClawMachine {
                button_a: (69.0, 23.0),
                button_b: (27.0, 71.0),
                prize: (18641.0, 10279.0),
            },
            false,
        ),
        (
            ClawMachine {
                button_a: (94.0, 34.0),
                button_b: (22.0, 67.0),
                prize: (10000000008400.0, 10000000005400.0),
            },
            false,
        ),
        (
            ClawMachine {
                button_a: (26.0, 66.0),
                button_b: (67.0, 21.0),
                prize: (10000000012748.0, 10000000012176.0),
            },
            true,
        ),
        (
            ClawMachine {
                button_a: (17.0, 86.0),
                button_b: (84.0, 37.0),
                prize: (10000000007870.0, 10000000006450.0),
            },
            false,
        ),
        (
            ClawMachine {
                button_a: (69.0, 23.0),
                button_b: (27.0, 71.0),
                prize: (10000000018641.0, 10000000010279.0),
            },
            true,
        ),
    ];

    #[test]
    fn it_works() {
        for (index, (machine, result)) in CASES.iter().enumerate() {
            assert_eq!(
                is_solvable(solve_system(machine)),
                *result,
                "{:?} - (a, b) = {:?}",
                index,
                solve_system(machine)
            );
        }
    }
}
