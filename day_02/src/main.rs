use day_two::calculate_points;

fn main() {
    if let Err(err) = calculate_points() {
        eprintln!("{err}");
    }
}
