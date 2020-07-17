use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;
use serde_json::map::Map;
use serde_json::Value;

// api_key = cb3dcd4fe2e3b8745cb772d9ff4c34f2
// https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}

#[derive(StructOpt, Debug)]
#[structopt(name = "wthr")]
struct Opts {
    city: String,
    // imperial
    #[structopt(short = "u", long = "unit", default_value = "metric")]
    unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    base: String,
    clouds: Cloud,
    cod: i32,
    coord: Coord,
    dt: i32,
    id: i32,
    main: Main,
    name: String,
    sys: Sys,
    timezone: i32,
    visibility: i32,
    // weather: Vec<String>,
    // wind: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cloud {
    all: i32

}

#[derive(Serialize, Deserialize, Debug)]
struct Coord {
    lat: f32,
    lon: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Main {
    feels_like: f32,
    humidity: i32,
    pressure: i32,
    temp: f32,
    temp_max: f32,
    temp_min: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sys {
    country: String,
    id: i32,
    sunrise: i32,
    sunset: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Opts::from_args();

    // let current_weather = get_weather(&cli.city, &cli.unit);
    // println!("{:?}", current_weather);

    // let final_url = format!("https://api.openweathermap.org/data/2.5/weather?q=bordeaux&appid=cb3dcd4fe2e3b8745cb772d9ff4c34f2&unit=metric");
    let final_url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=cb3dcd4fe2e3b8745cb772d9ff4c34f2&units={}", &cli.city, &cli.unit);
    let res = reqwest::blocking::get(&final_url)?.text()?;

    // process the string into JSON and map it to struct Weather
    let current_weather = match process_response(&res) {
        Ok(w) => println!("The weather in {} is {}°C. Feels like {}°C, nice !", w.name, w.main.temp, w.main.feels_like),
        Err(e) => println!("Boooh! You suck! Here's the error: {}", e)
    };

    println!("{:?}", current_weather);
    Ok(())
}

fn process_response(data: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    let v: Weather = serde_json::from_str(&data)?;
    Ok(v)
}
