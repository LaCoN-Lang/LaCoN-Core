use super::dimensions::Dimension;

#[derive(Debug, Clone)]
pub enum CalcMode {
    Linear,      // Обычная физика: 1 + 1 = 2
    Exponential, // Инженерная физика: 1 * 2 = 2
    Logarithmic, // dB, pH: сложение требует антилогарифмировани
}

#[derive(Debug, Clone)]
pub enum Formula {
    None,              // Для безразмерных величин
    Simple(Dimension), // Одна размерность (н-р, Mass)
    Complex {
        // Составная (н-р, m/s2)
        num: &'static [Dimension],
        den: &'static [Dimension],
    },
}

#[derive(Debug, Clone)]
pub struct UnitProps {
    pub scale: f64,
    pub offset: f64,
    pub exponent: f64,
    pub mode: CalcMode,
}

impl UnitProps {
    pub const DEFAULT: Self = Self {
        scale: 1.0,
        offset: 0.0,
        exponent: 1.0,
        mode: CalcMode::Linear,
    };
}
