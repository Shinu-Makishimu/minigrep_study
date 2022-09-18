use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // Can read parameters from command line

    let conf:Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        erprintln!("App error: {}", e);
        process::exit(1);
    }
}





