use crate::shared::candle_interval::CandleInterval;
use ahash::AHashMap;
use chrono::{DateTime, Utc};

pub fn calculate_candle_dates(
    intervals: &[CandleInterval],
    start_date: DateTime<Utc>,
    end_date: Option<DateTime<Utc>>,
) -> AHashMap<CandleInterval, DateTime<Utc>> {
    let mut dates = AHashMap::with_capacity(intervals.len());

    for interval in intervals.iter() {
        let interval_start_date = interval.get_start_date(start_date);

        if let Some(end_date) = end_date {
            while interval_start_date <= interval.get_end_date(end_date) {
                dates.insert(interval.to_owned(), interval_start_date);
            }
        } else {
            dates.insert(interval.to_owned(), interval_start_date);
        }
    }

    dates
}

#[cfg(test)]
mod tests {
    use crate::shared::candle_interval::CandleInterval;
    use crate::shared::utils::calculate_candle_dates;
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    fn calculate_candle_dates_1() {
        let intervals = [
            CandleInterval::Minute,
            CandleInterval::ThreeMinutes,
            CandleInterval::FiveMinutes,
            CandleInterval::FifteenMinutes,
            CandleInterval::ThirtyMinutes,
            CandleInterval::Hour,
            CandleInterval::TwoHours,
            CandleInterval::FourHours,
            CandleInterval::SixHours,
            CandleInterval::EightHours,
            CandleInterval::TwelveHours,
            CandleInterval::Day,
            CandleInterval::ThreeDays,
            CandleInterval::SevenDays,
            CandleInterval::Month,
        ];
        let initial_date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let dates = calculate_candle_dates(&intervals, initial_date, None);

        assert_eq!(intervals.len(), dates.len());

        for candle_type in intervals.iter() {
            let date = dates.get(candle_type);
            assert_eq!(date, Some(&candle_type.get_start_date(initial_date)))
        }
    }
}
