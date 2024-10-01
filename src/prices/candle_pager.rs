use crate::prices::candle::BidAskCandle;
use crate::shared::candle_interval::CandleInterval;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug)]
pub struct CandlePager {
    instrument: String,
    candle_type: CandleInterval,
    from_date: DateTime<Utc>,
    to_date: DateTime<Utc>,
    page_id: Option<String>,
    limit: usize,
    last_item_no: usize,
}

impl CandlePager {
    pub fn new(
        instrument: String,
        candle_type: CandleInterval,
        from_date: DateTime<Utc>,
        to_date: DateTime<Utc>,
        page_id: Option<String>,
        limit: usize,
    ) -> Self {
        let from_date = candle_type.get_start_date(from_date);
        let to_date = candle_type.get_start_date(to_date);

        if from_date > to_date {
            panic!("Invalid date range: from can't be more than to")
        }

        Self {
            instrument,
            candle_type,
            from_date,
            to_date,
            page_id,
            limit,
            last_item_no: 0,
        }
    }

    pub fn get_instrument(&self) -> &str {
        &self.instrument
    }

    pub fn get_limit(&self) -> usize {
        self.limit
    }

    pub fn get_next_page_id(&self) -> Option<String> {
        let total_items_count = self
            .candle_type
            .get_dates_count(self.from_date, self.to_date);

        if self.limit > total_items_count {
            // there is only one page
            return None;
        }

        let remaining_item_count = self.limit - self.last_item_no;
        let candle_duration = self.candle_type.get_duration(self.from_date);
        let total_duration = candle_duration * remaining_item_count as i32;
        let from_date = self.from_date + total_duration + candle_duration;

        if from_date > self.to_date {
            return None;
        }

        Some(from_date.timestamp_millis().to_string())
    }

    pub fn move_page_id(&mut self) -> Option<String> {
        let next_page_id = self.get_next_page_id()?;
        let date = Utc
            .timestamp_millis_opt(next_page_id.parse().unwrap())
            .unwrap();
        self.from_date = date;

        Some(next_page_id)
    }

    pub fn move_candle_id(&mut self) -> Option<String> {
        if self.last_item_no >= self.limit {
            return None;
        }

        if self.last_item_no == 0 {
            self.from_date = self.candle_type.get_start_date(self.from_date);
            self.to_date = self.candle_type.get_end_date(self.to_date);
        }

        if let Some(page_id) = self.page_id.as_ref() {
            let page_id = page_id.parse::<i64>().expect("Failed to parse page_id");
            self.from_date = Utc.timestamp_millis_opt(page_id).unwrap()
        }

        if self.from_date >= self.to_date {
            return None;
        }

        let id = BidAskCandle::generate_id(&self.instrument, &self.candle_type, self.from_date);
        self.last_item_no += 1;
        self.from_date = self.from_date + self.candle_type.get_duration(self.from_date);

        Some(id)
    }

    pub fn get_page_candle_ids(&self) -> Vec<String> {
        if self.last_item_no >= self.limit {
            return vec![];
        }

        let mut from_date = self.candle_type.get_start_date(self.from_date);

        if let Some(page_id) = self.page_id.as_ref() {
            let page_id = page_id.parse::<i64>().expect("Failed to parse page_id");
            from_date = Utc.timestamp_millis_opt(page_id).unwrap()
        }

        let to_date = self.candle_type.get_end_date(self.to_date);

        let dates_count = self.candle_type.get_dates_count(self.from_date, to_date);

        let limit = if self.limit > dates_count {
            dates_count
        } else {
            self.limit
        };

        let mut ids = Vec::with_capacity(limit);

        for _ in 0..limit {
            if from_date >= to_date {
                return ids;
            }

            let id = BidAskCandle::generate_id(&self.instrument, &self.candle_type, from_date);
            ids.push(id);
            from_date = from_date + self.candle_type.get_duration(from_date);
        }

        ids
    }
}

#[cfg(test)]
mod tests {
    use crate::prices::candle_pager::CandlePager;
    use crate::shared::candle_interval::CandleInterval;
    use chrono::{DateTime, Duration, TimeZone, Utc};

    #[test]
    fn get_next_candle_id() {
        let mut pager = CandlePager {
            instrument: "test".to_string(),
            candle_type: CandleInterval::Minute,
            from_date: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            to_date: Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(),
            page_id: None,
            limit: 2,
            last_item_no: 0,
        };

        assert_eq!(pager.move_candle_id(), Some("0test946684800".to_string()));
        assert_eq!(1, pager.last_item_no);

        assert_eq!(pager.move_candle_id(), Some("0test946684860".to_string()));
        assert_eq!(2, pager.last_item_no);

        let id = pager.move_candle_id();
        assert_eq!(id, None);
    }

