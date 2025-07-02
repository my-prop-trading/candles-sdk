# candles-sdk
Rust library for collecting and managing candlestick (OHLC) timeseries data. Designed for financial and crypto trading environments.

Supported intervals:
| Variant          | Duration    | Description                       |
| ---------------- | ----------- | --------------------------------- |
| `Minute`         | 1m          | One-minute candles                |
| `Hour`           | 1h          | One-hour candles                  |
| `Day`            | 1d          | One-day candles                   |
| `Month`          | 1mo         | One-month candles                 |
| `ThreeMinutes`   | 3m          | Three-minute candles              |
| `FiveMinutes`    | 5m          | Five-minute candles               |
| `FifteenMinutes` | 15m         | Fifteen-minute candles            |
| `ThirtyMinutes`  | 30m         | Thirty-minute candles             |
| `TwoHours`       | 2h          | Two-hour candles                  |
| `FourHours`      | 4h          | Four-hour candles                 |
| `SixHours`       | 6h          | Six-hour candles                  |
| `EightHours`     | 8h          | Eight-hour candles                |
| `TwelveHours`    | 12h         | Twelve-hour candles               |
| `ThreeDays`      | 3d          | Three-day candles                 |
| `SevenDays`      | 7d / Weekly | Weekly candles                    |
| `Endless`        | -           | No interval; continuous/aggregate |
