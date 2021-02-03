use chrono::prelude::*;
// use std::collections::HashMap;
// // use std::env;
// use std::process::Command;
use std::thread;

use detect_desktop_environment::DesktopEnvironment;

use tramonto::config::Config;
use tramonto::sun::SunInfo;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config;

    match dirs::config_dir() {
        Some(mut cd) => {
            cd.push("tramonto.yml");
            config = Config::from_file(cd.to_str().unwrap())?;
        }
        None => {
            return Err("unable to determine config directory")?;
        }
    };

    let de = DesktopEnvironment::detect();
    if de != DesktopEnvironment::Xfce {
        return Err(format!("{:?} is not supported at this time", de))?;
    }

    let mut last_checked = Utc::now().day();
    let mut suninfo: SunInfo;

    // i don't like duplicating this code outside of the loop but with rust
    // we need to ensure that values are initialized
    {
        let loc = tramonto::ip::get_lat_lng().unwrap();
        suninfo = SunInfo::from_api(loc.0, loc.1).unwrap();
    }

    loop {
        let wait: std::time::Duration;

        {
            let now = Utc::now();

            if now.day() != last_checked {
                last_checked = now.day();
                println!("it's a new day");

                let loc = tramonto::ip::get_lat_lng().unwrap();
                println!("{:?}", loc);
                suninfo = tramonto::sun::SunInfo::from_api(loc.0, loc.1).unwrap();
                println!("{:?}", suninfo);
            }

            let whatdo = match tramonto::what_time_is_it(now, suninfo.sunup().with_timezone(&Utc), suninfo.sundown().with_timezone(&Utc)) {
                tramonto::TimeOfDay::PreDawn => (config.dark(), suninfo.sunup().timestamp() - Utc::now().timestamp()),
                tramonto::TimeOfDay::Daytime => (config.light(), suninfo.sundown().timestamp() - Utc::now().timestamp()),
                tramonto::TimeOfDay::PostDusk => {
                    let tomorrow = Utc.timestamp(Utc::now().timestamp() + 86_400, 0);
                    let timestamp = Utc.ymd(tomorrow.year(), tomorrow.month(), tomorrow.day()).and_hms(0, 0, 10).timestamp();

                    (config.dark(), timestamp - Utc::now().timestamp())
                }
            };

            match tramonto::switcher::switch_theme(&de, whatdo.0) {
                Err(_e) => return Err("unable to switch theme")?,
                _ => ()
            };

            // println!("waiting: {}", whatdo.1);
            // wait = std::time::Duration::from_secs(15);
            wait = std::time::Duration::from_secs(whatdo.1 as u64);
        }

        thread::sleep(wait);
    }

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {

//    let mut now: DateTime<Utc> = Utc::now();
//    println!("{:?}", now);

//    let nt = now.timestamp();
//    let upt = sunup.timestamp();
//    let downt = sundown.timestamp();
//    let lastchecked = now.day();

//    println!("now: {}, up: {}, down: {}", nt, upt, downt);

//    let tomorrowt = Utc.timestamp(nt + 86_400, 0);
//    // println!("{:?}", tomorrowt);
//    let tomorrow = Utc
//        .ymd(tomorrowt.year(), tomorrowt.month(), tomorrowt.day())
//        .and_hms(0, 0, 10);
//    println!("{:?}", tomorrow);

//    loop {
//        now = Utc::now();

//        if now.day() != lastchecked {
//            println!("it's a new day");
//        } else {
//            println!("it's the same day");
//        }

//        let timeofday =
//            tramonto::what_time_is_it(now, sunup.with_timezone(&Utc), sundown.with_timezone(&Utc));

//        match timeofday {
//            tramonto::TimeOfDay::PreDawn => {
//                println!("lib function predawn");
//                let output = Command::new("xfconf-query")
//                    .arg("-c")
//                    .arg("xsettings")
//                    .arg("-p")
//                    .arg("/Net/ThemeName")
//                    .arg("-s")
//                    .arg("Matcha-dark-azul")
//                    .output()
//                    .expect("failed to update theme");
//            }
//            tramonto::TimeOfDay::Daytime => {
//                println!("lib function daytime");

//                let output = Command::new("xfconf-query")
//                    .arg("-c")
//                    .arg("xsettings")
//                    .arg("-p")
//                    .arg("/Net/ThemeName")
//                    .arg("-s")
//                    .arg("Matcha-light-azul")
//                    .output()
//                    .expect("failed to update theme");
//            }
//            tramonto::TimeOfDay::PostDusk => {
//                println!("lib function postdusk");

//                let output = Command::new("xfconf-query")
//                    .arg("-c")
//                    .arg("xsettings")
//                    .arg("-p")
//                    .arg("/Net/ThemeName")
//                    .arg("-s")
//                    .arg("Matcha-dark-azul")
//                    .output()
//                    .expect("failed to update theme");
//            }
//        };

//        if upt > nt {
//            println!("it's before dawn");
//            println!("apply dark mode");
//            println!("sleep for: {}", upt - nt);
//        } else if upt <= nt || nt <= downt {
//            println!("it's between dawn and dusk");
//            println!("apply light mode");
//            println!("sleep for: {}", downt - nt);
//        } else {
//            println!("it's after dusk");
//            println!("apply dark mode");
//            println!("sleep for: {}", tomorrow.timestamp() - nt);
//        }

//        let one_minute = std::time::Duration::from_secs(60);
//        thread::sleep(one_minute);
//    }
//    // println!("{}", nt - upt);
//    // println!("{}", nt - downt);

//    Ok(())
//}
