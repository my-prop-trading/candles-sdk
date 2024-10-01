use crate::shared::candle_interval::CandleInterval;
use ahash::AHashMap;
use chrono::{DateTime, Utc};

pub fn calculate_candle_dates(
    intervals: &[CandleInterval],
    datetime: DateTime<Utc>,
) -> AHashMap<CandleInterval, DateTime<Utc>> {
    let mut dates = AHashMap::with_capacity(intervals.len());

    for interval in intervals.iter() {
        dates.insert(interval.to_owned(), interval.get_start_date(datetime));
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
        let dates = calculate_candle_dates(&intervals, initial_date);

        assert_eq!(intervals.len(), dates.len());

        for candle_type in intervals.iter() {
            let date = dates.get(candle_type);
            assert_eq!(date, Some(&candle_type.get_start_date(initial_date)))
        }
    }
}
