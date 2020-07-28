mod wthr;

use structopt::StructOpt;
use wthr::structs::TempUnit;
use wthr::{format_error, format_print, format_print_forecast};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "wthr",
    about = "A small CLI utility that retrieves and display the weather for a given city using the OpenWeather API",
    after_help = "Feel free to report any issue you find here: https://github.com/Gogopex/wthr/issues"
)]
struct Opts {
    city: String,

    #[structopt(
        short = "u",
        long = "unit",
        default_value = "metric",
        case_insensitive = true,
        possible_values = &TempUnit::variants(),
        help = "Other possible units include <imperial>, <kelvin>"
    )]
    unit: TempUnit,

    #[structopt(
        short,
        long,
        help = "Outputs a forecast of the weather for the next 5 days for a given city"
    )]
    forecast: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Opts::from_args();

    let mode = if cli.forecast { "forecast" } else { "weather" };
    let final_url = format!("https://api.openweathermap.org/data/2.5/{}?q={}&appid=cb3dcd4fe2e3b8745cb772d9ff4c34f2&units={}", mode, cli.city, cli.unit);
    let res = reqwest::blocking::get(&final_url)?.text()?;

    if mode == "forecast" {
        match wthr::process_response_forecast(&res) {
            Ok(f) => format_print_forecast(f, &cli.unit),
            Err(e) => format_error(e.to_string()),
        };
    } else {
        match wthr::process_response(&res) {
            Ok(w) => format_print(w, &cli.unit),
            Err(e) => format_error(e.to_string()),
        };
    }

    Ok(())
}
