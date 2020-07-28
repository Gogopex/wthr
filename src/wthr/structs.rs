use serde_derive::{Deserialize, Serialize};
use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum TempUnit {
        Metric,
        Imperial,
        Kelvin
    }
}

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
    pub weather: Vec<Description>,
    // wind: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub list: Option<Vec<WeatherForecast>>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    pub description: String,
    pub main: String,
}
