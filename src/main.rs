use pico_args;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;
use std::io::Error;
use std::io::Write;
use std::process::ExitCode;

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

const ALL_DIRS: [Direction; 4] = [
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

    fn scramble(&mut self, amount: u8, seed: u64) -> () {
        let mut rng = StdRng::seed_from_u64(seed);
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
        for col in 0..SIZE {
            if col > 0 {
                write!(f, "|")?;
            }
            write!(f, "{{")?;
            for row in 0..SIZE {
                if row > 0 {
                    write!(f, "|")?;
                }
                match self.grid[row][col] {
                    Tile::Number(x) => write!(f, "{x}")?,
                    Tile::Empty => write!(f, "")?,
                };
            }
            write!(f, "}}")?;
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

struct SolutionLink {
    depth: usize,
    step: Option<(u64, Direction)>,
}

fn collect(visited: HashMap<u64, SolutionLink>, start: u64) -> Vec<(u64, Direction)> {
    let mut next = start;
    let mut path = vec![];
    loop {
        let entry = visited.get(&next);
        match entry {
            Some(link) => match link.step {
                Some(step) => {
                    next = step.0;
                    path.push(step);
                }
                None => break,
            },
            None => break,
        }
    }

    return path;
}

fn solve<T: Write>(out: &mut T, scramble: u8, seed: u64) -> Result<bool, Error> {
    let mut visited = HashMap::new();

    let mut p1: Puzzle = Puzzle::new();
    assert!(p1.cost == p1.compute_cost());

    p1.scramble(scramble, seed);
    visited.insert(
        p1.uniq(),
        SolutionLink {
            depth: 0,
            step: None,
        },
    );

    writeln!(out, "digraph structs {{")?;
    writeln!(out, "\tnode [shape=record];")?;
    writeln!(out, "\ts{} [label=\"{p1}\" color=\"blue\"];", p1.uniq())?;

    let mut states = BinaryHeap::new();
    states.push(p1);

    while states.len() > 0 {
        let mut p = states.pop().unwrap();

        let from = p.uniq();
        let link = visited.get(&from).unwrap();
        let next = link.depth + 1;

        for d in ALL_DIRS {
            let slid = p.slide(d);

            if slid {
                let to = p.uniq();
                writeln!(out, "\ts{from} -> s{to};")?;
                let entry = visited.get(&to);
                let new = entry.is_none();

                // update with shortest path if we've rediscovered a new route to a node
                if new || entry.unwrap().depth > next {
                    visited.insert(
                        to,
                        SolutionLink {
                            depth: next,
                            step: Some((from, d)),
                        },
                    );
                }

                if new {
                    let solved = p.is_solved();
                    let color = if solved { "red" } else { "black" };

                    writeln!(out, "\ts{to} [label=\"{p}\" color=\"{color}\"];")?;

                    if !solved {
                        let np = p.clone();
                        states.push(np);
                    }

                    if solved {
                        let path = collect(visited, to);
                        let mut prev = to;
                        for p in path {
                            writeln!(out, "\ts{} -> s{prev} [color=\"green\"]; // {:?}", p.0, p.1)?;
                            prev = p.0;
                        }
                        writeln!(out, "}}")?;
                        return Ok(true);
                    }
                }

                assert!(p.slide(invert(d)));
            }
        }
    }

    Ok(false)
}

#[derive(Debug)]
struct AppArgs {
    moves: u8,
    seed: u64,
}

fn parse_moves(s: &str) -> Result<u8, &'static str> {
    s.parse().map_err(|_| "moves not a number")
}

fn parse_seed(s: &str) -> Result<u64, &'static str> {
    s.parse().map_err(|_| "seed not a number")
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    let args = AppArgs {
        moves: pargs
            .opt_value_from_fn("--moves", parse_moves)?
            .unwrap_or(255),
        seed: pargs.opt_value_from_fn("--seed", parse_seed)?.unwrap_or(0),
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn main() -> ExitCode {
    let mut out = std::io::stdout().lock();
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            let _ = writeln!(out, "invalid arguments: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let res = solve(&mut out, args.moves, args.seed);

    match res {
        Ok(false) => ExitCode::FAILURE,
        Ok(true) => ExitCode::SUCCESS,
        _ => ExitCode::FAILURE,
    }
}
