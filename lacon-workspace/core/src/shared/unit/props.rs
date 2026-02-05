use super::CalcMode;
use super::UnitKind;

#[derive(Debug, Clone)]
pub enum Formula {
	None,             // Для безразмерных величин
	Simple(UnitKind), // Одна размерность (н-р, Mass)
	Complex {
		// Составная (н-р, m/s2)
		num: &'static [UnitKind],
		den: &'static [UnitKind],
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
