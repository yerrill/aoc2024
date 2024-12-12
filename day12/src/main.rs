use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";
const ROWS: usize = 140;
const COLS: usize = ROWS;
const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Farm = [[char; COLS]; ROWS];
type Checked = [[bool; COLS]; ROWS];
type Region = HashSet<(usize, usize)>;

fn _debug_arr(arr: &Farm) -> String {
    let mut result: String = String::new();

    for r in 0..ROWS {
        for c in 0..COLS {
            result += arr[r][c].to_string().as_str();
        }
        result += "\n";
    }

    result
}

struct Point(usize, usize);

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point(self.0, self.1)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Pt").field(&self.0).field(&self.1).finish()
    }
}

impl std::ops::Add<(isize, isize)> for Point {
    type Output = Option<Point>;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        let r: isize = isize::try_from(self.0).expect("Point - Failed to parse usize to isize");
        let c: isize = isize::try_from(self.1).expect("Point - Failed to parse usize to isize");

        let new_point = (r + rhs.0, c + rhs.1);

        if let (Ok(new_row), Ok(new_col)) =
            (usize::try_from(new_point.0), usize::try_from(new_point.1))
        {
            if new_row < ROWS && new_col < COLS {
                Some(Point(new_row, new_col))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Point {
    fn in_region(&self, region: &Region) -> bool {
        if let Some(_) = region.get(&(self.0, self.1)) {
            true
        } else {
            false
        }
    }
}

fn plot_search(
    farm: &Farm,
    checked: &mut Checked,
    point: Point,
    region: &mut Region,
) -> (usize, usize) {
    let current_value: char = farm[point.0][point.1];

    checked[point.0][point.1] = true;
    region.insert((point.0, point.1));

    let mut area = 1;
    let mut perimeter = 0;

    for dir in DIRS {
        if let Some(p) = point + dir {
            if farm[p.0][p.1] == current_value && !checked[p.0][p.1] {
                let (search_area, search_perimeter) = plot_search(farm, checked, p, region);
                area += search_area;
                perimeter += search_perimeter;
            } else if farm[p.0][p.1] != current_value {
                perimeter += 1;
            }
        } else {
            perimeter += 1;
        }
    }

    (area, perimeter)
}

fn count_corners(point: Point, region: &Region) -> usize {
    let option_in_region = |p: Option<Point>| {
        if let Some(val) = p {
            val.in_region(region)
        } else {
            false
        }
    };

    const DELTAS: [isize; 2] = [-1, 1];
    let mut count: usize = 0;

    for row in DELTAS {
        for col in DELTAS {
            if !option_in_region(point + (row, 0)) && !option_in_region(point + (0, col)) {
                count += 1;
            }

            if option_in_region(point + (row, 0))
                && option_in_region(point + (0, col))
                && !option_in_region(point + (row, col))
            {
                count += 1;
            }
        }
    }

    count
}

fn costs(farm: Farm) {
    let mut checked: Checked = [[false; COLS]; ROWS];
    let mut part1_acc: usize = 0;
    let mut part2_acc: usize = 0;

    for row in 0..ROWS {
        for col in 0..COLS {
            if !checked[row][col] {
                let mut region: Region = HashSet::new();
                let (area, perimeter) =
                    plot_search(&farm, &mut checked, Point(row, col), &mut region);

                let sides: usize = region
                    .iter()
                    .map(|p| count_corners(Point(p.0, p.1), &region))
                    .sum();

                part1_acc += area * perimeter;
                part2_acc += area * sides;
            }
        }
    }

    println!("Part 1 - Cost: {}", part1_acc);
    println!("Part 2 - Cost: {}", part2_acc);
}

fn main() {
    let farm: Farm = {
        let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read contents of INPUT file");

        let mut farm: Farm = [[' '; COLS]; ROWS];

        for (row, row_str) in contents.split("\n").filter(|l| l.len() > 0).enumerate() {
            for (col, ch) in row_str.chars().enumerate() {
                farm[row][col] = ch;
            }
        }

        farm
    };

    costs(farm);
}
