use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";
const X_SIZE: isize = 101;
const Y_SIZE: isize = 103;
const UX_SIZE: usize = 101;
const UY_SIZE: usize = 103;
const TIME: isize = 100;

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    vector: (isize, isize),
}

fn sim_robot(robot: &Robot, time: isize) -> Robot {
    let new_x: isize = (robot.pos.0 + robot.vector.0 * time).rem_euclid(X_SIZE);
    let new_y: isize = (robot.pos.1 + robot.vector.1 * time).rem_euclid(Y_SIZE);

    Robot {
        pos: (new_x, new_y),
        ..*robot
    }
}

fn safety_factor(robots: &Vec<Robot>) -> usize {
    let mid_x: isize = X_SIZE / 2;
    let mid_y: isize = Y_SIZE / 2;

    let mut quads: [usize; 4] = [0, 0, 0, 0];

    for robot in robots {
        let x = robot.pos.0;
        let y = robot.pos.1;

        if x < mid_x && y < mid_y {
            quads[0] += 1;
        }

        if x > mid_x && y < mid_y {
            quads[1] += 1;
        }

        if x < mid_x && y > mid_y {
            quads[2] += 1;
        }

        if x > mid_x && y > mid_y {
            quads[3] += 1;
        }
    }

    quads[0] * quads[1] * quads[2] * quads[3]
}

fn display_room(robots: &Vec<Robot>) {
    let mut display: [[isize; UX_SIZE]; UY_SIZE] = [[0; UX_SIZE]; UY_SIZE];

    for robot in robots {
        let u_x: usize = usize::try_from(robot.pos.0).unwrap();
        let u_y: usize = usize::try_from(robot.pos.1).unwrap();

        display[u_y][u_x] += 1;
    }

    let mut out: String = String::new();

    for y in 0..display.len() {
        for x in 0..display[y].len() {
            if display[y][x] <= 0 {
                out += ".";
            } else {
                out += display[y][x].to_string().as_str();
            }
        }
        out += "\n";
    }

    print!("{}", out);
}

fn point_distance(robots: &Vec<Robot>) -> isize {
    let mid_x: isize = X_SIZE / 2;
    let mid_y: isize = Y_SIZE / 2;
    let mut acc: isize = 0;

    for r in robots.iter() {
        acc += (mid_x - r.pos.0).abs() + (mid_y - r.pos.1).abs();
    }

    acc
}

fn merry_search(robots: &Vec<Robot>) {
    const SEARCH_LENGTH: usize = 10000;
    const THRESHOLD: isize = 20000;

    let mut robot_step = robots.iter().map(|r| sim_robot(r, 1)).collect::<Vec<_>>();

    for num in 2..SEARCH_LENGTH {
        robot_step = robot_step
            .iter()
            .map(|r| sim_robot(r, 1))
            .collect::<Vec<_>>();

        let distance = point_distance(&robot_step);

        if distance < THRESHOLD {
            println!("t = {:?}, distance = {:?}", num, distance);

            display_room(&robot_step);
        }
    }
}

fn main() {
    let robots: Vec<Robot> = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut robots: Vec<Robot> = Vec::new();
        let re = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();

        for (_, [pos_x, pos_y, vec_x, vec_y]) in re.captures_iter(&contents).map(|c| c.extract()) {
            robots.push(Robot {
                pos: (
                    pos_x.parse().expect("Failed to parse pos_x"),
                    pos_y.parse().expect("Failed to parse pos_y"),
                ),
                vector: (
                    vec_x.parse().expect("Failed to parse vec_x"),
                    vec_y.parse().expect("Failed to parse vec_y"),
                ),
            })
        }

        robots
    };

    display_room(&robots);

    let simulated = robots
        .iter()
        .map(|r| sim_robot(r, TIME))
        .collect::<Vec<_>>();
    let part1_safety = safety_factor(&simulated);
    println!("Part 1 - Safety Score: {:?}", part1_safety);

    merry_search(&robots);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLES: [Robot; 12] = [
        Robot {
            pos: (0, 4),
            vector: (3, -3),
        },
        Robot {
            pos: (6, 3),
            vector: (-1, -3),
        },
        Robot {
            pos: (10, 3),
            vector: (-1, 2),
        },
        Robot {
            pos: (2, 0),
            vector: (2, -1),
        },
        Robot {
            pos: (0, 0),
            vector: (1, 3),
        },
        Robot {
            pos: (3, 0),
            vector: (-2, -2),
        },
        Robot {
            pos: (7, 6),
            vector: (-1, -3),
        },
        Robot {
            pos: (3, 0),
            vector: (-1, -2),
        },
        Robot {
            pos: (9, 3),
            vector: (2, 3),
        },
        Robot {
            pos: (7, 3),
            vector: (-1, 2),
        },
        Robot {
            pos: (2, 4),
            vector: (2, -3),
        },
        Robot {
            pos: (9, 5),
            vector: (-3, -3),
        },
    ];

    #[test]
    fn internal() {
        for robot in SAMPLES {
            println!("{:?}", sim_robot(&robot, TIME));
        }

        let safety = safety_factor(
            &(SAMPLES
                .iter()
                .map(|r| sim_robot(r, TIME))
                .collect::<Vec<_>>()),
        );

        assert_eq!(safety, 12);
    }
}
