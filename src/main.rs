use std::collections::HashMap;
use chrono::prelude::*;
// use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args: Vec<String> = env::args().collect();
    //
    let response = reqwest::get("https://ipinfo.io/json")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    // println!("{:?}", response);
    // println!("loc={}", response.get("loc").unwrap());
    //
    let loc: Vec<&str> = response.get("loc").unwrap().split(',').collect();
    // println!("{:?}", loc);
    //
    if loc.len() != 2 {
        panic!("didn't get two for lat/lng");
    }

    let url = format!("https://api.sunrise-sunset.org/json?lat={}&lng={}&formatted=0", loc[0], loc[1]);
    // println!("{}", url);

    let ss_response = reqwest::get(&url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    // println!("{:?}", ss_response);
    println!("sunup: {}, sundown: {}", ss_response["results"]["sunrise"], ss_response["results"]["sunset"]);
    //

    let sunup = DateTime::parse_from_rfc3339(ss_response["results"]["sunrise"].as_str().unwrap()).unwrap();
    let sundown = DateTime::parse_from_rfc3339(ss_response["results"]["sunset"].as_str().unwrap()).unwrap();
    println!("{:?}", sunup);
    println!("{:?}", sundown);

    let now: DateTime<Utc> = Utc::now();
    println!("{:?}", now);

    let nt = now.timestamp();
    let upt = sunup.timestamp();
    let downt = sundown.timestamp();

    println!("now: {}, up: {}, down: {}", nt, upt, downt);

    let tomorrowt = Utc.timestamp(nt + 86_400, 0);
    // println!("{:?}", tomorrowt);
    let tomorrow = Utc.ymd(tomorrowt.year(), tomorrowt.month(), tomorrowt.day()).and_hms(0, 0, 10);
    println!("{:?}", tomorrow);

    if upt > nt {
        println!("it's before dawn");
        println!("apply dark mode");
        println!("sleep for: {}", upt - nt);
    } else if upt <= nt || nt <= downt {
        println!("it's between dawn and dusk");
        println!("apply light mode");
        println!("sleep for: {}", downt - nt);
    } else {
        println!("it's after dusk");
        println!("apply dark mode");
        println!("sleep for: {}", tomorrow.timestamp() - nt);
    }
    // println!("{}", nt - upt);
    // println!("{}", nt - downt);

    Ok(())
}
