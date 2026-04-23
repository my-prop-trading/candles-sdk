use crate::shared::candle_data::CandleData;
use crate::shared::candle_index::CandleIndex;
use crate::shared::candle_interval::CandleInterval;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeCandle {
    pub interval: CandleInterval,
    pub date: DateTime<Utc>,
    pub ref_id: String,
    pub data: CandleData,
}

impl From<&TradeCandle> for CandleIndex {
    fn from(value: &TradeCandle) -> Self {
        Self::new(value.ref_id.to_string(), value.interval, value.date)
    }
}

impl TradeCandle {
    pub fn new(index: CandleIndex, pnl: f64) -> Self {
        Self {
            interval: index.candle_interval,
            date: index.interval_start_date,
            ref_id: index.ref_id.clone(),
            data: CandleData::new(pnl),
        }
    }

    pub fn update(&mut self, pnl: f64) {
        self.data.update(pnl);
    }
}
