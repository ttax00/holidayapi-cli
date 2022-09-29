mod args;

use args::HolidayArgs;
use clap::Parser;

/// Testing about
fn main() {
    let args = HolidayArgs::parse();
    dbg!(args);
}
