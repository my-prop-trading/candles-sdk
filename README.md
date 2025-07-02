# candles-sdk
Rust library for collecting and managing candlestick (OHLC) timeseries data. Designed for financial and crypto trading environments.

Supported intervals:
| Variant          | Duration    | Description                       |
| ---------------- | ----------- | --------------------------------- |
| `Minute = 0`         | 1m          | One-minute candles                |
| `Hour = 1`           | 1h          | One-hour candles                  |
| `Day = 2`            | 1d          | One-day candles                   |
| `Month = 3`          | 1mo         | One-month candles                 |
| `ThreeMinutes = 4`   | 3m          | Three-minute candles              |
| `FiveMinutes = 5`    | 5m          | Five-minute candles               |
| `FifteenMinutes = 6` | 15m         | Fifteen-minute candles            |
| `ThirtyMinutes = 7`  | 30m         | Thirty-minute candles             |
| `TwoHours = 8`       | 2h          | Two-hour candles                  |
| `FourHours = 9`      | 4h          | Four-hour candles                 |
| `SixHours = 10`       | 6h          | Six-hour candles                  |
| `EightHours = 11`     | 8h          | Eight-hour candles                |
| `TwelveHours = 12`    | 12h         | Twelve-hour candles               |
| `ThreeDays = 13`      | 3d          | Three-day candles                 |
| `SevenDays = 14`      | 7d / Weekly | Weekly candles                    |
| `Endless = 15`        | -           | No interval; continuous/aggregate |
