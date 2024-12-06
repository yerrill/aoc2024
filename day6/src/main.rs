use std::fmt;
use std::fs::File;
use std::io::prelude::*;

const INPUT: &str = "input.txt";
const ROWS: usize = 130;
const COLS: usize = 130;
const ITERATION_SAFETY: usize = 10000;
const STARTING_VECTOR: (isize, isize) = (-1, 0);

struct Obstacle([bool; 4]);

impl Copy for Obstacle { }

impl Clone for Obstacle {
    fn clone(&self) -> Self {
        Obstacle(self.0.clone())
    }
}

impl Obstacle {
    fn new() -> Obstacle {
        Obstacle([false; 4])
    }

    fn hash(v: (isize, isize)) -> usize {
        match v {
            (-1, 0) => 0,
            (0, 1) => 1,
            (1, 0) => 2,
            (0, -1) => 3,
            _ => panic!("Direction Resolve Failure")
        }
    }

    fn hit(&mut self, v: (isize, isize)) -> bool {
        let hit = self.0[Obstacle::hash(v)];
        self.0[Obstacle::hash(v)] = true;
        hit
    }
}

enum Tile {
    Empty,
    Visited,
    Obstacle(Obstacle),
}

impl Copy for Tile { }

impl Clone for Tile {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => f.write_str("."),
            Tile::Visited => f.write_str("X"),
            Tile::Obstacle(_) => f.write_str("#"),
        }
    }
}

enum MoveResult {
    Exited,
    Obstacle,
    ObstacleLoop,
    Ok,
}

enum GameResult {
    Exited,
    ObstacleLoop,
}

impl fmt::Debug for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::Exited => f.write_str("Exited"),
            GameResult::ObstacleLoop => f.write_str("Loop"),
        }
    }
}

struct Game {
    board: [[Tile; COLS]; ROWS],
    guard_position: (usize, usize),
    guard_vector: (isize, isize),
}

impl Game {
    fn new(string_in: String) -> Result<Self, String> {
        let mut map: [[Tile; COLS]; ROWS] = [[Tile::Empty; COLS]; ROWS];

        let mut guard_pos: Option<(usize, usize)> = None;

        for (row, row_text) in string_in.split("\n").filter(|l| l.len() > 0).enumerate() {
            for (col, val) in row_text.chars().enumerate() {
                if val == '.' {
                    map[row][col] = Tile::Empty;
                } else if val == '#' {
                    map[row][col] = Tile::Obstacle(Obstacle::new());
                } else if val == '^' {
                    guard_pos = Some((row, col));
                    map[row][col] = Tile::Visited;
                } else {
                    panic!("Unknown input value");
                }
            }
        }

        if let None = guard_pos {
            return Err("Bad Guard Position".to_string());
        }

        Ok(Self{
            board: map,
            guard_position: guard_pos.unwrap(),
            guard_vector: STARTING_VECTOR,
        })
    }

    fn step(&mut self) -> MoveResult {
        let current_pos: (isize, isize) = (isize::try_from(self.guard_position.0).unwrap(), isize::try_from(self.guard_position.1).unwrap());

        let new_row = usize::try_from(current_pos.0 + self.guard_vector.0);
        let new_col = usize::try_from(current_pos.1 + self.guard_vector.1);

        if let (Ok(r), Ok(c)) = (new_row, new_col) {
            if r >= ROWS || c >= COLS {
                return MoveResult::Exited;
            }

            if let Tile::Obstacle(mut ob) = self.board[r][c] {
                let previous_hit = ob.hit(self.guard_vector);

                if previous_hit {
                    return MoveResult::ObstacleLoop;
                }

                self.board[r][c] = Tile::Obstacle(ob);
                return MoveResult::Obstacle;
            }

            self.board[r][c] = Tile::Visited;
            self.guard_position = (r, c);
            return MoveResult::Ok;

        } else {
            return MoveResult::Exited;
        }
    }

    fn turn_right(&mut self) {
        self.guard_vector = (self.guard_vector.1, -self.guard_vector.0);
    }

    fn visited_tiles(&self) -> usize{
        self.board.iter().map(|l| l.iter().filter(|t| if let Tile::Visited = t { true } else { false }).count()).sum()
    }

    fn play(&mut self) -> GameResult {
        let mut state: MoveResult = MoveResult::Ok;
        let mut iterations = 0;

        loop {
            if let MoveResult::Exited = state {
                return GameResult::Exited;
            }
    
            state = self.step();
    
            if let MoveResult::Obstacle = state {
                self.turn_right();
            }
    
            if let MoveResult::ObstacleLoop = state {
                return GameResult::ObstacleLoop;
            }

            iterations += 1;
            if iterations > ITERATION_SAFETY {
                panic!("Iteration limit hit in Game.play");
            }
        }
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String = String::new();

        out += format!("Guard Position: {:?}, Guard Vector: {:?}", self.guard_position, self.guard_vector).as_str();
        out += "\n";

        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                out += format!("{:?}", self.board[row][col]).as_str();
            }
            out += "\n";
        }

        f.write_str(out.as_str())
    }
}

impl Clone for Game {
    fn clone(&self) -> Self {
        let mut new_board: [[Tile; COLS]; ROWS] = [[Tile::Empty; COLS]; ROWS];

        for row in 0..ROWS {
            for col in 0..COLS {
                new_board[row][col] = self.board[row][col];
            }
        }

        Game {
            board: new_board,
            guard_position: self.guard_position,
            guard_vector: self.guard_vector,
        }
    }
}

fn part_1(game: &Game) {
    let mut game: Game = game.clone();
    let _game_result: GameResult = game.play();

    //println!("{:?}\n{:?}", game, _game_result);
    println!("Part 1 - Visited Tiles: {:?}", game.visited_tiles());
}

fn part_2(game: &Game) {
    let mut variants = 0;

    for row in 0..ROWS {
        for col in 0..COLS {
            if let Tile::Empty = game.board[row][col] {

                let mut variant = game.clone();
                variant.board[row][col] = Tile::Obstacle(Obstacle::new());
                let game_result = variant.play();

                if let GameResult::ObstacleLoop = game_result {
                    variants += 1;
                    //println!("{:?}", variant);
                }
            }
        }
    }

    println!("Part 2 - Guard in loop Variants: {:?}", variants);
}

fn main() {
    let mut file: File = File::open(INPUT).expect("Failed to open INPUT file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contents of INPUT file");

    let game: Game = Game::new(contents).unwrap();

    part_1(&game);
    part_2(&game);

}