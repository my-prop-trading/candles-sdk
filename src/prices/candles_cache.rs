use crate::prices::candle::{BidAskCandle, BidAskCandleData};
use crate::shared::candle_interval::CandleInterval;
use ahash::AHashMap;
use chrono::{DateTime, Utc};
use crate::shared::utils::calculate_candle_dates;

pub struct BidAskCandlesCache {
    candles_by_ids: AHashMap<String, BidAskCandle>,
    pub intervals: Vec<CandleInterval>,
    pub last_update_date: Option<DateTime<Utc>>,
}

impl BidAskCandlesCache {
    pub fn new(candle_intervals: Vec<CandleInterval>) -> Self {
        let mut candle_intervals = candle_intervals;
        candle_intervals.dedup();
        candle_intervals.sort();

        Self {
            candles_by_ids: AHashMap::new(),
            intervals: candle_intervals,
            last_update_date: None,
        }
    }

    pub fn get_all(&self) -> &AHashMap<String, BidAskCandle> {
        &self.candles_by_ids
    }

    pub fn len(&self) -> usize {
        self.candles_by_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles_by_ids.is_empty()
    }

    pub fn contains(&self, candle_id: &str) -> bool {
        self.candles_by_ids.contains_key(candle_id)
    }

    pub fn insert(&mut self, candle: BidAskCandle) {
        #[cfg(feature = "console-log")]
        println!(
            "insert candle {}: {} {}; {} total count",
            candle.instrument,
            candle.date.to_rfc3339(),
            candle.get_id(),
            self.candles_by_ids.len() + 1
        );

        self.candles_by_ids.insert(candle.get_id(), candle);
    }

    pub fn insert_or_update(
        &mut self,
        datetime: DateTime<Utc>,
        instrument: &str,
        bid: f64,
        ask: f64,
        bid_vol: f64,
        ask_vol: f64,
    ) {
        for interval in self.intervals.iter() {
            let candle_datetime = interval.get_start_date(datetime);
            let id = BidAskCandle::generate_id(instrument, interval, candle_datetime);
            let candle = self.candles_by_ids.get_mut(&id);

            if let Some(candle) = candle {
                candle.update(datetime, bid, ask, bid_vol, ask_vol);
            } else {
                #[cfg(feature = "console-log")]
                println!(
                    "insert candle {}: {} {}; {} total count",
                    instrument.to_owned(),
                    datetime.to_rfc3339(),
                    id,
                    self.candles_by_ids.len() + 1
                );

                self.candles_by_ids.insert(
                    id,
                    BidAskCandle {
                        ask_data: BidAskCandleData::new(datetime, ask, ask_vol),
                        bid_data: BidAskCandleData::new(datetime, bid, bid_vol),
                        index: interval.to_owned(),
                        instrument: instrument.to_string(),
                        date: candle_datetime,
                    },
                );
            }
        }

        self.last_update_date.replace(Utc::now());
    }

    /// Gets candles with date bigger or equals specified date
    pub fn get_after(&self, datetime: DateTime<Utc>) -> Option<Vec<&BidAskCandle>> {
        if self.candles_by_ids.len() == 0 {
            return None;
        }

        let candle_dates = calculate_candle_dates(&self.intervals, datetime);

        let candles = self
            .candles_by_ids
            .iter()
            .filter_map(|(_id, candle)| {
                let current_date = candle_dates
                    .get(&candle.index)
                    .expect("wrong calculate_candle_dates");

                if candle.date >= *current_date {
                    Some(candle)
                } else {
                    None
                }
            })
            .collect();

        Some(candles)
    }

    /// Removes candles with date less or equals specified date
    pub fn remove_before(
        &mut self,
        datetime: DateTime<Utc>,
        candle_type: Option<CandleInterval>,
    ) -> i32 {
        let mut removed_count = 0;

        if let Some(candle_type) = candle_type {
            self.candles_by_ids.retain(|_id, candle| {
                let current_date = candle_type.get_start_date(datetime);

                if candle.date <= current_date && candle.index == candle_type {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
        } else {
            let dates = calculate_candle_dates(&self.intervals, datetime);

            self.candles_by_ids.retain(|_id, candle| {
                let current_date = dates
                    .get(&candle.index)
                    .expect("Wrong calculate_candle_dates");

                if candle.date <= *current_date {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
        }

        removed_count
    }

    pub fn get(&self, id: &str) -> Option<&BidAskCandle> {
        self.candles_by_ids.get(id)
    }
}

