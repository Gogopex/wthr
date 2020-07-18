mod structs;
use structs::*;

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
