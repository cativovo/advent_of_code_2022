use day_six::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}
