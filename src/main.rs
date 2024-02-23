use serde::Deserialize;
use std::fs;
use reqwest::Error;
use serde_json::{json, Value};

async fn post_it(wkz: &str, city: i32) -> Result<Value, Error> {
    let url = "https://wunschkennzeichen.zulassung.de/api/check";
    let json_data = json!({
        "numberPlateText": wkz,
        "registrationOfficeServiceId": city,
        "vehicleType": "CAR",
        "licensePlateType": "REGULAR",
        "secondLineLength": null,
        "editableLength": 8,
        "startMonth": null,
        "endMonth": null
    }).to_string();

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "WKZ-rs/1.0.0")
        .header("Origin", "https://wunschkennzeichen.zulassung.de")
        .body(json_data)
        .send()
        .await?;

    //println!("Status: {}", response.status());

    let response_body = response.text().await?;
    //println!("Response body:\n{}", response_body);

    let json: Value = serde_json::from_str(&response_body).unwrap();
    //println!("{}", &json["results"]);

    Ok(json["results"].clone())
}


#[derive(Debug, Deserialize)]
struct WKZList {
    wkzs: Vec<WKZ>,
}

#[derive(Debug, Deserialize)]
struct WKZ {
    pattern: String,
    city: i32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let data = fs::read_to_string("config.cfg").expect("Unable to read file");
    let json: WKZList = serde_json::from_str(&data).expect("JSON was not well formatted");

    for wkz in json.wkzs {
        println!("Pattern: {}, {}", &wkz.pattern, post_it(&wkz.pattern, wkz.city).await?);
    }

    Ok(())
}