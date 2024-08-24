use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Number(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const ALL_DIRS:[Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

fn invert(d: Direction) -> Direction {
    match d {
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
    }
}

const SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Puzzle {
    grid: [[Tile; SIZE]; SIZE],
    empty_pos: (usize, usize),
    cost: usize,
}

const LIMIT: usize = SIZE - 1;
const MAX: usize = SIZE * SIZE;

fn solved_pos(n: usize) -> (usize, usize) {
    ((n - 1) / SIZE, (n - 1) % SIZE)
}

impl Puzzle {
    // Initialize the puzzle
    fn new() -> Self {
        let mut grid = [[Tile::Empty as Tile; SIZE]; SIZE];
        for i in 1..MAX {
            let (row, col) = solved_pos(i);
            grid[row][col] = Tile::Number(i as u8);
        }
        let empty_pos = (LIMIT, LIMIT);
        grid[LIMIT][LIMIT] = Tile::Empty;
        let cost = 0; // always zero for a solved state
        Puzzle {
            grid,
            empty_pos,
            cost,
        }
    }

    fn uniq(&self) -> u64 {
        let mut res: u64 = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                let val = match self.grid[row][col] {
                    Tile::Number(n) => n,
                    Tile::Empty => 0,
                };
                res *= 16;
                res += val as u64;
            }
        }
        res
    }

    fn scramble(&mut self, amount:u8) -> () {
        let mut rng = rand::thread_rng();
        let mut previous = None;

        let mut moves = 0;
        while moves < amount {
            let di: usize = rng.gen::<usize>() % ALL_DIRS.len();
            let d = ALL_DIRS[di];

            if Some(d) != previous && self.slide(d) {
                previous = Some(d);
                moves += 1;
            }
        }
    }

    // Check if the puzzle is solved
    fn is_solved(&self) -> bool {
        self.cost == 0
    }

    fn compute_cost(&self) -> usize {
        let mut cost = 0;
        for row in 0..SIZE {
            for col in 0..SIZE {
                let actual = self.grid[row][col];
                let (grow, gcol) = match actual {
                    Tile::Number(n) => solved_pos(n as usize),
                    Tile::Empty => (LIMIT, LIMIT),
                };

                let diff = row.abs_diff(grow) + gcol.abs_diff(col);

                cost += diff;
            }
        }
        cost
    }

    fn swap(&mut self, pt_a: (usize, usize), pt_b: (usize, usize)) -> () {
        let tmp = self.grid[pt_a.0][pt_a.1];
        self.grid[pt_a.0][pt_a.1] = self.grid[pt_b.0][pt_b.1];
        self.grid[pt_b.0][pt_b.1] = tmp;

        self.cost = self.compute_cost(); // TODO: could probably do this incrementally
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
                    Tile::Empty => write!(f, " - "),
                };
            }
            let _ = write!(f, "\n");
        }

        Ok(())
    }
}

impl Ord for Puzzle {
    fn cmp(&self, other: &Self) -> Ordering {
        // invert ordering and use uniq id to break ties
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.uniq().cmp(&other.uniq()))
    }
}

impl PartialOrd for Puzzle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Output format idea:
//
// digraph structs {
//     node [shape=record];
//     s410141 [label="{1|4|7}|{2|5|8}|{3|6|}}"color="blue"];
//     s410142 [label="{1|4|7}|{2|5|}|{3|6|8}}"];
//     s410141 -> s410142
//     s410142 -> s410141
// }

fn main() {
    let mut visited = HashSet::new();

    let mut p1: Puzzle = Puzzle::new();
    assert!(p1.cost == p1.compute_cost());

    p1.scramble(255);
    visited.insert(p1.uniq());

    println!("{p1}\n {} ({})", p1.uniq(), p1.cost);

    let mut states = BinaryHeap::new();
    states.push(p1);

    while states.len() > 0 {
        let mut p = states.pop().unwrap();

        let from = p.uniq();
        println!("popped: {from}");

        for d in ALL_DIRS {
            let slid = p.slide(d);

            if slid {
                let to = p.uniq();
                if !visited.contains(&to) {
                    visited.insert(p.uniq());

                    let solved = p.is_solved();
                    if !solved {
                        let np = p.clone();
                        states.push(np);
                    }

                    println!("{from} + {:?} -> {to} ({})\n{p}", d, p.cost);
                    if solved {
                        println!("solved!");
                        return;
                    }
                }

                assert!(p.slide(invert(d)));
            }
        }
    }
}
