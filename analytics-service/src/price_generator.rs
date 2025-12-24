use rand::Rng;

/// Generates realistic price movements using random walk with mean reversion
pub struct PriceGenerator {
    current_price: f64,
    base_price: f64,
    volatility: f64,
    mean_reversion_speed: f64,
}

impl PriceGenerator {
    pub fn new(initial_price: f64, volatility: f64) -> Self {
        Self {
            current_price: initial_price,
            base_price: initial_price,
            volatility,
            mean_reversion_speed: 0.01,
        }
    }

    /// Generate the next price and volume
    pub fn next_price(&mut self) -> (f64, f64) {
        let mut rng = rand::thread_rng();

        // Random walk component
        let random_change = rng.gen_range(-1.0..1.0) * self.volatility;

        // Mean reversion component (pulls price back toward base)
        let mean_reversion = (self.base_price - self.current_price) * self.mean_reversion_speed;

        // Occasional larger moves (simulating news events)
        let news_event = if rng.gen_ratio(1, 100) {
            rng.gen_range(-0.05..0.05) // 5% move
        } else {
            0.0
        };

        // Calculate percentage change
        let total_change = random_change + mean_reversion + news_event;
        self.current_price *= 1.0 + total_change;

        // Ensure price stays positive
        if self.current_price < 0.01 {
            self.current_price = 0.01;
        }

        // Generate volume (higher volume on larger price moves)
        let base_volume = rng.gen_range(100_000.0..1_000_000.0);
        let volume_multiplier = 1.0 + total_change.abs() * 10.0;
        let volume = base_volume * volume_multiplier;

        (self.current_price, volume)
    }

    /// Update volatility (useful for market conditions)
    pub fn set_volatility(&mut self, volatility: f64) {
        self.volatility = volatility;
    }

    /// Get current price
    pub fn current_price(&self) -> f64 {
        self.current_price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_generation() {
        let mut generator = PriceGenerator::new(100.0, 0.01);

        for _ in 0..100 {
            let (price, volume) = generator.next_price();
            assert!(price > 0.0);
            assert!(volume > 0.0);
        }
    }

    #[test]
    fn test_price_stays_positive() {
        let mut generator = PriceGenerator::new(0.1, 0.5);

        for _ in 0..1000 {
            let (price, _) = generator.next_price();
            assert!(price >= 0.01);
        }
    }
}

