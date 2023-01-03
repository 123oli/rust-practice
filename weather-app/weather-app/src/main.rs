use weather_library::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "28eaf702f309415856e42c0d453c7da3";
    let client = WeatherClient::new(api_key);

    let weather2 = client.fetch_weather2("Pokhara".to_string()).await?;
    let weather_data = json::parse(&weather2)?;

    println!("{}", weather_data["coord"]);
    println!("{}", weather_data["weather"]);
    println!("{}", weather_data["main"]["temp"]);
    println!("{}", weather_data);

    Ok(())
}
