use std::time::Duration;

struct HttpClientBuilder {
    base_url: String,
    timeout: Option<Duration>,
    headers: Vec<(String, String)>
}

impl HttpClientBuilder {
    fn new(base_url: &str) -> Self {
        HttpClientBuilder {
            base_url: base_url.to_owned(),
            timeout: None,
            headers: Vec::new(),
        }
    }

    fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_owned(), value.to_owned()));
        self
    }

    fn build(self) -> HttpClient {
        HttpClient {
            base_url: self.base_url,
            timeout: self.timeout,
            headers: self.headers,
        }
    }
}

struct HttpClient {
    base_url: String,
    timeout: Option<Duration>,
    headers: Vec<(String, String)>
}

fn main() {
    let client = HttpClientBuilder::new("https://www.example.com")
        .timeout(Duration::from_secs(10))
        .header("User-Agent", "My App/1.0")
        .build();
}