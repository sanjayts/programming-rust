use serde_json::Value;

const URL: &str = "http://www.worldtimeapi.org/api/timezone/Europe/London";

fn main() {
    let result = reqwest::blocking::get(URL);
    let response = result.unwrap();
    let text = response.text().unwrap();
    let text = text.as_str();
    let json: Value = serde_json::from_str(text).unwrap();
    println!("Time in London now is {}", json["datetime"]);
}
