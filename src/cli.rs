// Salati - CLI
//
//! CLI to compute prayer times
//!
//! ## Example
//!
//! salati -c "51.5072,0.1276" --method karachi
//!
use clap::Parser;

use salati::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[clap(short, long)]
    coordinates: String,
    #[clap(long, arg_enum)]
    method: Method,
    #[clap(long, arg_enum, default_value_t=Madhab::default())]
    madhab: Madhab,
    #[clap(long, arg_enum, default_value_t=Twilight::default())]
    twilight: Twilight,
    #[clap(long, arg_enum)]
    /// If not provided, we will default to the recommended configuration based on coordinates
    high_latitude_rule: Option<HighLatitudeRule>,
    #[clap(long, arg_enum, default_value_t=PolarCircleResolution::default())]
    polar_circle_resolution: PolarCircleResolution,
}

pub fn main() {
    let args = Cli::parse();

    println!(
        "Using coordinates: {}, method: {:?}\n",
        args.coordinates, args.method
    );

    let parts: Vec<&str> = args.coordinates.split(',').collect();
    let long: f64 = parts[0].parse().unwrap();
    let lat: f64 = parts[1].parse().unwrap();
    let coordinates = Coordinates::new(long, lat);

    let date = Utc::today();
    let mut params = ParametersBuilder::with(Method::MuslimWorldLeague, Madhab::Shafi);
    params.twilight = args.twilight;
    params.polar_circle_resolution = args.polar_circle_resolution;
    match args.high_latitude_rule {
        Some(rule) => params.high_latitude_rule = rule,
        None => params.high_latitude_rule = HighLatitudeRule::recommended(coordinates),
    }

    let prayers = PrayerTimes::new(date, coordinates, params);

    let format_dt: fn(PrayerTime) -> String =
        |pt| -> String { pt.datetime.unwrap().format("%H:%M %p").to_string() };
    println!("Fajr     : {}", format_dt(prayers.fajr));
    println!("Sunrise  : {}", format_dt(prayers.sunrise));
    println!("Dhuhr    : {}", format_dt(prayers.dhuhr));
    println!("Asr      : {}", format_dt(prayers.asr));
    println!("Maghrib  : {}", format_dt(prayers.maghrib));
    println!("Isha     : {}", format_dt(prayers.isha));
    println!("Midnight : {}", format_dt(prayers.middle_of_the_night));
    println!("Qiyam    : {}", format_dt(prayers.qiyam));
}
