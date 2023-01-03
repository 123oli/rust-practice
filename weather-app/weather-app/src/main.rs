use weather_library::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "28eaf702f309415856e42c0d453c7da3";
    let client = WeatherClient::new(api_key);

    // let location = Location {
    //     name: "San Francisco".to_owned(),
    //     coordinates: Coordinates { 
    //         latitude: 37.7749, 
    //         langitude: 122.4194 
    //         },
    //     };

    // client.add_location(location);

    // let weather = client.get_weather2("San Francisco");

    let weather2 = client.fetch_weather2("Kathmandu".to_string()).await?;
    let weather_data = json::parse(&weather2)?;
    // let status = weather_data["status"].clone();
    // if status  == "1" {
    // }
    let live_val = weather_data["lives"][0].clone();
    let weather = live_val["weather"].clone();
    let temperature = live_val["temprature"].clone();
    println!("weather: {}", weather);
    println!("temprature: {}", temperature);
    
    // println!("{:?}", weather);

    Ok(())
}
