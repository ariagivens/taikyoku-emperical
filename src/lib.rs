use std::fmt::Display;

use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Simulation {
    step1_points: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Square {
    Empty,
    Friendly,
    Opponent,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Square::Empty => ' ',
            Square::Friendly => 'F',
            Square::Opponent => 'O'
        };
        write!(f, "{}", c)
    }
}

const BOARD_WIDTH: usize = 36;
const BOARD_HEIGHT: usize = 36;

#[derive(Clone)]
struct Grid {
    squares: [Square; BOARD_WIDTH * BOARD_HEIGHT],
}

impl Grid {
    fn new() -> Self {
        Grid {
            squares: [Square::Empty; BOARD_WIDTH * BOARD_HEIGHT],
        }
    }

    fn get(&self, x: usize, y: usize) -> Square {
        self.squares[y * BOARD_WIDTH + x]
    }

    fn set(&mut self, square: Square, x: usize, y: usize) {
        self.squares[y * BOARD_WIDTH + x] = square;
    }

    // fn with(&self, square: Square, x: usize, y:usize) -> Self {
    //     let mut grid = self.clone();
    //     grid.set(square, x, y);
    //     grid
    // }

    fn randomly_place(&mut self, rng: &mut impl Rng, square: Square) {
        let x = rng.gen_range(0..BOARD_WIDTH);
        let y = rng.gen_range(0..BOARD_HEIGHT);
        if self.get(x, y) == Square::Empty {
            self.set(square, x, y);
        } else {
            self.randomly_place(rng, square);
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                write!(f, "|{}", self.get(x, y))?;
            }
            write!(f, "|\n")?;
        }

        Ok(())
    }
}

pub fn simulate() -> Simulation {
    let mut rng = thread_rng();

    let num_pieces = rng.gen_range(3..804);
    let num_friendlies = rng.gen_range(1..i32::min(num_pieces - 1, 402));
    let num_pieces = num_friendlies + i32::min(402, num_pieces - num_friendlies);
    let mut grid = Grid::new();

    for i in 0..num_pieces {
        let square = if i < num_friendlies {
            Square::Friendly
        } else {
            Square::Opponent
        };

        grid.randomly_place(&mut rng, square);
    }

    //println!("{}", grid);

    let mut step1_points = 0;

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            if y > 0 && grid.get(x, y - 1) != Square::Friendly {
                step1_points += 1;
            }
        }
    }

    Simulation { step1_points }
}
