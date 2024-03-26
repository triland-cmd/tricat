fn main() {
    if let Err(e) = tricat::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
