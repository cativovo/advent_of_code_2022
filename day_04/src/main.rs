use day_four::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}
