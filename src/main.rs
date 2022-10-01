mod args;

use args::HolidayArgs;
use args::{HolidaySubcommand, HolidaysArgs, KeyArgs};
use clap::Parser;
use holidayapi_rust::HolidayAPI;
use serde::{Deserialize, Serialize};
use std::process;

#[derive(Serialize, Deserialize)]
struct MyConfig {
    api_key: Option<String>,
}
const APP_NAME: &str = "holidayapi-cli";

impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self { api_key: None }
    }
}
/// Testing about
#[tokio::main]
async fn main() {
    let args = HolidayArgs::parse();
    let cfg = confy::load::<MyConfig>(APP_NAME, None).expect("Config file didn't load properly:");

    dbg!(&args);
    match args.command {
        HolidaySubcommand::Key(cmd) => {
            handle_key_cmd(cmd, cfg);
        }
        HolidaySubcommand::Holiday(cmd) => {
            handle_holidays_cmd(cmd, cfg).await;
        }
    }
}
fn is_valid_key(key: &str) {
    if let Err(err) = HolidayAPI::is_valid_key(key) {
        println!("{} is not a valid key", key);
        println!("{}", err);
        process::exit(1);
    }
}

async fn handle_holidays_cmd(cmd: HolidaysArgs, cfg: MyConfig) {
    match cfg.api_key {
        Some(key) => {
            let api = HolidayAPI::new(&key).expect("Error");
            let mut req = api.holidays(&cmd.country, cmd.year);
            if let Some(month) = cmd.month {
                req.month(month);
            }
            if let Some(day) = cmd.day {
                req.day(day);
            }

            if cmd.public {
                req.public(cmd.public);
            }
            if cmd.subdivisions {
                req.subdivisions(cmd.subdivisions);
            }
            if let Some(search) = cmd.search {
                req.search(&search);
            }
            if let Some(language) = cmd.language {
                req.language(&language);
            }
            if cmd.previous {
                req.previous(cmd.previous);
            }
            if cmd.upcoming {
                req.upcoming(cmd.upcoming);
            }
            if cmd.pretty {
                req.pretty(cmd.pretty);
            }

            println!("Do holiday request");

            println!("{}", req.get_raw().await.expect("Error"));
        }
        None => {
            println!("Please provide api key with argument -k, --key <KEY>");
            process::exit(1);
        }
    }
}

fn handle_key_cmd(cmd: KeyArgs, mut cfg: MyConfig) {
    match cmd.key {
        Some(new_key) => {
            is_valid_key(&new_key);
            println!("Api key set to: {}", &new_key);
            cfg.api_key = Some(new_key);
            confy::store(APP_NAME, None, cfg).expect("Config failed to save:");
        }
        None => {
            match cfg.api_key {
                Some(key) => println!("Current key: {}", key),
                None => println!("Please provide api key with argument -k, --key <KEY>"),
            }
            // TODO: print current key, or notify missing key
        }
    }
}
