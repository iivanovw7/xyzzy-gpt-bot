use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CoinGeckoHistory {
    pub prices: Vec<Vec<f64>>,
}

pub struct MarketHistory {
    prices: Vec<f64>,
}

impl MarketHistory {
    pub fn new(initial_prices: Vec<f64>) -> Self {
        MarketHistory {
            prices: initial_prices,
        }
    }

    pub fn update_price(&mut self, new_price: f64) {
        self.prices.push(new_price);

        if self.prices.len() > 50 {
            self.prices.remove(0);
        }
    }

    pub fn calculate_rsi(&self) -> f64 {
        let period = 14;
        if self.prices.len() < period + 1 {
            return 50.0;
        }

        let mut gain_sum = 0.0;
        let mut loss_sum = 0.0;

        for i in 1..=period {
            let change = self.prices[self.prices.len() - period + i - 1]
                - self.prices[self.prices.len() - period + i - 2];
            if change >= 0.0 {
                gain_sum += change;
            } else {
                loss_sum += change.abs();
            }
        }

        let mut avg_gain = gain_sum / period as f64;
        let mut avg_loss = loss_sum / period as f64;

        for i in (period + 1)..self.prices.len() {
            let change = self.prices[i] - self.prices[i - 1];
            let current_gain = if change >= 0.0 { change } else { 0.0 };
            let current_loss = if change < 0.0 { change.abs() } else { 0.0 };

            avg_gain = (avg_gain * (period as f64 - 1.0) + current_gain) / period as f64;
            avg_loss = (avg_loss * (period as f64 - 1.0) + current_loss) / period as f64;
        }

        if avg_loss == 0.0 {
            return 100.0;
        }

        let rs = avg_gain / avg_loss;

        100.0 - (100.0 / (1.0 + rs))
    }

    pub fn calculate_bollinger_bands(&self) -> (f64, f64, f64, String) {
        let period = 20;
        let std_dev_multiplier = 2.0;

        if self.prices.len() < period {
            return (0.0, 0.0, 0.0, "mid".to_string());
        }

        let slice = &self.prices[self.prices.len() - period..];
        let mean: f64 = slice.iter().sum::<f64>() / period as f64;

        let variance: f64 = slice.iter().map(|&p| (p - mean).powi(2)).sum::<f64>() / period as f64;

        let std_dev = variance.sqrt();
        let lower_band = mean - (std_dev_multiplier * std_dev);
        let upper_band = mean + (std_dev_multiplier * std_dev);
        let middle_band = mean;

        let current_price = *self.prices.last().unwrap_or(&0.0);
        let status = if current_price <= lower_band {
            "lower"
        } else if current_price >= upper_band {
            "upper"
        } else {
            "mid"
        };

        (lower_band, middle_band, upper_band, status.to_string())
    }

    fn calculate_ema(&self, period: usize) -> Vec<f64> {
        if self.prices.len() < period {
            return Vec::new();
        }

        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut emas = Vec::with_capacity(self.prices.len() - period + 1);

        let first_sma: f64 = self.prices[0..period].iter().sum::<f64>() / period as f64;

        emas.push(first_sma);

        for i in period..self.prices.len() {
            let ema = (self.prices[i] - emas.last().unwrap()) * multiplier + emas.last().unwrap();
            emas.push(ema);
        }
        emas
    }

    pub fn calculate_macd(&self) -> (f64, f64, f64) {
        let fast_period = 12;
        let slow_period = 26;
        let signal_period = 9;

        if self.prices.len() < slow_period + signal_period {
            return (0.0, 0.0, 0.0);
        }

        let emas_fast = self.calculate_ema(fast_period);
        let emas_slow = self.calculate_ema(slow_period);

        let macd_line: Vec<f64> = emas_fast
            .iter()
            .skip(slow_period - fast_period)
            .zip(emas_slow.iter())
            .map(|(fast, slow)| fast - slow)
            .collect();

        if macd_line.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let macd_history = MarketHistory::new(macd_line.clone());
        let signal_line = macd_history.calculate_ema(signal_period);

        if signal_line.is_empty() {
            return (0.0, 0.0, 0.0);
        }

        let current_macd = *macd_line.last().unwrap_or(&0.0);
        let current_signal = *signal_line.last().unwrap_or(&0.0);
        let histogram = current_macd - current_signal;

        (current_macd, current_signal, histogram)
    }

    pub fn calculate_stochastic_oscillator(&self) -> (f64, f64) {
        let k_period = 14;
        let d_period = 3;

        if self.prices.len() < k_period {
            return (0.0, 0.0);
        }

        let mut percent_k_values = Vec::new();

        for i in k_period - 1..self.prices.len() {
            let slice_start = i - (k_period - 1);
            let slice = &self.prices[slice_start..=i];

            let high = slice.iter().fold(f64::MIN, |acc, &x| acc.max(x));
            let low = slice.iter().fold(f64::MAX, |acc, &x| acc.min(x));
            let current_close = *slice.last().unwrap();

            if (high - low).abs() < f64::EPSILON {
                percent_k_values.push(50.0);
            } else {
                let k = ((current_close - low) / (high - low)) * 100.0;
                percent_k_values.push(k);
            }
        }

        if percent_k_values.len() < d_period {
            return (
                *percent_k_values.last().unwrap_or(&0.0),
                *percent_k_values.last().unwrap_or(&0.0),
            );
        }

        let k_history = MarketHistory::new(percent_k_values.clone());
        let percent_d_values = k_history.calculate_sma_for_stochastic(d_period);

        let current_k = *percent_k_values.last().unwrap_or(&0.0);
        let current_d = *percent_d_values.last().unwrap_or(&0.0);

        (current_k, current_d)
    }

    fn calculate_sma_for_stochastic(&self, period: usize) -> Vec<f64> {
        if self.prices.len() < period {
            return Vec::new();
        }

        let mut smas = Vec::new();
        for i in period - 1..self.prices.len() {
            let sum: f64 = self.prices[i - (period - 1)..=i].iter().sum();
            smas.push(sum / period as f64);
        }
        smas
    }
}
