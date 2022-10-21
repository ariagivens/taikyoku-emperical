use rand::Rng;
use std::fmt::{Debug, Display};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Square {
    Empty,
    Friendly,
    Opponent,
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Square::Empty => ' ',
            Square::Friendly => 'F',
            Square::Opponent => 'O',
        };
        write!(f, "{}", c)
    }
}

pub const BOARD_WIDTH: usize = 36;
pub const BOARD_HEIGHT: usize = 36;

#[derive(Clone)]
pub struct Grid {
    squares: [Square; BOARD_WIDTH * BOARD_HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            squares: [Square::Empty; BOARD_WIDTH * BOARD_HEIGHT],
        }
    }

    pub fn get<T, U>(&self, x: T, y: U) -> Square
    where
        T: TryInto<usize>,
        <T as TryInto<usize>>::Error: Debug,
        U: TryInto<usize>,
        <U as TryInto<usize>>::Error: Debug,
    {
        let x = x.try_into().unwrap();
        let y = y.try_into().unwrap();
        self.squares[y * BOARD_WIDTH + x]
    }

    pub fn set<T, U>(&mut self, square: Square, x: T, y: U)
    where
        T: TryInto<usize>,
        <T as TryInto<usize>>::Error: Debug,
        U: TryInto<usize>,
        <U as TryInto<usize>>::Error: Debug,
    {
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.squares[y * BOARD_WIDTH + x] = square;
    }

    pub fn randomly_place(&mut self, rng: &mut impl Rng, square: Square) {
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
