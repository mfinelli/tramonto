use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    light: String,
    dark: String,
}

impl Config {
    fn new(config: &str) -> Result<Config, &'static str> {
        let config: Result<Config, _> = serde_yaml::from_str(&config);

        match config {
            Ok(c) => Ok(c),
            Err(e) => Err("unable to parse config or missing values"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok() {
        let c = Config { light: "light".to_string(), dark: "dark".to_string() };
        assert_eq!(Config::new("---\nlight: light\ndark: dark\n").unwrap(), c);
    }

    #[test]
    fn missing() {
        let e = Err("unable to parse config or missing values");
        assert_eq!(Config::new("---\nlight: light\n"), e);
    }
}
