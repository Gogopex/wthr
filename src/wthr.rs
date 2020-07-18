use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherForecast {
    pub dt: i32,
    pub main: Main,
    pub clouds: Cloud,
    pub visibility: i32,
    pub dt_txt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub base: String,
    pub clouds: Cloud,
    pub cod: i32,
    pub coord: Coord,
    pub dt: i32,
    pub id: i32,
    pub main: Main,
    pub name: String,
    pub sys: Sys,
    pub timezone: i32,
    pub visibility: i32,
    // weather: Vec<String>,
    // wind: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    // pub city: Vec<City>,
    // pub cnt: i32,
    // pub cod: String,
    pub list: Option<Vec<WeatherForecast>>,
    // pub message: i32,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct City {
//     country: String
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Cloud {
    pub all: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub feels_like: f32,
    pub humidity: i32,
    pub pressure: i32,
    pub temp: f32,
    pub temp_max: f32,
    pub temp_min: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub country: String,
    pub id: i32,
    pub sunrise: i32,
    pub sunset: i32,
}

pub fn process_response(data: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&data)?)
}

pub fn process_response_forecast(data: &str) -> Result<Forecast, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&data)?)
}

pub fn format_print(w: Weather) {
    let mut temp_message = "";
    match w.main.feels_like {
        v if v <= 0.0 => {
            temp_message = "Freezing!";
        }
        v if v <= 9.9 => {
            temp_message = "Cover up!";
        }
        v if v <= 14.9 => {
            temp_message = "Chilly!";
        }
        v if v <= 21.9 => {
            temp_message = "Nice!";
        }
        v if v <= 31.9 => {
            temp_message = "Sunglasses out!";
        }
        _ => println!("Error: what kind of temperature is this?"),
    }
    println!(
        "The weather in {} is {}°C. Feels like {}°C! {}",
        w.name, w.main.temp, w.main.feels_like, temp_message
    )
}

pub fn format_print_forecast(f: Forecast) {
    let vec_w = match f.list {
        Some(it) => it,
        _ => return,
    };
    for w in vec_w {
        println!(
            "The weather for {} will be {}°C. Feels like {}°C!",
            w.dt_txt, w.main.temp, w.main.feels_like
        );
    }
}

pub fn format_error(e: String) {
    println!("An error occured. {}", e)
}
