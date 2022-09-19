use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let conf:Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}





