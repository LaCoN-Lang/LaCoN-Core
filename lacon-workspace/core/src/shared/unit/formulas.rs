use super::dimensions::Dimension;

pub struct DimensionFormula {
    pub num: &'static [(Dimension, i8)],
    pub den: &'static [(Dimension, i8)],
}

pub enum DimensionMode {
    Linear,      // Обычная физика: 1 + 1 = 2
    Exponential, // Инженерная физика: 1 * 2 = 2
    Logarithmic, // dB, pH: сложение требует антилогарифмировани
}

pub struct DimensionData {
    pub formula: DimensionFormula,
    pub mode: DimensionMode,
}

impl DimensionData {
    pub const DEFAULT: Self = Self {
        formula: DimensionFormula { num: &[], den: &[] },
        mode: DimensionMode::Linear,
    };
}
