mod structs;
use chrono::prelude::*;
use std::collections::{BTreeMap, HashMap};
use structs::*;

pub fn process_response(data: &str) -> Result<Weather, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&data)?)
}

pub fn process_response_forecast(data: &str) -> Result<Forecast, Box<dyn std::error::Error>> {
    Ok(serde_json::from_str(&data)?)
}

pub fn format_print(w: Weather, m: String) {
    let symb = if m == "imperial" { "°F" } else { "°C" };

    let mapping_desc = mapping_desc();
    let description = &w.weather.last().unwrap().description as &str;
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

    let mapping_wd = mapping_wd();
    let mut forecast_hm = BTreeMap::new();
    for w in vec_w {
        let dt = Utc
            .datetime_from_str(&w.dt_txt, "%Y-%m-%d %H:%M:%S")
            .unwrap();
        forecast_hm.insert(dt.weekday().number_from_monday(), w.main.temp.round());
    }

    for k in forecast_hm.keys() {
        let forecast_string = format!("{}: {}°C", mapping_wd[k], forecast_hm[k].to_string());
        println!("{}", forecast_string);
    }
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

pub fn mapping_wd() -> HashMap<u32, &'static str> {
    let mut mapping_wd = HashMap::new();
    mapping_wd.insert(1, "Mon");
    mapping_wd.insert(2, "Tue");
    mapping_wd.insert(3, "Wed");
    mapping_wd.insert(4, "Thur");
    mapping_wd.insert(5, "Fri");
    mapping_wd.insert(6, "Sat");
    mapping_wd.insert(7, "Sun");

    mapping_wd
}
