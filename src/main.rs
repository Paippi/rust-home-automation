use rust_home_automation::rules::Rule;
use ruuvi_db::db::{create_connection, read_ruuvitag_data};
use weather_information::weather::get_current_weather;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let rule = Rule::new(basic_rule 
            );
    println!("{:?}", rule.poll());
    println!("{:?}", rule.poll());
    Ok(())

    let res = get_current_weather(&weather_api_key, &location)
        .await
        .ok_or("foo");
    match res {
        Ok(val) => println!("{}", val),
        _ => println!("Nope"),
    };
    Ok(())
}
async fn basic_rule() -> bool{
        let pool = create_connection().await;
        let ruuvitags = read_ruuvitag_data(&pool).await.unwrap();
        let weather_api_key = env::var("WEATHER_API_KEY").unwrap();
        let location = env::var("WEATHER_LOCATION").unwrap();
        let temperature = get_current_weather(&weather_api_key, &location).await.unwrap();
        ruuvitags[0].temperature_millicelcius as f64 / 1000_f64 > temperature
}

