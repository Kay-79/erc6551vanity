use std::env;
use std::process;
use erc6551vanity::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = erc6551vanity::cpu(config) {
        eprintln!("CPU application error: {e}");
        process::exit(1);
    }
}
