use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div},
};

use rand::{thread_rng, Rng};
use rayon::{iter::repeatn, prelude::*};

#[derive(Debug, Clone)]
pub struct Simulation {
    orthogonal_steps: [f64; 8],
    diagonal_steps: [f64; 8],
    orthogonal_jumps: [f64; 4],
    diagonal_jumps: [f64; 4],
    dove: f64,
    orthogonal_range: f64,
    diagonal_range: f64,
    knight_jump: f64,
    orthogonal_flying_jump: f64,
    diagonal_flying_jump: f64,
    orthogonal_flying_capture: f64,
    diagonal_flying_capture: f64,
}

impl Simulation {
    fn new() -> Self {
        Simulation {
            orthogonal_steps: [0.0; 8],
            diagonal_steps: [0.0; 8],
            dove: 0.0,
            orthogonal_jumps: [0.0; 4],
            diagonal_jumps: [0.0; 4],
            orthogonal_range: 0.0,
            diagonal_range: 0.0,
            knight_jump: 0.0,
            orthogonal_flying_jump: 0.0,
            diagonal_flying_jump: 0.0,
            orthogonal_flying_capture: 0.0,
            diagonal_flying_capture: 0.0,
        }
    }
}

impl Add for Simulation {
    type Output = Simulation;

    fn add(mut self, rhs: Self) -> Self::Output {
        for n in 0..8 {
            self.orthogonal_steps[n] += rhs.orthogonal_steps[n];
            self.diagonal_steps[n] += rhs.diagonal_steps[n];
        }
        for n in 0..4 {
            self.orthogonal_jumps[n] += rhs.orthogonal_jumps[n];
            self.diagonal_jumps[n] += rhs.diagonal_jumps[n];
        }
        self.dove += rhs.dove;
        self.orthogonal_range += rhs.orthogonal_range;
        self.diagonal_range += rhs.diagonal_range;
        self.knight_jump += rhs.knight_jump;
        self.orthogonal_flying_jump += rhs.orthogonal_flying_jump;
        self.diagonal_flying_jump += rhs.diagonal_flying_jump;
        self.orthogonal_flying_capture += rhs.orthogonal_flying_capture;
        self.diagonal_flying_capture += rhs.diagonal_flying_capture;

        self
    }
}

impl AddAssign for Simulation {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Div<f64> for Simulation {
    type Output = Simulation;

    fn div(mut self, divisor: f64) -> Self::Output {
        for n in 0..8 {
            self.orthogonal_steps[n] /= divisor;
            self.diagonal_steps[n] /= divisor;
        }
        for n in 0..4 {
            self.orthogonal_jumps[n] /= divisor;
            self.diagonal_jumps[n] /= divisor;
        }
        self.dove /= divisor;
        self.orthogonal_range /= divisor;
        self.diagonal_range /= divisor;
        self.knight_jump /= divisor;
        self.orthogonal_flying_jump /= divisor;
        self.diagonal_flying_jump /= divisor;
        self.orthogonal_flying_capture /= divisor;
        self.diagonal_flying_capture /= divisor;

        self
    }
}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Simulation Results:")?;
        writeln!(f, "\tOrthogonal Steps:")?;
        for n in 1..8 {
            writeln!(f, "\t\t{n}: {}", self.orthogonal_steps[n])?;
        }
        writeln!(f, "\tDiagonal Steps:")?;
        for n in 1..8 {
            writeln!(f, "\t\t{n}: {}", self.diagonal_steps[n])?;
        }
        writeln!(f, "\tOrthogonal Range: {}", self.orthogonal_range)?;
        writeln!(f, "\tDiagonal Range: {}", self.diagonal_range)?;
        writeln!(f, "\tOrthogonal Jumps:")?;
        for n in 2..4 {
            writeln!(f, "\t\t{n}: {}", self.orthogonal_jumps[n])?;
        }
        writeln!(f, "\tDiagonal Jumps:")?;
        for n in 2..4 {
            writeln!(f, "\t\t{n}: {}", self.diagonal_jumps[n])?;
        }
        writeln!(f, "\tKnight-Style Jump: {}", self.knight_jump)?;
        writeln!(f, "\tDove Moves: {}", self.dove)?;
        writeln!(f, "\tOrthogonal Flying Jump: {}", self.orthogonal_flying_jump)?;
        writeln!(f, "\tDiagonal Flying Jump: {}", self.diagonal_flying_jump)?;
        writeln!(f, "\tOrthogonal Flying Capture: {}", self.orthogonal_flying_capture)?;
        writeln!(f, "\tDiagonal Flying Capture: {}", self.diagonal_flying_capture)?;
        Ok(())
    }
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
            Square::Opponent => 'O',
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

    fn get<T, U>(&self, x: T, y: U) -> Square
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

    fn set<T, U>(&mut self, square: Square, x: T, y: U)
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

