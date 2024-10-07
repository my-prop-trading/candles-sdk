use crate::accounts::candle::{AccountCandle, AccountData};
use crate::shared::candle_index::CandleIndex;
use crate::shared::candle_interval::CandleInterval;
use crate::shared::utils::calculate_candle_dates;
use ahash::AHashMap;
use chrono::{DateTime, Utc};

pub struct AccountCandlesCache {
    candles_by_indexes: AHashMap<CandleIndex, AccountCandle>,
    pub intervals: Vec<CandleInterval>,
    pub last_update_date: Option<DateTime<Utc>>,
}

impl AccountCandlesCache {
    pub fn new(candle_intervals: Vec<CandleInterval>) -> Self {
        let mut candle_intervals = candle_intervals;
        candle_intervals.dedup();
        candle_intervals.sort();

        Self {
            candles_by_indexes: AHashMap::new(),
            intervals: candle_intervals,
            last_update_date: None,
        }
    }

    pub fn get_all(&self) -> &AHashMap<CandleIndex, AccountCandle> {
        &self.candles_by_indexes
    }

    pub fn len(&self) -> usize {
        self.candles_by_indexes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.candles_by_indexes.is_empty()
    }

    pub fn contains(&self, index: &CandleIndex) -> bool {
        self.candles_by_indexes.contains_key(index)
    }

    pub fn insert_or_replace(&mut self, candle: AccountCandle) -> Option<AccountCandle> {
        self.candles_by_indexes.insert((&candle).into(), candle)
    }

    pub fn get_mut(&mut self, index: &CandleIndex) -> Option<&mut AccountCandle> {
        self.candles_by_indexes.get_mut(index)
    }

    pub fn update_or_create(&mut self, date: DateTime<Utc>, ref_id: &str, data: AccountData) {
        for interval in self.intervals.iter() {
            let index = CandleIndex::new(ref_id, interval.to_owned(), date);
            let candle = self.candles_by_indexes.get_mut(&index);

            if let Some(candle) = candle {
                candle.update(&data);
            } else {
                self.candles_by_indexes
                    .insert(index.clone(), AccountCandle::new(index, &data));
            }
        }

        self.last_update_date.replace(Utc::now());
    }

    /// Gets candles with date bigger or equals specified date
    pub fn get_after(&self, date: DateTime<Utc>) -> Option<Vec<&AccountCandle>> {
        if self.candles_by_indexes.len() == 0 {
            return None;
        }

        let candle_dates = calculate_candle_dates(&self.intervals, date);

        let candles = self
            .candles_by_indexes
            .iter()
            .filter_map(|(_id, candle)| {
                let current_date = candle_dates
                    .get(&candle.interval)
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
    pub fn remove_before(&mut self, date: DateTime<Utc>, interval: Option<CandleInterval>) -> i32 {
        let mut removed_count = 0;

        if let Some(interval) = interval {
            self.candles_by_indexes.retain(|_id, candle| {
                let current_date = interval.get_start_date(date);

                if candle.date <= current_date && candle.interval == interval {
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
        } else {
            let dates = calculate_candle_dates(&self.intervals, date);

            self.candles_by_indexes.retain(|_id, candle| {
                let current_date = dates
                    .get(&candle.interval)
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

    pub fn get(&self, index: &CandleIndex) -> Option<&AccountCandle> {
        self.candles_by_indexes.get(index)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;
    #[test]
    pub fn insert_or_replace_1() {
        let date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 12, 12, 3, 23, 34).unwrap();
        let intervals = vec![CandleInterval::Minute];
        let data = AccountData {
            equity: 1000.0,
            balance: 1000.0,
            pnl: 0.0,
        };
        let id = "1";
        let index = CandleIndex::new(id, intervals[0], date);
        let candle = AccountCandle::new(index.clone(), &data);
        let mut cache = AccountCandlesCache::new(intervals);

        cache.insert_or_replace(candle);

        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
    }

    #[test]
    pub fn update_or_create_1() {
        let date: DateTime<Utc> = Utc.with_ymd_and_hms(2000, 12, 12, 3, 23, 34).unwrap();
        let intervals = vec![CandleInterval::Minute];
        let data_1 = AccountData {
            equity: 1000.0,
            balance: 1000.0,
            pnl: 0.0,
        };
        let data_2 = AccountData {
            equity: 1010.0,
            balance: 1000.0,
            pnl: 10.0,
        };
        let id = "1";
        let index = CandleIndex::new(id, intervals[0], date);
        let candle = AccountCandle::new(index.clone(), &data_1);
        let mut cache = AccountCandlesCache::new(intervals);

        cache.insert_or_replace(candle);
        cache.update_or_create(date, id, data_2.clone());
        let cache_candle = cache.get(&index).unwrap();

        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());
        assert_eq!(cache_candle.equity_data.close, data_2.equity);
    }
}
