mod structs;

use chrono::prelude::*;
use std::collections::{BTreeMap, HashMap};
use structs::{Forecast, Weather};

pub fn process_response(data: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&data)?)
}

pub fn process_response_forecast(data: &str) -> Result<Forecast, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&data)?)
}

pub fn format_print(w: Weather, m: String) {
    let symb = if m == "imperial" { "°F" } else { "°C" };

    let mapping_desc = mapping_desc();
    let description = &w.weather.first().unwrap().description as &str;
    let emoji = mapping_desc[description];

    println!(
        "{0} - {1} -- Current temperature is {2}{3}. Feels like {4}{3}!",
        description,
        emoji,
        w.main.temp.round(),
        symb,
        w.main.feels_like.round()
    )
}

pub fn format_print_forecast(f: Forecast) {
    let vec_w = match f.list {
        Some(it) => it,
        _ => return,
    };

    let mut forecast_btm = BTreeMap::new();
    for w in vec_w {
        let temp = w.main.temp.round() as u32;
        let dt = Utc
            .datetime_from_str(&w.dt_txt, "%Y-%m-%d %H:%M:%S")
            .unwrap();
        forecast_btm.insert(dt, temp);
    }

    let mut tt = HashMap::new();

    for key in forecast_btm.keys() {
        let current_day = key.weekday().number_from_monday();
        match current_day {
            1..=7 => {
                let min_max = get_avg_temp(&forecast_btm, &current_day);
                tt.insert(key.weekday().to_string(), min_max);
            }
            _ => {
                println!("Error: The current day could not be found");
            }
        };
    }

    println!("{:?}  ", tt);
}

pub fn format_error(e: String) {
    println!("An error occured. {}", e)
}

pub fn mapping_desc() -> HashMap<&'static str, &'static str> {
    let mut mapping_desc = HashMap::new();
    mapping_desc.insert("clear sky", "😎");
    mapping_desc.insert("few clouds", "⛅");
    mapping_desc.insert("scattered clouds", "🌥");
    mapping_desc.insert("broken clouds", "🌥");
    mapping_desc.insert("shower rain", "🌦");
    mapping_desc.insert("rain", "🌧");
    mapping_desc.insert("thunderstorm", "⛈");
    mapping_desc.insert("snow", "🌨");
    mapping_desc.insert("mist", "🌫 ");

    mapping_desc
}

fn get_avg_temp(forecasts: &BTreeMap<DateTime<Utc>, u32>, current_day: &u32) -> (u32, u32) {
    let to_avg = &mut Vec::new();

    for (k, v) in forecasts {
        if k.weekday().number_from_monday() == *current_day {
            to_avg.push(v);
        }
    }

    let max = to_avg.iter_mut().max().unwrap().to_owned();
    let min = to_avg.iter_mut().min().unwrap().to_owned();

    (min, max)
}
