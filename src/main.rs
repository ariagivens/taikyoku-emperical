use num_format::{Locale, ToFormattedString};
use shogi_piece_values::{simulate_n_par, assign_values_to_faces};
use std::time::Instant;

fn main() {
    let n = 100_000;
    let timer = Instant::now();
    let simulation = simulate_n_par(n);
    let pawn = simulation.pawn();
    let simulation = simulation / pawn;
    let value_assignments = assign_values_to_faces(&simulation);
    println!(
        "Finished {} simulations in ~{} seconds.",
        n.to_formatted_string(&Locale::en),
        timer.elapsed().as_secs()
    );
    println!("{simulation}\n");
    println!("{value_assignments}");
}
