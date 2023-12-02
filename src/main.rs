use clap::Parser;
use std::process;

use calander::Date;

/// Calander for terminal by magicisyou
#[derive(Parser)]
#[command(version)]
struct Config {
    /// Month
    month: Option<u32>,
    /// Year
    year: Option<u32>,
}

impl Config {
    pub fn extract(&self) -> Result<Option<Date>, &'static str> {
        if let Some(month) = self.month {
            if let Some(year) = self.year {
                if !(1..=12).contains(&month) {
                    return Err("Month should be in range 1 to 12");
                }
                return Ok(Some(Date { month, year }));
            }
            return Err("Month and Year is required to show calander");
        }
        Ok(None)
    }
}

fn main() {
    let config = Config::parse();
    let date = config.extract();
    match date {
        Ok(d) => {
            if let Err(e) = calander::run(d) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
        Err(e) => eprintln!("Error: {e}"),
    }
}
