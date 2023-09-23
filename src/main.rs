use std::env;
use std::process;

use calander::Config;

fn main() {
    let config: Option<Config> = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1);
    });
    if let Err(e) = calander::run(config) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
