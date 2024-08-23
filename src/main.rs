use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Number(u8),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const SIZE: usize = 3;

#[derive(Debug, Clone, Copy)]
struct Puzzle {
    grid: [[Tile; SIZE]; SIZE],
    empty_pos: (usize, usize), // Position of the empty tile
}

impl Puzzle {
    // Initialize the puzzle
    fn new() -> Self {
        let mut grid = [[Tile::Empty as Tile; SIZE]; SIZE];
        for row in 0..SIZE {
            for col in 0..SIZE {
                let val = col + (row * SIZE) + 1;
                grid[row][col] = Tile::Number(val as u8);
            }
        }
        let limit = SIZE - 1;
        let empty_pos = (limit, limit);
        grid[limit][limit] = Tile::Empty;
        Puzzle { grid, empty_pos }
    }

    fn uniq(&self) -> u64 {
        let mut res: u64 = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                let val = match self.grid[row][col] {
                    Tile::Number(n) => n,
                    Tile::Empty => 0
                };
                res *= 16;
                res += val as u64;
            }
        }
        res
    }

    fn scramble(&mut self) -> () {
        let mut rng = rand::thread_rng();

        for row in 0..SIZE {
            for col in 0..SIZE {
                let r: usize = rng.gen::<usize>() % SIZE;
                let c: usize = rng.gen::<usize>() % SIZE;

                self.swap((row, col), (r, c));

                if self.grid[row][col] == Tile::Empty {
                    self.empty_pos = (row, col);
                } else if self.grid[r][c] == Tile::Empty {
                    self.empty_pos = (r, c);
                }
            }
        }
    }

    // Check if the puzzle is solved
    fn is_solved(&self) -> bool {
        let mut expected = 1;
        for row in 0..SIZE {
            for col in 0..SIZE {
                let actual = self.grid[row][col];
                if actual
                    != if expected < SIZE * SIZE {
                        Tile::Number(expected as u8)
                    } else {
                        Tile::Empty
                    }
                {
                    return false;
                }
                expected += 1;
            }
        }
        true
    }

    fn swap(&mut self, pt_a: (usize, usize), pt_b: (usize, usize)) -> () {
        let tmp = self.grid[pt_a.0][pt_a.1];
        self.grid[pt_a.0][pt_a.1] = self.grid[pt_b.0][pt_b.1];
        self.grid[pt_b.0][pt_b.1] = tmp;
    }

    fn swap_with_empty(&mut self, pt: (usize, usize)) -> bool {
        self.swap(pt, self.empty_pos);
        self.empty_pos = pt;

        return true;
    }

    // Implement sliding moves
    fn slide(&mut self, direction: Direction) -> bool {
        // Implement sliding logic and update `empty_pos`
        // Return true if the move was successful
        match direction {
            Direction::Up => {
                self.empty_pos.0 > 0
                    && self.swap_with_empty((self.empty_pos.0 - 1, self.empty_pos.1))
            }
            Direction::Down => {
                self.empty_pos.0 + 1 < SIZE
                    && self.swap_with_empty((self.empty_pos.0 + 1, self.empty_pos.1))
            }
            Direction::Left => {
                self.empty_pos.1 > 0
                    && self.swap_with_empty((self.empty_pos.0, self.empty_pos.1 - 1))
            }
            Direction::Right => {
                self.empty_pos.1 + 1 < SIZE
                    && self.swap_with_empty((self.empty_pos.0, self.empty_pos.1 + 1))
            }
        }
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..SIZE {
            for col in 0..SIZE {
                let _ = match self.grid[row][col] {
                    Tile::Number(x) => write!(f, "{x:2} "),
                    Tile::Empty => write!(f, " - ")
                };
            }
            let _ = write!(f, "\n");
        }

        Ok(())
    }
}

fn invert(d:Direction) -> Direction {
    match d {
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down
    }
}

fn main() {
    let mut visited = HashSet::new();

    let mut p1: Puzzle = Puzzle::new();
    assert!(p1.is_solved());

    p1.scramble();
    visited.insert(p1.uniq());

    println!("{p1}\n {}", p1.uniq());

    let dirs = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];

    let mut states = VecDeque::new();
    states.push_back(p1);

    while states.len() > 0 {
        let mut p = states.pop_front().unwrap();
        for d in dirs {
            let slid = p.slide(d);

            if slid {
                let u = p.uniq();
                if !visited.contains(&u) {
                    visited.insert(p.uniq());

                    if p.is_solved() {
                        break;
                    } else {
                        let np = p.clone();
                        states.push_back(np);
                    }

                    println!("{:?}\n{p}\n{}", d, p.uniq());
                }

                p.slide(invert(d));
            }
        }
    }
}
