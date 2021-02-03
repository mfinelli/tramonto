use chrono::prelude::*;
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
                suninfo = SunInfo::from_api(loc.0, loc.1).unwrap();
                println!("{:?}", suninfo);
            }

            let whatdo = match tramonto::what_time_is_it(
                now,
                suninfo.sunup().with_timezone(&Utc),
                suninfo.sundown().with_timezone(&Utc),
            ) {
                tramonto::TimeOfDay::PreDawn => (
                    config.dark(),
                    suninfo.sunup().timestamp() - now.timestamp() + 30,
                ),
                tramonto::TimeOfDay::Daytime => (
                    config.light(),
                    suninfo.sundown().timestamp() - now.timestamp() + 30,
                ),
                tramonto::TimeOfDay::PostDusk => {
                    let tomorrow = Utc.timestamp(now.timestamp() + 86_400, 0);
                    let timestamp = Utc
                        .ymd(tomorrow.year(), tomorrow.month(), tomorrow.day())
                        .and_hms(0, 0, 0)
                        .timestamp();

                    (config.dark(), timestamp - now.timestamp() + 30)
                }
            };

            match tramonto::switcher::switch_theme(&de, whatdo.0) {
                Err(_e) => return Err("unable to switch theme")?,
                _ => (),
            };

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
