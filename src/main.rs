use holidayapi_rust::*;
use std::env;

fn get_api() -> HolidayAPI {
    let args = env::args().collect::<Vec<String>>();
    if let Some(key) = args.get(1) {
        match HolidayAPI::new(key) {
            Ok(api) => return api,
            Err(err) => panic!("{}", err),
        }
    } else {
        [
            "HolidayAPI's command line interface, written in Rust.",
            "\nUSAGE: holiday [COMMAND] [OPTIONS]",
            "\nCOMMAND:",
            "    key, k          register or update a key",
            "    holidays, h     make a request to holidays endpoint",
            "    countries, c    make a request to countries endpoint",
            "    languages, l    make a request to languages endpoint",
            "    workday, w      make a request to workday endpoint",
            "    workdays, ws    make a request to workdays endpoint",
            "\nSee 'holiday help <command>' for more information.",
        ]
        .iter()
        .for_each(|s| println!("{}", s));
        quit::with_code(0);
    }
}

#[quit::main]
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let api = get_api();

    let mut line = String::new();
    println!("Enter your name :");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
}
