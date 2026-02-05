use super::CalcMode;
use super::UnitKind;

pub struct UnitFormula {
	pub num: &'static [(UnitKind, i8)],
	pub den: &'static [(UnitKind, i8)],
}

pub struct UnitData {
	pub formula: UnitFormula,
	pub mode: CalcMode,
}

impl UnitData {
	pub const DEFAULT: Self = Self {
		formula: UnitFormula { num: &[], den: &[] },
		mode: CalcMode::Linear,
	};
}
