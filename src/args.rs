use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct HolidayArgs {
    #[clap(subcommand)]
    pub command: HolidaySubcommand,
}

#[derive(Debug, Subcommand)]
pub enum HolidaySubcommand {
    /// set api key to provided or response with current key.
    Key(KeyArgs),
    Holiday(HolidaysArgs),
}

#[derive(Debug, Args)]
pub struct KeyArgs {
    #[arg(short, long, value_name = "key", default_value = None)]
    pub key: Option<String>,
}

#[derive(Debug, Args)]
pub struct HolidaysArgs {
    /// A temporary API key, won't be save to config, and overrides config for this request.  
    pub key: Option<String>,
    /// For countries, ISO 3166-1 alpha-2 or ISO 3166-1 alpha-3 format.
    ///
    /// For states / provinces (with our States & Provinces plan), ISO 3166-2 format.
    ///
    /// Accepts up to 10 comma separated values.
    #[arg(short, long)]
    pub country: String,
    /// year in ISO 8601 format.
    #[arg(short, long)]
    pub year: i32,

    /// 1 or 2 digit month (1-12).
    #[arg(short, long, value_name = "1-12")]
    pub month: Option<i32>,
    /// 1 or 2 digit day (1-31 depending on the month).
    #[arg(short, long, value_name = "1-31", requires = "month")]
    pub day: Option<i32>,

    /// Return only public holidays.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub public: bool,

    /// Return state / province holidays alongside countrywide holidays.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub subdivisions: bool,

    /// Search holidays by name. Minimum 5 characters.
    #[arg(long)]
    pub search: Option<String>,

    /// ISO 639-1 format (with exceptions).
    #[arg(long)]
    pub language: Option<String>,

    /// Return the first day of holidays that occur before the specific date.
    #[arg(long, requires_all = &["month", "day"], conflicts_with = "upcoming", action = clap::ArgAction::SetTrue)]
    pub previous: bool,

    /// Return the first day of holidays that occur after the specific date.
    #[arg(long, requires_all = &["month", "day"], conflicts_with = "previous", action = clap::ArgAction::SetTrue)]
    pub upcoming: bool,

    /// Response format (csv, json, php, tsv, yaml and xml). Defaults to JSON.
    #[arg(short, long, default_value_t = {"json".to_string()})]
    pub format: String,

    /// Prettifies results to be more human-readable.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}
