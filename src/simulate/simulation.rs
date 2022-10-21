use std::fmt::Display;
use std::ops::{Add, AddAssign, Div};

#[derive(Debug, Clone)]
pub struct Simulation {
    pub orthogonal_steps: [f64; 8],
    pub diagonal_steps: [f64; 8],
    pub orthogonal_jumps: [f64; 4],
    pub diagonal_jumps: [f64; 4],
    pub dove: f64,
    pub orthogonal_range: f64,
    pub diagonal_range: f64,
    pub knight_jump: f64,
    pub orthogonal_flying_jump: f64,
    pub diagonal_flying_jump: f64,
    pub orthogonal_flying_capture: f64,
    pub diagonal_flying_capture: f64,
    pub orthogonal_jump_then_range: f64,
    pub diagonal_jump_then_range: f64,
    pub orthogonal_hook: f64,
    pub diagonal_hook: f64,
    pub full_lion: f64,
    pub limited_lion: f64,
}

impl Simulation {
    pub fn new() -> Self {
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
            orthogonal_jump_then_range: 0.0,
            diagonal_jump_then_range: 0.0,
            orthogonal_hook: 0.0,
            diagonal_hook: 0.0,
            full_lion: 0.0,
            limited_lion: 0.0,
        }
    }

    pub fn pawn(&self) -> f64 {
        self.orthogonal_steps[1]
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
        self.orthogonal_jump_then_range += rhs.orthogonal_jump_then_range;
        self.diagonal_jump_then_range += rhs.diagonal_jump_then_range;
        self.orthogonal_hook += rhs.orthogonal_hook;
        self.diagonal_hook += rhs.diagonal_hook;
        self.full_lion += rhs.full_lion;
        self.limited_lion += rhs.limited_lion;

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
        self.orthogonal_jump_then_range /= divisor;
        self.diagonal_jump_then_range /= divisor;
        self.orthogonal_hook /= divisor;
        self.diagonal_hook /= divisor;
        self.full_lion /= divisor;
        self.limited_lion /= divisor;

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
        writeln!(
            f,
            "\tOrthogonal Jump then Range: {}",
            self.orthogonal_jump_then_range
        )?;
        writeln!(
            f,
            "\tDiagonal Jump then Range: {}",
            self.diagonal_jump_then_range
        )?;
        writeln!(
            f,
            "\tOrthogonal Flying Jump: {}",
            self.orthogonal_flying_jump
        )?;
        writeln!(f, "\tDiagonal Flying Jump: {}", self.diagonal_flying_jump)?;
        writeln!(
            f,
            "\tOrthogonal Flying Capture: {}",
            self.orthogonal_flying_capture
        )?;
        writeln!(
            f,
            "\tDiagonal Flying Capture: {}",
            self.diagonal_flying_capture
        )?;
        writeln!(f, "\tOrthogonal Hook Moves: {}", self.orthogonal_hook)?;
        writeln!(f, "\tDiagonal Hook Moves: {}", self.diagonal_hook)?;
        writeln!(f, "\tDove Moves: {}", self.dove)?;
        writeln!(f, "\tFull lion: {}", self.full_lion)?;
        writeln!(f, "\tLimited lion: {}", self.limited_lion)?;

        Ok(())
    }
}
