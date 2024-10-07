use crate::shared::candle_interval::CandleInterval;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSecondsWithFrac};

#[serde_as]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CandleData {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub low_after_high: f64,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    pub timestamp: DateTime<Utc>,
}

impl CandleData {
    pub fn new(value: f64) -> Self {
        Self {
            open: value,
            close: value,
            high: value,
            low: value,
            low_after_high: value,
            timestamp: Utc::now(),
        }
    }

    pub fn update(&mut self, value: f64) {
        self.close = value;
        self.timestamp = Utc::now();

        if self.open == 0.0 {
            self.open = value;
        }

        if self.high < value || self.high == 0.0 {
            self.high = value;
            self.low_after_high = value;
        }

        if self.low > value || self.low == 0.0 {
            self.low = value;
        }

        if self.low_after_high > value || self.low_after_high == 0.0 {
            self.low_after_high = value
        }
    }

    pub fn get_candle_date(&self, candle_type: CandleInterval) -> DateTime<Utc> {
        candle_type.get_start_date(self.timestamp)
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::candle_data::CandleData;

    #[test]
    pub fn update_low_after_high_1() {
        let mut data = CandleData {
            open: 0.0,
            close: 0.0,
            high: 9500.0,
            low: 0.0,
            low_after_high: 9000.0,
            timestamp: Default::default(),
        };
        let value = 11000.0;

        data.update(value);

        assert_eq!(data.low_after_high, value);
    }
}