    #[test]
    fn get_next_page_id() {
        let mut pager = CandlePager {
            instrument: "test".to_string(),
            candle_type: CandleInterval::Minute,
            from_date: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            to_date: Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(),
            page_id: None,
            limit: 3,
            last_item_no: 0,
        };

        assert_eq!(pager.get_next_page_id(), Some("946685040000".to_string()));
        pager.move_candle_id();
        assert_eq!(pager.get_next_page_id(), Some("946685040000".to_string()));
        pager.move_candle_id();
        pager.move_candle_id();
        assert_eq!(pager.get_next_page_id(), Some("946685040000".to_string()));
    }

    #[test]
    fn get_page_candle_ids() {
        let pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleInterval::Minute,
            from_date: Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap(),
            to_date: Utc.with_ymd_and_hms(2000, 1, 2, 0, 0, 0).unwrap(),
            page_id: None,
            limit: 5,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();

        assert_eq!(ids.len(), pager.limit);
    }

    #[test]
    fn test_1() {
        let pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleInterval::Hour,
            from_date: Utc
                .timestamp_millis_opt("1696547451000".parse().unwrap())
                .unwrap(),
            to_date: Utc
                .timestamp_millis_opt("1697627451000".parse().unwrap())
                .unwrap(),
            page_id: None,
            limit: 1500,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();

        assert_eq!(301, ids.len());
        assert_eq!(None, pager.get_next_page_id());
    }

    #[test]
    fn test_2() {
        let pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleInterval::FourHours,
            from_date: Utc
                .timestamp_millis_opt("1701275150000".parse().unwrap())
                .unwrap(),
            to_date: Utc
                .timestamp_millis_opt("1701275486000".parse().unwrap())
                .unwrap(),
            page_id: None,
            limit: 1500,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();
        assert_eq!(1, ids.len());
    }

    #[test]
    fn test_4() {
        let from: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let to: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 12, 6, 0, 0, 0).unwrap();

        let pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleInterval::Month,
            from_date: from,
            to_date: to,
            page_id: None,
            limit: 10000,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();
        assert_eq!(ids.len(), 12);
    }

    #[test]
    fn test_5() {
        let from_date = Utc
            .timestamp_millis_opt("1697564165000".parse().unwrap())
            .unwrap();
        let to_date = Utc
            .timestamp_millis_opt("1701884165000".parse().unwrap())
            .unwrap();

        let mut pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleInterval::FourHours,
            from_date,
            to_date,
            page_id: None,
            limit: 10000,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();
        let mut count = 0;

        while let Some(_) = pager.move_candle_id() {
            count += 1;
        }

        assert_eq!(ids.len(), count);
    }

    #[test]
    fn test_6() {
        let to_date = Utc
            .timestamp_millis_opt("1702055569000".parse().unwrap())
            .unwrap();
        let from_date = to_date - Duration::minutes(5);

        let mut pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleInterval::Minute,
            from_date,
            to_date,
            page_id: None,
            limit: 10000,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();
        let mut count = 0;
        let mut last_move_date = from_date;

        while let Some(id) = pager.move_candle_id() {
            count += 1;
            last_move_date = Utc
                .timestamp_millis_opt(id.replace("0BTCUSDT", "").parse::<i64>().unwrap() * 1000)
                .unwrap();
        }

        let last_get_date = Utc
            .timestamp_millis_opt(
                ids[ids.len() - 1]
                    .replace("0BTCUSDT", "")
                    .parse::<i64>()
                    .unwrap()
                    * 1000,
            )
            .unwrap();

        assert_eq!(last_move_date, last_get_date);
        assert_eq!(ids.len(), count);
    }

    #[test]
    fn test_7() {
        let to_date = Utc
            .timestamp_millis_opt("1702055569000".parse().unwrap())
            .unwrap();
        let from_date = to_date - Duration::minutes(60);

        let mut pager = CandlePager {
            instrument: "BTCUSDT".to_string(),
            candle_type: CandleInterval::Minute,
            from_date,
            to_date,
            page_id: None,
            limit: 10000,
            last_item_no: 0,
        };

        let ids = pager.get_page_candle_ids();
        let mut count = 0;
        let mut last_move_date = from_date;

        while let Some(id) = pager.move_candle_id() {
            count += 1;
            last_move_date = Utc
                .timestamp_millis_opt(id.replace("0BTCUSDT", "").parse::<i64>().unwrap() * 1000)
                .unwrap();
        }

        let last_get_date = Utc
            .timestamp_millis_opt(
                ids[ids.len() - 1]
                    .replace("0BTCUSDT", "")
                    .parse::<i64>()
                    .unwrap()
                    * 1000,
            )
            .unwrap();

        assert_eq!(last_move_date, last_get_date);
        assert_eq!(ids.len(), count);
    }
}
