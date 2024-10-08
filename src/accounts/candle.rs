use crate::shared::candle_data::CandleData;
use crate::shared::candle_index::CandleIndex;
use crate::shared::candle_interval::CandleInterval;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountData {
    pub equity: f64,
    pub balance: f64,
    pub pnl: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AccountCandle {
    pub interval: CandleInterval,
    pub date: DateTime<Utc>,
    pub ref_id: String,
    pub balance_data: CandleData,
    pub equity_data: CandleData,
    pub pnl_data: CandleData,
}

impl From<&AccountCandle> for CandleIndex {
    fn from(value: &AccountCandle) -> Self {
        Self::new(value.ref_id.to_string(), value.interval, value.date)
    }
}

impl AccountCandle {
    pub fn new(index: CandleIndex, data: &AccountData) -> Self {
        Self {
            interval: index.candle_interval,
            date: index.interval_start_date,
            ref_id: index.ref_id.clone(),
            balance_data: CandleData::new(data.balance),
            equity_data: CandleData::new(data.equity),
            pnl_data: CandleData::new(data.pnl),
        }
    }
    pub fn update(&mut self, data: &AccountData) {
        self.balance_data.update(data.balance);
        self.equity_data.update(data.equity);
        self.pnl_data.update(data.pnl);
    }
}
