use shogi_piece_values::simulate;

fn main() {
    let simulation = simulate();
    println!("{simulation:?}");
}
