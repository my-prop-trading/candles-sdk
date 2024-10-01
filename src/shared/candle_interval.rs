use ahash::AHashSet;
use chrono::{DateTime, Datelike, Utc};
use chrono::{Duration, TimeZone};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Clone,
    IntoPrimitive,
    TryFromPrimitive,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Copy,
)]
#[repr(i32)]
pub enum CandleInterval {
    Minute = 0,
    Hour = 1,
    Day = 2,
    Month = 3,
    ThreeMinutes = 4,
    FiveMinutes = 5,
    FifteenMinutes = 6,
    ThirtyMinutes = 7,
    TwoHours = 8,
    FourHours = 9,
    SixHours = 10,
    EightHours = 11,
    TwelveHours = 12,
    ThreeDays = 13,
    SevenDays = 14,
    Infinity = 15,
}

impl CandleInterval {
    pub fn get_start_date(&self, datetime: DateTime<Utc>) -> DateTime<Utc> {
        let timestamp_sec = datetime.timestamp();

        match self {
            CandleInterval::Minute => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 60) * 1000)
                .unwrap(),
            CandleInterval::Hour => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 3600) * 1000)
                .unwrap(),
            CandleInterval::Day => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 86400) * 1000)
                .unwrap(),
            CandleInterval::Month => {
                let date = Utc.timestamp_millis_opt(timestamp_sec * 1000).unwrap();
                let start_of_month: DateTime<Utc> = Utc
                    .with_ymd_and_hms(date.year(), date.month(), 1, 0, 0, 0)
                    .unwrap();

                start_of_month
            }
            CandleInterval::ThreeMinutes => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 180) * 1000)
                .unwrap(),
            CandleInterval::FiveMinutes => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 300) * 1000)
                .unwrap(),
            CandleInterval::FifteenMinutes => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 900) * 1000)
                .unwrap(),
            CandleInterval::ThirtyMinutes => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 1800) * 1000)
                .unwrap(),
            CandleInterval::TwoHours => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 7200) * 1000)
                .unwrap(),
            CandleInterval::FourHours => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 14400) * 1000)
                .unwrap(),
            CandleInterval::SixHours => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 21600) * 1000)
                .unwrap(),
            CandleInterval::EightHours => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 28800) * 1000)
                .unwrap(),
            CandleInterval::TwelveHours => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 43200) * 1000)
                .unwrap(),
            CandleInterval::ThreeDays => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 604800) * 1000)
                .unwrap(),
            CandleInterval::SevenDays => Utc
                .timestamp_millis_opt((timestamp_sec - timestamp_sec % 1036800) * 1000)
                .unwrap(),
            CandleInterval::Infinity => Utc.timestamp_millis_opt(0).unwrap(),
        }
    }

    pub fn get_start_dates(
        &self,
        datetime_from: DateTime<Utc>,
        datetime_to: DateTime<Utc>,
    ) -> AHashSet<DateTime<Utc>> {
        let mut dates = AHashSet::new();
        let date_from = self.get_start_date(datetime_from);
        dates.insert(date_from);
        let date_to = self.get_start_date(datetime_to);

        let mut last_date = self.get_start_date(date_from);

        while last_date < date_to {
            let next_date = self.get_start_date(last_date) + self.get_duration(last_date);
            last_date = self.get_start_date(next_date);
            dates.insert(last_date);
        }

        dates
    }

    pub fn get_end_date(&self, datetime: DateTime<Utc>) -> DateTime<Utc> {
        let start = self.get_start_date(datetime);
        let duration = self.get_duration(datetime);

        start + duration
    }

    pub fn get_dates_count(&self, date_from: DateTime<Utc>, date_to: DateTime<Utc>) -> usize {
        let date_from = self.get_start_date(date_from);
        let date_to = self.get_end_date(date_to);

        match self {
            CandleInterval::Month => {
                let year_diff = date_to.year() - date_from.year();
                let month_diff = date_to.month() - date_from.month();
                let total_month_diff = year_diff * 12 + month_diff as i32;

                total_month_diff as usize
            }
            CandleInterval::Minute => {
                let duration = date_to.signed_duration_since(date_from);
                let minute_count = duration.num_minutes();

                minute_count as usize
            }
            _ => {
                let duration = self.get_duration(date_from);
                let duration_between = date_to - date_from;
                let count = duration_between.num_seconds() / duration.num_seconds();

                count as usize
            }
        }
    }

    pub fn get_duration(&self, datetime: DateTime<Utc>) -> Duration {
        let duration = match self {
            CandleInterval::Minute => Duration::seconds(60),
            CandleInterval::Hour => Duration::seconds(3600),
            CandleInterval::Day => Duration::seconds(86400),
            CandleInterval::Month => {
                let start_of_month: DateTime<Utc> = Utc
                    .with_ymd_and_hms(datetime.year(), datetime.month(), 1, 0, 0, 0)
                    .unwrap();
                let next_month = if datetime.month() == 12 {
                    1
                } else {
                    datetime.month() + 1
                };

                let next_year = if datetime.month() == 12 {
                    datetime.year() + 1
                } else {
                    datetime.year()
                };

                let end_of_month: DateTime<Utc> = Utc
                    .with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0)
                    .unwrap();

                end_of_month - start_of_month
            }
            CandleInterval::ThreeMinutes => Duration::minutes(3),
            CandleInterval::FiveMinutes => Duration::minutes(5),
            CandleInterval::FifteenMinutes => Duration::minutes(15),
            CandleInterval::ThirtyMinutes => Duration::minutes(30),
            CandleInterval::TwoHours => Duration::hours(2),
            CandleInterval::FourHours => Duration::hours(4),
            CandleInterval::SixHours => Duration::hours(6),
            CandleInterval::EightHours => Duration::hours(8),
            CandleInterval::TwelveHours => Duration::hours(12),
            CandleInterval::ThreeDays => Duration::days(3),
            CandleInterval::SevenDays => Duration::days(7),
            CandleInterval::Infinity => Duration::max_value(),
        };

        duration
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::candle_interval::CandleInterval;
    use ahash::AHashSet;
    use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

    #[test]
    fn count_minute() {
        let candle_type = CandleInterval::Minute;
        let duration = Duration::minutes(15);
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = from + duration - Duration::seconds(1);

        let count = candle_type.get_dates_count(from, to);

        assert_eq!(count, duration.num_minutes() as usize);
    }

    #[test]
    fn count_hour() {
        let candle_type = CandleInterval::Hour;
        let duration = Duration::hours(15);
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = from + duration - Duration::seconds(1);

        let count = candle_type.get_dates_count(from, to);

        assert_eq!(count, duration.num_hours() as usize);
    }

    #[test]
    fn count_day() {
        let candle_type = CandleInterval::Day;
        let duration = Duration::days(15);
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = from + duration - Duration::seconds(1);

        let count = candle_type.get_dates_count(from, to);

        assert_eq!(count, duration.num_days() as usize);
    }

    #[test]
    #[ignore]
    fn count_month() {
        let candle_type = CandleInterval::Month;
        let num_months = 12;
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = Utc.with_ymd_and_hms(2000, num_months, 1, 0, 0, 0).unwrap();

        let count = candle_type.get_dates_count(from, to);

        assert_eq!(count, num_months as usize);
    }

    #[test]
    fn get_date_for_minute() {
        let candle_type = CandleInterval::Minute;
        let src_date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 1, 1, 34).unwrap();

        let start_date = candle_type.get_start_date(src_date);

        assert_eq!(start_date.year(), src_date.year());
        assert_eq!(start_date.month(), src_date.month());
        assert_eq!(start_date.day(), src_date.day());
        assert_eq!(start_date.hour(), src_date.hour());
        assert_eq!(start_date.hour(), src_date.hour());
        assert_eq!(start_date.minute(), src_date.minute());
        assert_eq!(start_date.second(), 0);
    }

    #[test]
    fn get_date_for_hour() {
        let candle_type = CandleInterval::Hour;
        let src_date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 1, 23, 34).unwrap();

        let start_date = candle_type.get_start_date(src_date);

        assert_eq!(start_date.year(), src_date.year());
        assert_eq!(start_date.month(), src_date.month());
        assert_eq!(start_date.day(), src_date.day());
        assert_eq!(start_date.hour(), src_date.hour());
        assert_eq!(start_date.minute(), 0);
        assert_eq!(start_date.second(), 0);
    }

    #[test]
    fn get_date_for_day() {
        let candle_type = CandleInterval::Day;
        let src_date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 3, 23, 34).unwrap();

        let start_date = candle_type.get_start_date(src_date);

        assert_eq!(start_date.year(), src_date.year());
        assert_eq!(start_date.month(), src_date.month());
        assert_eq!(start_date.day(), src_date.day());
        assert_eq!(start_date.hour(), 0);
        assert_eq!(start_date.minute(), 0);
        assert_eq!(start_date.second(), 0);
    }

    #[test]
    fn get_start_date_for_month() {
        let candle_type = CandleInterval::Month;
        let src_date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 12, 3, 23, 34).unwrap();

        let start_date = candle_type.get_start_date(src_date);

        assert_eq!(start_date.year(), src_date.year());
        assert_eq!(start_date.month(), src_date.month());
        assert_eq!(start_date.day(), 1);
        assert_eq!(start_date.hour(), 0);
        assert_eq!(start_date.minute(), 0);
        assert_eq!(start_date.second(), 0);
    }

    #[test]
    fn get_end_date_for_month() {
        let candle_type = CandleInterval::Month;
        let src_date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 12, 12, 3, 23, 34).unwrap();

        let end_date = candle_type.get_end_date(src_date);

        assert_eq!(end_date.year(), src_date.year() + 1);
        assert_eq!(end_date.month(), 1);
        assert_eq!(end_date.day(), 1);
        assert_eq!(end_date.hour(), 0);
        assert_eq!(end_date.minute(), 0);
        assert_eq!(end_date.second(), 0);
    }

    #[test]
    fn get_start_dates_for_minute() {
        let duration = Duration::minutes(15);
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = from + duration;
        let candle_type = CandleInterval::Minute;

        let dates = candle_type.get_start_dates(from, to);
        let dates: AHashSet<DateTime<Utc>> = dates.into_iter().collect();

        assert!(dates.contains(&candle_type.get_start_date(from)));
        assert!(dates.contains(&candle_type.get_start_date(to)));
        assert_eq!(dates.len(), (duration.num_minutes() + 1) as usize);

        for _ in 0..duration.num_minutes() {
            let date = candle_type.get_start_date(from) + candle_type.get_duration(from);
            assert!(dates.contains(&date));
        }
    }

    #[test]
    fn get_start_dates_for_hour() {
        let duration = Duration::hours(15);
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = from + duration;
        let candle_type = CandleInterval::Hour;

        let dates = candle_type.get_start_dates(from, to);
        let dates: AHashSet<DateTime<Utc>> = dates.into_iter().collect();

        assert!(dates.contains(&candle_type.get_start_date(from)));
        assert!(dates.contains(&candle_type.get_start_date(to)));
        assert_eq!(dates.len(), (duration.num_hours() + 1) as usize);

        for _ in 0..duration.num_hours() {
            let date = candle_type.get_start_date(from) + candle_type.get_duration(from);
            assert!(dates.contains(&date));
        }
    }

    #[test]
    fn get_start_dates_for_day() {
        let duration = Duration::days(15);
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = from + duration;
        let candle_type = CandleInterval::Day;

        let dates = candle_type.get_start_dates(from, to);
        let dates: AHashSet<DateTime<Utc>> = dates.into_iter().collect();

        assert!(dates.contains(&candle_type.get_start_date(from)));
        assert!(dates.contains(&candle_type.get_start_date(to)));
        assert_eq!(dates.len(), (duration.num_days() + 1) as usize);

        for _ in 0..duration.num_days() {
            let date = candle_type.get_start_date(from) + candle_type.get_duration(from);
            assert!(dates.contains(&date));
        }
    }

    #[test]
    fn get_start_dates_for_month() {
        let num_months = 12;
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = Utc.with_ymd_and_hms(2000, num_months, 1, 0, 0, 0).unwrap();
        let candle_type = CandleInterval::Month;

        let dates = candle_type.get_start_dates(from, to);
        let dates: AHashSet<DateTime<Utc>> = dates.into_iter().collect();

        assert!(dates.contains(&candle_type.get_start_date(from)));
        assert!(dates.contains(&candle_type.get_start_date(to)));
        assert_eq!(dates.len(), num_months as usize);

        for _ in 0..num_months {
            let date = candle_type.get_start_date(from) + candle_type.get_duration(from);
            assert!(dates.contains(&date));
        }
    }
}
