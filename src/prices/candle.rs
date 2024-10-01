use crate::shared::candle_interval::CandleInterval;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampSecondsWithFrac};

#[derive(Clone)]
pub struct BidAskCandle {
    pub index: CandleInterval,
    pub date: DateTime<Utc>,
    pub instrument: String,
    // todo: use shared::CandleData
    pub bid_data: BidAskCandleData,
    pub ask_data: BidAskCandleData,
}

impl BidAskCandle {
    pub fn update(
        &mut self,
        datetime: DateTime<Utc>,
        bid: f64,
        ask: f64,
        bid_vol: f64,
        ask_vol: f64,
    ) {
        self.bid_data.update(datetime, bid, bid_vol);
        self.ask_data.update(datetime, ask, ask_vol);
    }

    pub fn generate_id(
        instrument: &str,
        candle_type: &CandleInterval,
        datetime: DateTime<Utc>,
    ) -> String {
        format!(
            "{}{}{}",
            candle_type.to_owned() as u8,
            instrument.to_string(),
            candle_type.get_start_date(datetime).timestamp(),
        )
    }

    pub fn get_id(&self) -> String {
        BidAskCandle::generate_id(&self.instrument, &self.index, self.date)
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidAskCandleData {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    #[serde_as(as = "TimestampSecondsWithFrac<f64>")]
    pub datetime: DateTime<Utc>,
    pub volume: f64,
}

impl BidAskCandleData {
    pub fn new(datetime: DateTime<Utc>, price: f64, volume: f64) -> Self {
        Self {
            open: price,
            close: price,
            high: price,
            low: price,
            datetime,
            volume,
        }
    }

    pub fn update(&mut self, datetime: DateTime<Utc>, price: f64, volume: f64) {
        self.close = price;
        self.volume += volume;
        self.datetime = datetime;

        if self.open == 0.0 {
            self.open = price;
        }

        if self.high < price || self.high == 0.0 {
            self.high = price;
        }

        if self.low > price || self.low == 0.0 {
            self.low = price;
        }
    }

    pub fn get_candle_date(&self, candle_type: CandleInterval) -> DateTime<Utc> {
        candle_type.get_start_date(self.datetime)
    }
}
