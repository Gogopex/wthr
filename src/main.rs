mod wthr;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "wthr")]
struct Opts {
    city: String,
    // @TODO: add help message for imperial
    #[structopt(short = "u", long = "unit", default_value = "metric")]
    unit: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Opts::from_args();

    let final_url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=cb3dcd4fe2e3b8745cb772d9ff4c34f2&units={}", &cli.city, &cli.unit);
    let res = reqwest::blocking::get(&final_url)?.text()?;

    // process the string into JSON and map it to struct Weather
    let _current_weather = match wthr::process_response(&res) {
        Ok(w) => wthr::format_print(w),
        Err(e) => wthr::format_error(e.to_string()),
    };

    Ok(())
}
