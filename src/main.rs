fn main() {
    if let Err(err) = mkrimod::run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
