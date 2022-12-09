use day_five::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}
