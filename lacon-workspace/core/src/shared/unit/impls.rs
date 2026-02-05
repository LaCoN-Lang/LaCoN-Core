use super::{UnitFormula, UnitKind};

impl UnitKind {
	pub fn formula(&self) -> UnitFormula {
		use UnitKind::*;
		match self {
			// --- Механика и Кинематика ---
			Frequency => UnitFormula { num: &[(Scalar, 1)], den: &[(Time, 1)] },
			Area => UnitFormula { num: &[(Length, 2)], den: &[] },
			Volume => UnitFormula { num: &[(Length, 3)], den: &[] },

			Velocity => UnitFormula { num: &[(Length, 1)], den: &[(Time, 1)] },
			Acceleration => UnitFormula { num: &[(Length, 1)], den: &[(Time, 2)] },
			Jerk => UnitFormula { num: &[(Length, 1)], den: &[(Time, 3)] },
			Snap => UnitFormula { num: &[(Length, 1)], den: &[(Time, 4)] },
			Crackle => UnitFormula { num: &[(Length, 1)], den: &[(Time, 5)] },
			Pop => UnitFormula { num: &[(Length, 1)], den: &[(Time, 6)] },

			Momentum => UnitFormula {
				num: &[(Mass, 1), (Length, 1)],
				den: &[(Time, 1)],
			},
			Force => UnitFormula {
				num: &[(Mass, 1), (Length, 1)],
				den: &[(Time, 2)],
			},
			Pressure => UnitFormula {
				num: &[(Mass, 1)],
				den: &[(Length, 1), (Time, 2)],
			},
			Density => UnitFormula { num: &[(Mass, 1)], den: &[(Length, 3)] },
			Energy => UnitFormula {
				num: &[(Mass, 1), (Length, 2)],
				den: &[(Time, 2)],
			},

			// --- Электричество ---
			Power => UnitFormula {
				num: &[(Mass, 1), (Length, 2)],
				den: &[(Time, 3)],
			},
			ElectricCharge => UnitFormula {
				num: &[(ElectricCurrent, 1), (Time, 1)],
				den: &[],
			},
			ElectricVoltage => UnitFormula {
				num: &[(Mass, 1), (Length, 2)],
				den: &[(ElectricCurrent, 1), (Time, 3)],
			},
			ElectricResistance => UnitFormula {
				num: &[(Mass, 1), (Length, 2)],
				den: &[(ElectricCurrent, 2), (Time, 3)],
			},
			ElectricConductance => UnitFormula {
				num: &[(ElectricCurrent, 2), (Time, 3)],
				den: &[(Mass, 1), (Length, 2)],
			},
			ElectricCapacitance => UnitFormula {
				num: &[(ElectricCurrent, 2), (Time, 4)],
				den: &[(Mass, 1), (Length, 2)],
			},

			// --- Свет ---
			LuminousFlux => UnitFormula { num: &[(LuminousIntensity, 1)], den: &[] },
			Illuminance => UnitFormula {
				num: &[(LuminousIntensity, 1)],
				den: &[(Length, 2)],
			},

			BitRate => UnitFormula { num: &[(Information, 1)], den: &[(Time, 1)] },

			DataDensity => UnitFormula { num: &[(Information, 1)], den: &[(Length, 2)] },

			// Всё остальное (Scalar, Length, Time, Mass и т.д.) не имеет формулы разложения
			_ => UnitFormula { num: &[], den: &[] },
		}
	}
}
