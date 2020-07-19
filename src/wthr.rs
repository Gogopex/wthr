mod structs;
use structs::*;
use chrono::prelude::*;
use std::collections::{HashMap, BTreeMap};

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
    // let now = Utc::now();
    
    let vec_w = match f.list {
        Some(it) => it,
        _ => return,
    };

    let mut mapping_wd = HashMap::new();
    mapping_wd.insert(1, "Mon");
    mapping_wd.insert(2, "Tue");
    mapping_wd.insert(3, "Wed");
    mapping_wd.insert(4, "Thur");
    mapping_wd.insert(5, "Fri");
    mapping_wd.insert(6, "Sat");
    mapping_wd.insert(7, "Sun");

    let mut forecast_hm = BTreeMap::new();

    // for (count, w) in vec_w.into_iter().enumerate() {
    for w in vec_w {
        let dt = Utc.datetime_from_str(&w.dt_txt, "%Y-%m-%d %H:%M:%S").unwrap();
        forecast_hm.insert(dt.weekday().number_from_monday(), w.main.temp);
        
        // let current_day = dt.day();
        // t.insert([count as u32, dt.day() as u32], w.main.temp);



        // forecast_hm.insert(dt.day(), w.main.temp);
        // dbg!("{}", dt.day());
        // if dt.day() == now.day() {
        //     println!("{}, {}", dt.day(), now.day());
        // }
    }

    for (k, v) in &forecast_hm {
        // let t = mapping_wd.get_key_value(dt.weekday().number_from_monday());
        println!("{}, {}", mapping_wd[&k], forecast_hm[&k]);
    }
    // let mut d = 0 as u32;
    // for v in forecast_hm {
    //     if d == v.0.day() {
    //         dbg!("{:?}", v);
    //     }

    //     d = v.0.day();
    // }
    // for v in forecast_hm {
    //     for d in &mapping_wd {
    //         println!("{}, {}", d.1, v.1);
    //     }
    }

pub fn format_error(e: String) {
    println!("An error occured. {}", e)
}
