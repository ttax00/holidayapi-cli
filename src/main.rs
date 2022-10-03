mod args;

use args::{CountriesArgs, HolidayArgs};
use args::{HolidaysArgs, KeyArgs, SubCommand};
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
        SubCommand::Key(cmd) => handle_key_cmd(cmd, cfg),
        SubCommand::Holiday(cmd) => handle_holidays_cmd(cmd, cfg).await,
        SubCommand::Country(cmd) => handle_countries_cmd(cmd, cfg).await,
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
    no_key_provided(cfg.api_key == None, cmd.key == None);
    let mut key = cfg.api_key.expect("Valid API");
    cmd.key.and_then(|k| Some(key = k)); // uses custom key when available

    let api = HolidayAPI::new(&key).expect("Error");
    let mut req = api.holidays(&cmd.country, cmd.year);

    cmd.month.and_then(|m| Some(req.month(m)));
    cmd.day.and_then(|d| Some(req.day(d)));
    cmd.public.then(|| req.public());
    cmd.subdivisions.then(|| req.subdivisions());
    cmd.search.and_then(|s| Some(req.search(&s)));
    cmd.language.and_then(|l| Some(req.language(&l)));
    cmd.previous.then(|| req.previous());
    cmd.upcoming.then(|| req.upcoming());
    req.format(&cmd.format);
    cmd.pretty.then(|| req.pretty());

    println!("{}", req.get_raw().await.expect("Error"));
}

async fn handle_countries_cmd(cmd: CountriesArgs, cfg: MyConfig) {
    no_key_provided(cfg.api_key == None, cmd.key == None);
    let mut key = cfg.api_key.expect("Valid API");
    cmd.key.and_then(|k| Some(key = k)); // uses custom key when available

    let api = HolidayAPI::new(&key).expect("Error");
    let mut req = api.countries();

    cmd.country.and_then(|c| Some(req.country(&c)));
    cmd.search.and_then(|s| Some(req.search(&s)));
    cmd.public.then(|| req.public());
    req.format(&cmd.format);
    cmd.pretty.then(|| req.pretty());

    println!("{}", req.get_raw().await.expect("Error"));
}

fn no_key_provided(key1: bool, key2: bool) {
    if key1 && key2 {
        println!("Please provide api key with argument -k, --key <KEY>");
        process::exit(1);
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
