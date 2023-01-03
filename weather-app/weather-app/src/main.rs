use weather_library::*;


fn main() {
let mut client = WeatherClient::new("your-api-key-here");

let location = Location {
    name: "San Francisco".to_owned(),
    coordinates: Coordinates { 
        latitude: 37.7749, 
        langitude: 122.4194 
        },
    };

    client.add_location(location);

    let weather = client.get_weather2("San Francisco");
    
    println!("{:?}", weather);
}
