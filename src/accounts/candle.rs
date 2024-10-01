use crate::shared::candle_data::CandleData;
use crate::shared::candle_index::CandleIndex;
use crate::shared::candle_interval::CandleInterval;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct AccountData {
    pub equity: f64,
    pub balance: f64,
    pub pnl: f64,
}

#[derive(Debug, Clone)]
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
    pub fn new(data: &AccountData) -> Self {
        Self {
            interval: CandleInterval::Minute,
            date: Default::default(),
            ref_id: Default::default(),
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
