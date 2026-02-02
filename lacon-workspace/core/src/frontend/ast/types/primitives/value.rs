pub struct Value {}

pub struct Quantity {
    // Сделать это под-типом Value и включить Number, Unit тип данных
    pub magnitude: f64,       // Это твой Number
    pub dimension: Dimension, // Это твоя размерность
    pub multiplier: f64,      // Это твой Unit
    pub unit: UnitDef,        // Это твой Unit
}

impl Quantity {
    pub fn add(&self, other: &Self) -> Result<Self, String> {
        let info = self.dimension.info();

        // Проверяем совместимость размерностей
        if self.dimension != other.dimension {
            return Err("Incompatible dimensions".into());
        }

        match info.mode {
            DimensionMode::Linear => {
                // Обычное сложение: 10m + 5m = 15m
                Ok(Quantity::new(self.value + other.value, self.dimension))
            }
            DimensionMode::Logarithmic => {
                // Логарифмическое сложение (например, dB):
                // base_sum = 10^(val1/10) + 10^(val2/10)
                // result = 10 * log10(base_sum)
                let val1_linear = 10.0_f64.powf(self.value / 10.0);
                let val2_linear = 10.0_f64.powf(other.value / 10.0);
                let result = 10.0 * (val1_linear + val2_linear).log10();
                Ok(Quantity::new(result, self.dimension))
            }
            DimensionMode::Exponential => {
                // Если нужно специфическое поведение для Da или подобных
                Ok(Quantity::new(self.value * other.value, self.dimension))
            }
        }
    }
}
