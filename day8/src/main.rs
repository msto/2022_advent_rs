fn main() {
    if let Err(e) = day8::get_args().and_then(day8::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
