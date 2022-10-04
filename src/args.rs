use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct HolidayArgs {
    #[clap(subcommand)]
    pub command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// set api key to provided or response with current key.
    Key(KeyArgs),
    Holiday(HolidaysArgs),
    Country(CountriesArgs),
    Languages(LanguagesArgs),
    Workday(WorkdayArgs),
    Workdays(WorkdaysArgs),
}

#[derive(Debug, Args)]
pub struct KeyArgs {
    #[arg(short, long, value_name = "key", default_value = None)]
    pub key: Option<String>,
}

#[derive(Debug, Args)]
pub struct HolidaysArgs {
    /// A temporary API key, won't be save to config, and overrides config for this request.  
    #[arg(short, long)]
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
    #[arg(short = 'b', long, action = clap::ArgAction::SetTrue)]
    pub subdivisions: bool,

    /// Search holidays by name. Minimum 5 characters.
    #[arg(short, long)]
    pub search: Option<String>,

    /// ISO 639-1 format (with exceptions).
    #[arg(short, long)]
    pub language: Option<String>,

    /// Return the first day of holidays that occur before the specific date.
    #[arg(long, requires_all = &["month", "day"], conflicts_with = "upcoming", action = clap::ArgAction::SetTrue)]
    pub previous: bool,

    /// Return the first day of holidays that occur after the specific date.
    #[arg(long, requires_all = &["month", "day"], conflicts_with = "previous", action = clap::ArgAction::SetTrue)]
    pub upcoming: bool,

    /// Response format (csv, json, php, tsv, yaml and xml).
    #[arg(short, long, default_value_t = {"json".to_string()})]
    pub format: String,

    /// Prettifies results to be more human-readable.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}

#[derive(Debug, Args)]
pub struct CountriesArgs {
    /// A temporary API key, won't be save to config, and overrides config for this request.  
    #[arg(short, long)]
    pub key: Option<String>,

    /// Return only the country with the specified code.
    #[arg(short, long)]
    pub country: Option<String>,

    /// Search countries by code and name. Minimum 2 characters.
    #[arg(short, long)]
    pub search: Option<String>,

    /// Return only public holidays.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub public: bool,

    /// Response format (csv, json, php, tsv, yaml and xml).
    #[arg(short, long, default_value_t = {"json".to_string()})]
    pub format: String,

    /// Prettifies results to be more human-readable.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}

#[derive(Debug, Args)]
pub struct LanguagesArgs {
    /// A temporary API key, won't be save to config, and overrides config for this request.  
    #[arg(short, long)]
    pub key: Option<String>,

    /// Return only the language with the specified code.
    #[arg(short, long)]
    pub language: Option<String>,

    /// Search languages by code and name. Minimum 2 characters.
    #[arg(short, long)]
    pub search: Option<String>,

    /// Response format (csv, json, php, tsv, yaml and xml).
    #[arg(short, long, default_value_t = {"json".to_string()})]
    pub format: String,

    /// Prettifies results to be more human-readable.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}

#[derive(Debug, Args)]
pub struct WorkdayArgs {
    /// A temporary API key, won't be save to config, and overrides config for this request.  
    #[arg(short, long)]
    pub key: Option<String>,

    /// For countries, ISO 3166-1 alpha-2 or ISO 3166-1 alpha-3 format.
    #[arg(short, long)]
    pub country: String,

    /// The date from which to begin counting.
    #[arg(short, long, value_name = "YYYY-MM-DD")]
    pub start: String,

    /// Number of working / business days to advance (positive integer) or retrogress (negative integer) from start.
    #[arg(short, long)]
    pub days: i32,

    /// Response format (csv, json, php, tsv, yaml and xml).
    #[arg(short, long, default_value_t = {"json".to_string()})]
    pub format: String,

    /// Prettifies results to be more human-readable.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}

#[derive(Debug, Args)]
pub struct WorkdaysArgs {
    /// A temporary API key, won't be save to config, and overrides config for this request.  
    #[arg(short, long)]
    pub key: Option<String>,

    /// For countries, ISO 3166-1 alpha-2 or ISO 3166-1 alpha-3 format.
    #[arg(short, long)]
    pub country: String,

    /// The date from which to begin counting.
    #[arg(short, long, value_name = "YYYY-MM-DD")]
    pub start: String,

    /// The date to count until.
    #[arg(short, long, value_name = "YYYY-MM-DD")]
    pub end: String,

    /// Response format (csv, json, php, tsv, yaml and xml).
    #[arg(short, long, default_value_t = {"json".to_string()})]
    pub format: String,

    /// Prettifies results to be more human-readable.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub pretty: bool,
}
