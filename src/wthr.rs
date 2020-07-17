use serde_derive::{Deserialize, Serialize};

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
    let v: Weather = serde_json::from_str(&data)?;
    Ok(v)
}