pub fn simulate_n(n: usize) -> Simulation {
    let mut sim = Simulation::new();
    for _ in 0..n {
        sim += simulate();
    }
    sim / n as f64
}

pub fn simulate_n_par(n: usize) -> Simulation {
    let sim = repeatn((), n)
        .map(|()| simulate())
        .reduce(Simulation::new, Simulation::add);
    sim / n as f64
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

    let mut orthogonal_steps = [0.0; 8];
    let mut diagonal_steps = [0.0; 8];
    let mut orthogonal_jumps = [0.0; 4];
    let mut diagonal_jumps = [0.0; 4];
    let mut dove = 0.0;
    let mut orthogonal_range = 0.0;
    let mut diagonal_range = 0.0;
    let mut knight_jump = 0.0;
    let mut orthogonal_flying_jump = 0.0;
    let mut diagonal_flying_jump = 0.0;
    let mut orthogonal_flying_capture = 0.0;
    let mut diagonal_flying_capture = 0.0;

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            let x = x as i64;
            let y = y as i64;
            for n in 0..8 {
                orthogonal_steps[n] += step_n_orthogonal(&grid, x, y, n as i64) as f64 / 4.0;
                diagonal_steps[n] += step_n_diagonal(&grid, x, y, n as i64) as f64 / 4.0;
            }
            for n in 0..4 {
                orthogonal_jumps[n] += jump_n_orthogonal(&grid, x, y, n as i64) as f64 / 4.0;
                diagonal_jumps[n] += jump_n_diagonal(&grid, x, y, n as i64) as f64 / 4.0;
            }
            dove += dove_moves(&grid, x, y) as f64 / 4.0;
            orthogonal_range += range_orthogonal(&grid, x, y) as f64 / 4.0;
            diagonal_range += range_diagonal(&grid, x, y) as f64 / 4.0;
            knight_jump += jump_knight(&grid, x, y) as f64 / 2.0;
            orthogonal_flying_jump += flying_jump_orthogonal(&grid, x, y) as f64 / 4.0;
            diagonal_flying_jump += flying_jump_diagonal(&grid, x, y) as f64 / 4.0;
            orthogonal_flying_capture += flying_capture_orthogonal(&grid, x, y) as f64 / 4.0;
            diagonal_flying_capture += flying_capture_diagonal(&grid, x, y) as f64 / 4.0;
        }
    }

    Simulation {
        orthogonal_steps,
        diagonal_steps,
        orthogonal_jumps,
        diagonal_jumps,
        dove,
        orthogonal_range,
        diagonal_range,
        knight_jump,
        orthogonal_flying_jump,
        diagonal_flying_jump,
        orthogonal_flying_capture,
        diagonal_flying_capture,
    }
}

fn step_n_orthogonal(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    step_n_north(grid, x, y, n)
        + step_n_east(grid, x, y, n)
        + step_n_south(grid, x, y, n)
        + step_n_west(grid, x, y, n)
}

fn step_n_diagonal(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    step_n_northeast(grid, x, y, n)
        + step_n_southeast(grid, x, y, n)
        + step_n_southwest(grid, x, y, n)
        + step_n_northwest(grid, x, y, n)
}

