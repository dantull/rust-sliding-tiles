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

const SIZE: usize = 4;

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

    fn swap_with_empty(&mut self, pt: (usize, usize)) -> bool {
        let tmp = self.grid[self.empty_pos.0][self.empty_pos.1];
        self.grid[self.empty_pos.0][self.empty_pos.1] = self.grid[pt.0][pt.1];
        self.grid[pt.0][pt.1] = tmp;

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
                self.empty_pos.0 > 1
                    && self.swap_with_empty((self.empty_pos.0, self.empty_pos.1 - 1))
            }
            Direction::Right => {
                self.empty_pos.1 + 1 < SIZE
                    && self.swap_with_empty((self.empty_pos.0, self.empty_pos.1 + 1))
            }
        }
    }
}

fn main() {
    let mut p: Puzzle = Puzzle::new();
    if p.is_solved() {
        println!("solved!");
    } else {
        println!("unsolved!");
    }
    let dirs = [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ];
    for d in dirs {
        let slid = p.slide(d);
        println!("slid? {slid}");
        let solved = p.is_solved();
        println!("solved? {solved}");
        println!("{:?}", p);
    }
}
