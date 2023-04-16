fn main() {
    if let Err(e) = day9::get_args().and_then(day9::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
