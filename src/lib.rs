mod face;
mod simulate;

use std::fmt::Display;

pub use simulate::{simulate_n, simulate_n_par, Simulation};

use face::{faces, Face, Move};

pub struct ValueAssignments(Vec<(Face, OpaqueNumber)>);

pub struct OpaqueNumber(u64);

impl Display for OpaqueNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let OpaqueNumber(n) = self;
        
        write!(f, "{}", n / 2)?;
        if n % 2 == 1 {
            write!(f, ".5")?;
        }
        
        Ok(())
    }
}

fn round_to_nearest_half(x: f64) -> OpaqueNumber {
    OpaqueNumber((2.0 * x).round() as u64)
}

impl Display for ValueAssignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ValueAssignments(assignments) = self;
        
        writeln!(f, "Value Assignments:")?;
        for (face, points) in assignments {
            writeln!(f, "\t{}: {}", face.name, points)?;
        }

        Ok(())
    }
}

pub fn assign_values_to_faces(simulation: &Simulation) -> ValueAssignments {
    let assignments = faces()
        .into_iter()
        .map(|face| {
            let value = round_to_nearest_half(assign_value_to_face(&simulation, &face));
            (face, value)
        })
        .collect();
    ValueAssignments(assignments)
}

pub fn assign_value_to_face(simulation: &Simulation, face: &Face) -> f64 {
    face.moves
        .iter()
        .map(|m| assign_value_to_move(simulation, m))
        .sum()
}

pub fn assign_value_to_move(simulation: &Simulation, m: &Move) -> f64 {
    match m {
        Move::Step(n, direction) if direction.is_orthogonal() => {
            simulation.orthogonal_steps[*n as usize]
        }
        Move::Step(n, _) => simulation.diagonal_steps[*n as usize],
        Move::Range(direction) if direction.is_orthogonal() => simulation.orthogonal_range,
        Move::Range(_) => simulation.diagonal_range,
        Move::Jump(n, direction) if direction.is_orthogonal() => {
            simulation.orthogonal_jumps[*n as usize]
        }
        Move::Jump(n, _) => simulation.diagonal_jumps[*n as usize],
        Move::Dove => simulation.dove,
        Move::KnightForward => simulation.knight_jump * 2.0,
        Move::KnightBackward => simulation.knight_jump * 2.0,
        Move::Hook(direction) if direction.is_orthogonal() => simulation.orthogonal_hook,
        Move::Hook(_) => simulation.diagonal_hook,
        Move::FullLion => simulation.full_lion,
        Move::LimitedLion => simulation.limited_lion,
        Move::JumpThenRange(n, direction) if direction.is_orthogonal() => {
            simulation.orthogonal_jump_then_range
        }
        Move::JumpThenRange(n, _) => simulation.diagonal_jump_then_range,
        Move::FlyingJump(direction) if direction.is_orthogonal() => {
            simulation.orthogonal_flying_jump
        }
        Move::FlyingJump(_) => simulation.diagonal_flying_jump,
        Move::FlyingCapture(direction) if direction.is_orthogonal() => {
            simulation.orthogonal_flying_capture
        }
        Move::FlyingCapture(_) => simulation.diagonal_flying_capture,
        Move::JumpOrRange(n, direction) if direction.is_orthogonal() => {
            simulation.orthogonal_jumps[*n as usize] + simulation.orthogonal_range
        }
        Move::JumpOrRange(n, _) => {
            simulation.diagonal_jumps[*n as usize] + simulation.diagonal_range
        }
    }
}