fn step_n_north(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 0, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_north(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn step_n_northeast(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_northeast(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn step_n_east(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_east(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn step_n_southeast(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_southeast(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn step_n_south(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 0, y, 1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_south(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn step_n_southwest(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, -1, y, 1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_southwest(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn step_n_west(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, -1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_west(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn step_n_northwest(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, -1, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + step_n_northwest(grid, xp, yp, n - 1),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn jump_n_orthogonal(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    jump_n_north(grid, x, y, n)
        + jump_n_east(grid, x, y, n)
        + jump_n_south(grid, x, y, n)
        + jump_n_west(grid, x, y, n)
}

fn jump_n_diagonal(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    jump_n_northeast(grid, x, y, n)
        + jump_n_southeast(grid, x, y, n)
        + jump_n_southwest(grid, x, y, n)
        + jump_n_northwest(grid, x, y, n)
}

fn try_add(x: i64, dx: i64, y: i64, dy: i64) -> Option<(i64, i64)> {
    let x = x as i64;
    let dx = dx as i64;
    let y = y as i64;
    let dy = dy as i64;
    if x + dx > 0
        && y + dy > 0
        && x + dx < (BOARD_WIDTH as i64) - 1
        && y + dy < (BOARD_HEIGHT as i64) - 1
    {
        Some(((x + dx) as i64, (y + dy) as i64))
    } else {
        None
    }
}

fn jump_n_north(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, 0, y, -n) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_n_northeast(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, n, y, -n) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_n_east(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, n, y, 0) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_n_southeast(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, n, y, n) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_n_south(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, 0, y, n) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_n_southwest(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, -n, y, n) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_n_west(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, -n, y, 0) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_n_northwest(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, -n, y, -n) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_knight(grid: &Grid, x: i64, y: i64) -> i64 {
    jump_knight_northeast(grid, x, y) + jump_knight_northwest(grid, x, y)
}

fn jump_knight_northeast(grid: &Grid, x: i64, y: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, 1, y, -2) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn jump_knight_northwest(grid: &Grid, x: i64, y: i64) -> i64 {
    if let Some((xp, yp)) = try_add(x, -1, y, -2) {
        match grid.get(xp, yp) {
            Square::Empty => 1,
            Square::Friendly => 0,
            Square::Opponent => 1,
        }
    } else {
        0
    }
}

fn dove_moves(grid: &Grid, x: i64, y: i64) -> i64 {
    dove_northeast(grid, x, y)
        + dove_southeast(grid, x, y)
        + dove_southwest(grid, x, y)
        + dove_northwest(grid, x, y)
}

fn dove_northeast(grid: &Grid, x: i64, y: i64) -> i64 {
    if y > 3 {
        step_n_northeast(grid, x + 3, y - 3, 3)
    } else {
        0
    }
}

fn dove_southeast(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_southeast(grid, x + 3, y + 3, 3)
}

fn dove_southwest(grid: &Grid, x: i64, y: i64) -> i64 {
    if x > 3 {
        step_n_southwest(grid, x - 3, y + 3, 3)
    } else {
        0
    }
}

fn dove_northwest(grid: &Grid, x: i64, y: i64) -> i64 {
    if x > 3 && y > 3 {
        step_n_west(grid, x - 3, y - 3, 3)
    } else {
        0
    }
}

fn range_orthogonal(grid: &Grid, x: i64, y: i64) -> i64 {
    range_north(grid, x, y)
        + range_east(grid, x, y)
        + range_south(grid, x, y)
        + range_west(grid, x, y)
}

fn range_diagonal(grid: &Grid, x: i64, y: i64) -> i64 {
    range_northeast(grid, x, y)
        + range_southeast(grid, x, y)
        + range_southwest(grid, x, y)
        + range_northwest(grid, x, y)
}

fn range_north(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_north(grid, x, y, BOARD_HEIGHT as i64)
}

fn range_northeast(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_northeast(grid, x, y, BOARD_HEIGHT as i64)
}

fn range_east(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_east(grid, x, y, BOARD_HEIGHT as i64)
}

fn range_southeast(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_southeast(grid, x, y, BOARD_HEIGHT as i64)
}

fn range_south(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_south(grid, x, y, BOARD_HEIGHT as i64)
}

fn range_southwest(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_southwest(grid, x, y, BOARD_HEIGHT as i64)
}

fn range_west(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_west(grid, x, y, BOARD_HEIGHT as i64)
}

fn range_northwest(grid: &Grid, x: i64, y: i64) -> i64 {
    step_n_northwest(grid, x, y, BOARD_HEIGHT as i64)
}

fn flying_jump_orthogonal(grid: &Grid, x: i64, y: i64) -> i64 {
    flying_jump_north(grid, x, y, BOARD_HEIGHT as i64, 3)
        + flying_jump_east(grid, x, y, BOARD_HEIGHT as i64, 3)
        + flying_jump_south(grid, x, y, BOARD_HEIGHT as i64, 3)
        + flying_jump_west(grid, x, y, BOARD_HEIGHT as i64, 3)
}

fn flying_jump_diagonal(grid: &Grid, x: i64, y: i64) -> i64 {
    flying_jump_northeast(grid, x, y, BOARD_HEIGHT as i64, 3)
        + flying_jump_southeast(grid, x, y, BOARD_HEIGHT as i64, 3)
        + flying_jump_southwest(grid, x, y, BOARD_HEIGHT as i64, 3)
        + flying_jump_northwest(grid, x, y, BOARD_HEIGHT as i64, 3)
}

fn flying_jump_north(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, 0, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_north(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => 1 + flying_jump_north(grid, xp, yp, n - 1, jumps - 1),
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => 1 + flying_jump_north(grid, xp, yp, n - 1, jumps - 1),
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_jump_northeast(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, 1, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_northeast(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => {
                1 + flying_jump_northeast(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => {
                1 + flying_jump_northeast(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_jump_east(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_east(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => 1 + flying_jump_east(grid, xp, yp, n - 1, jumps - 1),
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => 1 + flying_jump_east(grid, xp, yp, n - 1, jumps - 1),
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_jump_southeast(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, 1, y, 1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_southeast(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => {
                1 + flying_jump_southeast(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => {
                1 + flying_jump_southeast(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_jump_south(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, 0, y, 1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_south(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => 1 + flying_jump_south(grid, xp, yp, n - 1, jumps - 1),
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => 1 + flying_jump_south(grid, xp, yp, n - 1, jumps - 1),
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_jump_southwest(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, -1, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_southwest(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => {
                1 + flying_jump_southwest(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => {
                1 + flying_jump_southwest(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_jump_west(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, -1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_west(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => 1 + flying_jump_west(grid, xp, yp, n - 1, jumps - 1),
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => 1 + flying_jump_west(grid, xp, yp, n - 1, jumps - 1),
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_jump_northwest(grid: &Grid, x: i64, y: i64, n: i64, jumps: i64) -> i64 {
    match try_add(x, -1, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_jump_northwest(grid, xp, yp, n - 1, jumps),
            Square::Friendly if jumps > 0 => {
                1 + flying_jump_northwest(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Friendly => 0,
            Square::Opponent if jumps > 0 => {
                1 + flying_jump_northwest(grid, xp, yp, n - 1, jumps - 1)
            }
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn flying_capture_orthogonal(grid: &Grid, x: i64, y: i64) -> i64 {
    flying_capture_north(grid, x, y)
        + flying_capture_east(grid, x, y)
        + flying_capture_south(grid, x, y)
        + flying_capture_west(grid, x, y)
}

fn flying_capture_diagonal(grid: &Grid, x: i64, y: i64) -> i64 {
    flying_capture_northeast(grid, x, y)
        + flying_capture_southeast(grid, x, y)
        + flying_capture_southwest(grid, x, y)
        + flying_capture_northwest(grid, x, y)
}

fn flying_capture_north(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 0, y, -1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_north(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_north(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_north(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 0, y, -1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_north(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_north(grid, xp, yp),
        },
        _ => 0,
    }
}

fn flying_capture_northeast(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 1, y, -1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_northeast(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_northeast(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_northeast(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 1, y, -1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_northeast(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_northeast(grid, xp, yp),
        },
        _ => 0,
    }
}

fn flying_capture_east(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_east(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_east(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_east(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_east(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_east(grid, xp, yp),
        },
        _ => 0,
    }
}

fn flying_capture_southeast(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 1, y, 1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_southeast(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_southeast(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_southeast(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 1, y, 1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_southeast(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_southeast(grid, xp, yp),
        },
        _ => 0,
    }
}

fn flying_capture_south(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 0, y, 1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_south(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_south(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_south(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, 0, y, 1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_south(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_south(grid, xp, yp),
        },
        _ => 0,
    }
}

fn flying_capture_southwest(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, -1, y, 1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_southwest(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_southwest(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_southwest(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, -1, y, 1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_southwest(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_southwest(grid, xp, yp),
        },
        _ => 0,
    }
}

fn flying_capture_west(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, -1, y, 0) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_west(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_west(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_west(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, -1, y, 0) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_west(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_west(grid, xp, yp),
        },
        _ => 0,
    }
}

fn flying_capture_northwest(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, -1, y, -1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 1 + flying_capture_northwest(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_northwest(grid, xp, yp),
        },
        _ => 0,
    }
}

fn count_pieces_northwest(grid: &Grid, x: i64, y: i64) -> i64 {
    match try_add(x, -1, y, -1) {
        Some((xp, yp)) => match grid.get(xp, yp) {
            Square::Empty => 0 + count_pieces_northwest(grid, xp, yp),
            Square::Friendly | Square::Opponent => 1 + count_pieces_northwest(grid, xp, yp),
        },
        _ => 0,
    }
}

fn full_lion(grid: &Grid, x: i64, y: i64) -> i64 {
    let mut total = 0;
    for j in -2..=2 {
        for i in -2..=2 {
            if i == 0 && j == 0 {
                continue;
            }
            match try_add(x, j, y, i) {
                Some((xp, yp)) if total > 0 => match grid.get(xp, yp) {
                    Square::Empty | Square::Opponent => total += 1,
                    Square::Friendly => {},
                },
                _ => {},
            } 
        }
    }
    if total > 0 {
        total += 1;
    }
    return total;
}

fn limited_lion(grid: &Grid, x: i64, y: i64) -> i64 {
    let mut total: i64 = 0;
    for n in 1..=3 {
        total += jump_n_orthogonal(grid, x, y, n);
        total += jump_n_diagonal(grid, x, y, n);
    }
    if total > 0 {
        total += 1;
    }
    return total;
}

fn hook_north(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 0, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_north(grid, xp, yp, n-1) + step_n_east(grid, xp, yp, 36) + step_n_west(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn hook_northeast(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, -1) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_northeast(grid, xp, yp, n-1) + step_n_northwest(grid, xp, yp, 36) + step_n_southeast(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn hook_east(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_east(grid, xp, yp, n-1) + step_n_north(grid, xp, yp, 36) + step_n_south(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn hook_southeast(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_southeast(grid, xp, yp, n-1) + step_n_northeast(grid, xp, yp, 36) + step_n_southwest(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn hook_south(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_south(grid, xp, yp, n-1) + step_n_east(grid, xp, yp, 36) + step_n_west(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn hook_southwest(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_southwest(grid, xp, yp, n-1) + step_n_southeast(grid, xp, yp, 36) + step_n_northwest(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn hook_west(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_west(grid, xp, yp, n-1) + step_n_south(grid, xp, yp, 36) + step_n_north(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn hook_northwest(grid: &Grid, x: i64, y: i64, n: i64) -> i64 {
    match try_add(x, 1, y, 0) {
        Some((xp, yp)) if n > 0 => match grid.get(xp, yp) {
            Square::Empty => 1 + hook_northwest(grid, xp, yp, n-1) + step_n_southwest(grid, xp, yp, 36) + step_n_northeast(grid, xp, yp, 36),
            Square::Friendly => 0,
            Square::Opponent => 1,
        },
        _ => 0,
    }
}

fn jump_then_range_north(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_north(grid, x, y, 36);
    total += step_n_north(grid, x, y-1, 36);
    
    return total;
}

fn jump_then_range_northeast(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_northeast(grid, x, y, 36);
    total += step_n_northeast(grid, x+1, y-1, 36);
    
    return total;
}

fn jump_then_range_east(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_east(grid, x, y, 36);
    total += step_n_east(grid, x+1, y, 36);
    
    return total;
}

fn jump_then_range_southeast(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_east(grid, x, y, 36);
    total += step_n_east(grid, x+1, y+1, 36);
    
    return total;
}

fn jump_then_range_south(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_south(grid, x, y, 36);
    total += step_n_south(grid, x, y+1, 36);
    
    return total;
}

fn jump_then_range_southwest(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_southwest(grid, x, y, 36);
    total += step_n_southwest(grid, x-1, y+1, 36);
    
    return total;
}

fn jump_then_range_west(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_west(grid, x, y, 36);
    total += step_n_west(grid, x-1, y, 36);
    
    return total;
}

fn jump_then_range_northwest(grid: &Grid, x: i64, y: i64,) -> i64 {
    let mut total = 0;
    total += step_n_west(grid, x, y, 36);
    total += step_n_west(grid, x-1, y-1, 36);
    
    return total;
}
