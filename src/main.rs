use serde::Deserialize;
use std::fs;
use reqwest::Error;
use serde_json::{json, Map, Value};
use lettre::transport::smtp::authentication::Credentials; 
use lettre::{Message, SmtpTransport, Transport};
use std::thread::sleep;
use std::time::{Duration, Instant};

async fn post_it(wkz: &str, city: i32) -> Result<Response, Error> {
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

    let results: Response = serde_json::from_str(&response.text().await?).expect("JSON was not well formatted");

    Ok(results)
}


#[derive(Debug, Deserialize)]
struct WKZList {
    email: Email,
    wkzs: Vec<WKZ>,
}

#[derive(Debug, Deserialize)]
struct Email {
    sender: String,
    subject: String,
    smtp_username: String,
    smtp_password: String,
    smtp_server: String
}

#[derive(Debug, Deserialize)]
struct WKZ {
    pattern: String,
    city: i32,
    receiver: String,
    email_alert: bool,
}

#[derive(Debug, Deserialize)]
struct Response {
    success: bool,
    message: String,
    errors: Map<String, Value>,
    results: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let seconds = 3600;

    let interval = Duration::from_secs(seconds);
    let mut next_time = Instant::now() + interval;
    loop {
        execute().await?;

        println!("Sleeping for {} Seconds.", seconds);
        sleep(next_time - Instant::now());
        next_time += interval;
    }

    Ok(())
}

async fn execute() -> Result<(), Error> {
    let data = fs::read_to_string("config.cfg").expect("Unable to read file");
    let json: WKZList = serde_json::from_str(&data).expect("JSON was not well formatted");

    for wkz in json.wkzs {
        println!("Checking for {}", &wkz.pattern);
        let test = post_it(&wkz.pattern, wkz.city).await?;

        if test.success {
            println!("Found for Pattern: {}, {:?}", &wkz.pattern, test.results);

            if !wkz.email_alert {continue;}
            for kennzeichen in test.results {
                send_mail(&kennzeichen, &wkz.receiver, &json.email);
            }
        } else {
            println!("Found no matches for Pattern: {}", &wkz.pattern);
        }
    }

    Ok(())
}

fn send_mail(wkz: &str, receiver: &str, email_data: &Email) {
    let email = Message::builder() 
        .from(format!("WKZ-Checker <{}>", email_data.sender).parse().unwrap()) 
        .to(format!("Receiver <{}>", receiver).parse().unwrap()) 
        .subject(format!("{} {}", email_data.subject, wkz)) 
        .body(String::from(wkz)) 
        .unwrap(); 

    let creds = Credentials::new(email_data.smtp_username.clone(), email_data.smtp_password.clone()); 

    // Open a remote connection to gmail 
    let mailer = SmtpTransport::relay(&email_data.smtp_server) 
        .unwrap() 
        .credentials(creds) 
        .build(); 

    // Send the email 
    match mailer.send(&email) { 
    Ok(_) => println!("Email sent successfully! {}", &wkz), 
    Err(e) => panic!("Could not send email for {}: {:?}", &wkz, e), 
    }
}