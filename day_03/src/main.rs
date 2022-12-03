use day_three::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}
