use rust_home_automation::rules::Rule;
use ruuviscanner::ruuvitag::subscribe_ruuvitag;
use std::env;
use std::{thread, time::Duration};
use weather_information::weather::get_current_weather;

const OPTIMAL_TEMP: f64 = 20.0;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting home automation system...");
    let mac = "CC:6F:70:EE:4C:AD";
    let rx = subscribe_ruuvitag(&mac).await?;
    println!("Subscribed to a ruuvitag.");

    let weather_api_key = env::var("WEATHER_API_KEY")?;
    let location = env::var("WEATHER_LOCATION")?;

    let ventilation_rule = Rule::new(&"Open windows?", || async {
        let current_weather = get_current_weather(&weather_api_key, &location)
            .await
            .unwrap();
        let ruuvitag = rx.recv().unwrap();
        let temp_inside = ruuvitag.temperature_in_celcius();

        let result = temp_inside >= OPTIMAL_TEMP && current_weather < temp_inside;
        Ok(result)
    });

    println!("Monitoring room temperature...");
    loop {
        let should_open_windows = ventilation_rule.poll().await?;
        if should_open_windows {
            println!("For optimal temperature, open windows.");
        }
        thread::sleep(Duration::from_millis(5000));
    }
}
