use crate::shared::candle_interval::CandleInterval;
use chrono::{DateTime, Utc};
use std::fmt::Display;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CandleIndex {
    pub ref_id: String,
    pub candle_interval: CandleInterval,
    pub interval_start_date: DateTime<Utc>,
}

impl CandleIndex {
    pub fn new(
        ref_id: impl Into<String>,
        candle_interval: CandleInterval,
        date: DateTime<Utc>,
    ) -> Self {
        let interval_start_date = candle_interval.get_start_date(date);

        Self {
            ref_id: ref_id.into(),
            candle_interval,
            interval_start_date,
        }
    }

    pub fn as_string(&self) -> String {
        format!(
            "{}{}{}",
            self.candle_interval as u32,
            self.ref_id,
            self.interval_start_date
                .timestamp(),
        )
    }
}

impl Display for CandleIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod test {
    use crate::shared::candle_index::CandleIndex;
    use crate::shared::candle_interval::CandleInterval;
    use chrono::{DateTime, TimeZone, Utc};

    #[test]
    pub fn as_string_1() {
        let date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let id = CandleIndex::new("123", CandleInterval::Day, date);

        assert_eq!("2123946684800".to_string(), id.to_string());
    }
}
