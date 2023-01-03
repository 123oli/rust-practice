use std::collections::HashMap;
use reqwest::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub langitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub name: String,
    pub coordinates: Coordinates,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Temperature {
    pub degrees: f64,
    pub unit: TemperatureUnit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemperatureUnit {
    Fahrenheit,
    Celsius,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weather {
    pub temperature: Temperature,
    pub conditions: Vec<Condition>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weather2 {
    weather: Vec<WeatherDetails>,
}
#[derive(Debug, Clone, PartialEq)]

struct WeatherDetails {
    description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Clear,
    Cloudy,
    Rainy,
    Snowy,
}

pub struct WeatherClient {
    pub api_key: String,
    pub locations: HashMap<String, Location>
}

impl WeatherClient {
    pub fn new(api_key: &str) -> Self {
        WeatherClient {
            api_key: api_key.to_owned(),
            locations: HashMap::new(),
        }
    }

    pub fn add_location(&mut self, location: Location) {
        self.locations.insert(location.name.clone(), location);
    }

    // pub fn get_weather(&self, location: &str) -> Option<Weather> {
    //     let location = self.locations.get(location)?;
    //     let weather = self.fetch_weather(location)?;
    //     Some(weather)
    // }

    pub fn get_weather2(&self, location: &str) -> Option<&Location> {
       let location = self.locations.get(location).unwrap();
       Some(location)
    }

    // pub fn fetch_weather(&self, location: &Location) -> Option<Weather> {
    //     // Make a request to the weather API using the location coordinates and the API key.
    //     // Parse the response and return a Weather object.
    //     // If the request fails or the response is invalid, return None.
    //     None
    // }

    pub async fn fetch_weather2(&self, city: String) -> Result<String, Error> {
        let url: String = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
            city, self.api_key,
        );
    
        let body = reqwest::get(url)
            .await?
            .text()
            .await?;
        return Ok(body);
    }

}