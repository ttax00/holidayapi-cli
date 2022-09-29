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
    #[arg(short, long, value_name = "key")]
    key: Option<String>,
}

#[derive(Debug, Args)]
pub struct HolidaysArgs {
    /// year in ISO 8601 format.
    #[arg(short, long, value_name = "year")]
    year: i32,
    /// 1 or 2 digit month (1-12).
    #[arg(short, long, value_name = "1-12")]
    month: Option<i32>,
    /// 1 or 2 digit day (1-31 depending on the month).
    #[arg(short, long, value_name = "1-31", requires = "month")]
    day: Option<i32>,

    /// Return only public holidays.
    #[arg(long)]
    public: Option<bool>,

    /// Return state / province holidays alongside countrywide holidays.
    #[arg(short, long)]
    subdivisions: Option<bool>,

    /// Search holidays by name. Minimum 5 characters.
    #[arg(long)]
    search: Option<String>,

    /// ISO 639-1 format (with exceptions).
    #[arg(long)]
    language: Option<String>,

    /// Return the first day of holidays that occur before the specific date.
    #[arg(long, requires_all = &["month", "day"], conflicts_with = "upcoming")]
    previous: Option<bool>,

    /// Return the first day of holidays that occur after the specific date.
    #[arg(long, requires_all = &["month", "day"], conflicts_with = "previous")]
    upcoming: Option<bool>,

    /// Response format (csv, json, php, tsv, yaml and xml). Defaults to JSON.
    #[arg(short, long)]
    format: Option<String>,

    /// Prettifies results to be more human-readable.
    #[arg(short, long)]
    pretty: Option<bool>,
}
