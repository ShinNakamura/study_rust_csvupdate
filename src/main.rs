fn main() {
    if let Err(err) = csvupdate::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
