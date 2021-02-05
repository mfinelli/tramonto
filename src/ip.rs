use std::collections::HashMap;

pub fn get_lat_lng() -> Result<(String, String), Box<dyn std::error::Error>> {
    let response =
        reqwest::blocking::get("https://ipinfo.io/json")?.json::<HashMap<String, String>>()?;

    match response.get("loc") {
        Some(l) => Ok(split_loc(l)?),
        None => return Err("location missing from response")?,
    }
}

fn split_loc(loc: &str) -> Result<(String, String), &'static str> {
    let ll: Vec<&str> = loc.split(',').collect();

    if ll.len() != 2 {
        return Err("lat/lng split didn't return 2")?;
    }

    Ok((ll[0].to_string(), ll[1].to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_many() {
        let e = Err("lat/lng split didn't return 2");
        assert_eq!(split_loc("0,0,0"), e);
    }

    #[test]
    fn not_enough() {
        let e = Err("lat/lng split didn't return 2");
        assert_eq!(split_loc("0"), e);
    }

    #[test]
    fn just_right() {
        assert_eq!(
            split_loc("0,0").unwrap(),
            ("0".to_string(), "0".to_string())
        );
    }
}
