mod args;

use args::HolidayArgs;
use args::{HolidaySubcommand, HolidaysArgs, KeyArgs};
use clap::Parser;
use holidayapi_rust::HolidayAPI;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyConfig {
    api_key: Option<String>,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self { api_key: None }
    }
}
/// Testing about
fn main() {
    let args = HolidayArgs::parse();
    let _cfg = confy::load::<MyConfig>("holidayapi-cli", None);

    dbg!(&args);
    match args.command {
        HolidaySubcommand::Key(cmd) => handle_key_cmd(cmd),
        HolidaySubcommand::Holiday(cmd) => handle_holidays_cmd(cmd),
    }
}

fn handle_holidays_cmd(cmd: HolidaysArgs) {}

fn handle_key_cmd(cmd: KeyArgs) {}
