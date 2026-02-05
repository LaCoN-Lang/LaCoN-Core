#[derive(Debug, Clone)]
pub enum CalcMode {
	Linear,      // Обычная физика: 1 + 1 = 2
	Exponential, // Инженерная физика: 1 * 2 = 2
	Logarithmic, // dB, pH: сложение требует антилогарифмировани
}
