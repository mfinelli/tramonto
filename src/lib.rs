use chrono::prelude::*;

pub mod config;

#[derive(Debug)]
pub enum TimeOfDay {
    PreDawn,
    Daytime,
    PostDusk,
}

pub fn what_time_is_it(
    now: chrono::DateTime<Utc>,
    sunup: chrono::DateTime<Utc>,
    sundown: chrono::DateTime<Utc>,
) -> TimeOfDay {
    let now_timestamp = now.timestamp();
    let sunup_timestamp = sunup.timestamp();
    let sundown_timestamp = sundown.timestamp();

    if now_timestamp < sunup_timestamp {
        TimeOfDay::PreDawn
    } else if now_timestamp >= sunup_timestamp && now_timestamp <= sundown_timestamp {
        TimeOfDay::Daytime
    } else {
        TimeOfDay::PostDusk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predawn() {
        let now = Utc.ymd(2020, 10, 10).and_hms(4, 0, 0);
        let dawn = Utc.ymd(2020, 10, 10).and_hms(6, 0, 0);
        let dusk = Utc.ymd(2020, 10, 10).and_hms(18, 0, 0);

        assert!(matches!(
            what_time_is_it(now, dawn, dusk),
            TimeOfDay::PreDawn
        ));
    }

    #[test]
    fn daytime() {
        let now = Utc.ymd(2020, 10, 10).and_hms(12, 0, 0);
        let dawn = Utc.ymd(2020, 10, 10).and_hms(6, 0, 0);
        let dusk = Utc.ymd(2020, 10, 10).and_hms(18, 0, 0);

        assert!(matches!(
            what_time_is_it(now, dawn, dusk),
            TimeOfDay::Daytime
        ));
    }

    #[test]
    fn postdusk() {
        let now = Utc.ymd(2020, 10, 10).and_hms(20, 0, 0);
        let dawn = Utc.ymd(2020, 10, 10).and_hms(6, 0, 0);
        let dusk = Utc.ymd(2020, 10, 10).and_hms(18, 0, 0);

        assert!(matches!(
            what_time_is_it(now, dawn, dusk),
            TimeOfDay::PostDusk
        ));
    }
}
