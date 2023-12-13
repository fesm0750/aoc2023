use std::error;
use std::{fs, str::FromStr};
use Direction::*;
use PipeKind::*;

pub fn run() {
    let mut maze: Grid = fs::read_to_string("inputs/day10").unwrap().parse().unwrap();

    // Part 01
    let distance = traverse_loop(&mut maze);
    println!("Farthest distance: {distance}");

    // Part 02
    // Find enclosed
}

/// Traverses the loop and returns the farthest point from the starting location
fn traverse_loop(maze: &mut Grid) -> usize {
    let (start_pos, start_dir) = maze.find_start();

    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut len: usize = 0;
    loop {
        len += 1;
        (pos, dir) = maze.walk(pos, dir);
        if pos == start_pos {
            break;
        }
    }

    len / 2
}

//-----
// Structs and Enums
//-----

struct Grid {
    vec: Vec<Pipe>,
    n_cols: usize,
    n_rows: usize,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pipe {
    kind: PipeKind,
    is_main_path: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PipeKind {
    Vertical,
    Horizontal,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

//-----
// Implementations
//-----

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
}

impl Pipe {
    /// Parses a pipe from a character.
    fn from_char(c: char) -> Pipe {
        let kind = match c {
            '|' => Vertical,
            '-' => Horizontal,
            'L' => NorthEastBend,
            'J' => NorthWestBend,
            '7' => SouthWestBend,
            'F' => SouthEastBend,
            '.' => Ground,
            'S' => Start,
            _ => panic!("Invalid Pipe"),
        };

        Pipe {
            kind,
            is_main_path: false,
        }
    }

    /// Returns the next direction the current pipe leads to.
    fn direct_to(&self, dir: Direction) -> Option<Direction> {
        match (self.kind, dir) {
            (Vertical, North) => Some(North),
            (Vertical, South) => Some(South),
            (Horizontal, East) => Some(East),
            (Horizontal, West) => Some(West),
            (NorthEastBend, South) => Some(East),
            (NorthEastBend, West) => Some(North),
            (NorthWestBend, South) => Some(West),
            (NorthWestBend, East) => Some(North),
            (SouthWestBend, North) => Some(West),
            (SouthWestBend, East) => Some(South),
            (SouthEastBend, North) => Some(East),
            (SouthEastBend, West) => Some(South),
            (Start, _) => Some(dir),
            (_, _) => None,
        }
    }

    /// Sets the attribute `is_main_path` as true.
    fn set_main_path(&mut self) {
        self.is_main_path = true;
    }
}

impl Grid {
    /// Returns a position from the index of the inner `Vec`.
    fn pos(&self, idx: usize) -> Position {
        Position {
            row: idx / self.n_cols,
            col: idx % self.n_rows,
        }
    }

    /// Traverse Grid looking for the starting location. Returns the position of the `start` node and a direction to
    /// a valid connection.
    fn find_start(&self) -> (Position, Direction) {
        let start = self.pos(self.vec.iter().position(|&pipe| pipe.kind == Start).unwrap());

        let north = if start.row > 0 {
            Some(Position::new(start.row - 1, start.col))
        } else {
            None
        };

        let south = if start.row < self.n_rows - 1 {
            Some(Position::new(start.row + 1, start.col))
        } else {
            None
        };

        // Checks if any pipe in one of the four directions is connected to `start`.
        // If not connected North or South, it must be connected East and West, defining default return as East in this
        // case.
        let dir = if north.is_some() && self.get(&north.unwrap()).direct_to(North).is_some() {
            North
        } else if south.is_some() && self.get(&south.unwrap()).direct_to(South).is_some() {
            South
        } else {
            East
        };

        (start, dir)
    }

    /// Returns a element of the grid from a given position.
    fn get(&self, pos: &Position) -> Pipe {
        let idx = pos.row * self.n_cols + pos.col;
        self.vec[idx]
    }

    /// Returns a mutable reference to a element of the grid from a given position.
    fn get_mut(&mut self, pos: &Position) -> &mut Pipe {
        let idx = pos.row * self.n_cols + pos.col;
        &mut self.vec[idx]
    }

    /// Returns the next position and flow direction.
    fn walk(&mut self, pos: Position, dir: Direction) -> (Position, Direction) {
        // No need to consider overflow, path is in a loop
        let new_pos = match dir {
            North => Position::new(pos.row - 1, pos.col),
            South => Position::new(pos.row + 1, pos.col),
            East => Position::new(pos.row, pos.col + 1),
            West => Position::new(pos.row, pos.col - 1),
        };

        let pipe = self.get_mut(&new_pos);
        pipe.set_main_path();
        let new_dir = pipe.direct_to(dir).unwrap();

        (new_pos, new_dir)
    }
}

impl FromStr for Grid {
    type Err = Box<dyn error::Error>;

    /// Generates a Grid of Pipes from a String.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_cols = s.split_once('\n').unwrap().0.len();
        let n_rows = s.len() / n_cols; // number of rows; +1 accounts for '\n'

        // extends borders of the grid
        let mut grid: Vec<Pipe> = Vec::with_capacity(n_cols * n_rows);
        grid.extend(s.chars().filter(|c| !c.is_ascii_whitespace()).map(Pipe::from_char));

        Ok(Grid {
            vec: grid,
            n_cols,
            n_rows,
        })
    }
}

//-----
// Tests
//-----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let mut input: Grid = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .parse()
            .unwrap();

        assert_eq!(traverse_loop(&mut input), 8);
    }
}
