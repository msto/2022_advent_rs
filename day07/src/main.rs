fn main() {
    if let Err(e) = day7::get_args().and_then(day7::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
