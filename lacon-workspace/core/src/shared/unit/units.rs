use super::definition::{PrefixGroup, UnitDef, UnitTree};
use super::dimensions::Dimension;
use super::prefixes::PREFIXES;
use super::props::{CalcMode, UnitProps};
use std::sync::LazyLock;

pub static UNITS: &[UnitDef] = units_array![
				[]
				UnitDef::new(
								"Hz",
								Dimension::Frequency,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"g",
								Dimension::Mass,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m",
								Dimension::Length,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"Å",
								Dimension::Length,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1e-10, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"m2",
								Dimension::Area,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m3",
								Dimension::Volume,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"L",
								Dimension::Volume,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 0.001, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"s",
								Dimension::Time,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"sec",
								Dimension::Time,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"mol",
								Dimension::AmountOfSubstance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"mol/m3",
								Dimension::MolarConcentration,
								Some(("mol", "m3")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"mol/L",
								Dimension::MolarConcentration,
								Some(("mol", "L")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 0.001, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"g/mol",
								Dimension::MolarConcentration,
								Some(("g", "mol")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m3/g",
								Dimension::SpecificVolume,
								Some(("m3", "g")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m3/mol",
								Dimension::MolarVolume,
								Some(("m3", "mol")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"Da", // Атомная единица массы
								Dimension::Mass,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1.66053906660e-24, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"J/mol",
								Dimension::MolarEnergy,
								Some(("J", "mol")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"J/mol⋅K",
								Dimension::MolarEntropy,
								Some(("J", "mol⋅K")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"J/mol*K",
								Dimension::MolarEntropy,
								Some(("J", "mol*K")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"g⋅m/s",
								Dimension::Momentum,
								Some(("g⋅m", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"g*m/s",
								Dimension::Momentum,
								Some(("g*m", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				//
				//
				UnitDef::new(
								"g/m2",
								Dimension::AreaDensity,
								Some(("g", "m2")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"g/m3",
								Dimension::Density,
								Some(("g", "m3")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"m/s",
								Dimension::Velocity,
								Some(("m", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/h",
								Dimension::Velocity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s2",
								Dimension::Acceleration,
								Some(("m", "s2")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s3",
								Dimension::Jerk,
								Some(("m", "s3")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s4",
								Dimension::Snap,
								Some(("m", "s4")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s5",
								Dimension::Crackle,
								Some(("m", "s5")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s6",
								Dimension::Pop,
								Some(("m", "s6")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"bit",
								Dimension::Information,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Byte",
								Dimension::Information,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 8.0, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"bit/s",
								Dimension::BitRate,
								Some(("bit", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Byte/s",
								Dimension::BitRate,
								Some(("B", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 8.0, ..UnitProps::DEFAULT }
				),
				//
				UnitDef::new(
								"Bel",
								Dimension::LogarithmicRatio,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"pH",
								Dimension::Acidity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"t",
								Dimension::Mass,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 1e6,
												..UnitProps::DEFAULT
								},
				),
				//
				UnitDef::new(
								"ft",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.3048,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"mi",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 1609.344,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"in",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.0254,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"em",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"rem",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"pt",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.000352778,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"pc",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.004233333,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"px",
								Dimension::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				//
				UnitDef::new(
								"min",
								Dimension::Time,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 60.0,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"hour",
								Dimension::Time,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 3600.0,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"day",
								Dimension::Time,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 86400.0,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"week",
								Dimension::Time,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 604800.0,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"month",
								Dimension::Time,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 2629746.0,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"year",
								Dimension::Time,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 31556952.0,
												..UnitProps::DEFAULT
								},
				),
				//
				UnitDef::new(
								"ft/s",
								Dimension::Velocity,
								Some(("m", "s")),
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps { scale: 0.3048, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"mi/h",
								Dimension::Velocity,
								Some(("m", "s")),
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps { scale: 0.44704, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"kn",
								Dimension::Velocity,
								Some(("m", "s")),
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps { scale: 0.514444, ..UnitProps::DEFAULT }
				),
				//
				UnitDef::new(
								"K",
								Dimension::Temperature,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				@multi ["deg", "\u{00B0}"] "C", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								offset: 273.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "F", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 5.0 / 9.0,
								offset: 255.3722222222222,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "De", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: -2.0 / 3.0,
								offset: 373.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Ra", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 5.0 / 9.0,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "N", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 100.0 / 33.0,
								offset: 273.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "D", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: -2.0 / 3.0,
								offset: 373.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Re", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 1.25,
								offset: 273.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Ro", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 40.0 / 21.0,
								offset: 258.864286,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "L", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								offset: 20.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "W", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 24.857191,
								offset: 542.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Da", Dimension::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 373.15,
								offset: 273.15,
								mode: CalcMode::Exponential,
								..UnitProps::DEFAULT
				},
				//
				UnitDef::new(
								"V",
								Dimension::ElectricVoltage,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"A",
								Dimension::ElectricCurrent,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W",
								Dimension::Power,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"J/s",
								Dimension::Power,
								Some(("J", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Ω",
								Dimension::ElectricResistance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"ohm",
								Dimension::ElectricResistance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"S",
								Dimension::ElectricConductance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"siemens",
								Dimension::ElectricConductance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"C",
								Dimension::ElectricCharge,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"F",
								Dimension::ElectricCapacitance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"J",
								Dimension::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W*s",
								Dimension::Energy,
								Some(("W", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W⋅s",
								Dimension::Energy,
								Some(("W", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W*s",
								Dimension::Energy,
								Some(("W", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Wh",
								Dimension::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"eV",
								Dimension::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1.602176634e-19, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"erg",
								Dimension::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1e-7, ..UnitProps::DEFAULT }
				),
				//
				UnitDef::new(
								"Pa",
								Dimension::Pressure,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"N",
								Dimension::Force,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"lm",
								Dimension::LuminousIntensity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"lx",
								Dimension::LuminousIntensity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"cd",
								Dimension::LuminousFlux,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"%",
								Dimension::Percent,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.01,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"‰",
								Dimension::Permille,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.001,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"‱",
								Dimension::PerTenThousand,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.0001,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"ppm",
								Dimension::PartPerMillion,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.000001,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"ppb",
								Dimension::PartPerBillion,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.000000001,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"ppt",
								Dimension::PartPerTrillion,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.000000000001,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"fr",
								Dimension::Fraction,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"deg",
								Dimension::Degree,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.017453292519943295,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"\u{00B0}",
								Dimension::Degree,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps {
												scale: 0.017453292519943295,
												..UnitProps::DEFAULT
								},
				),
				UnitDef::new(
								"rad",
								Dimension::Radian,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"D",
								Dimension::SpaceDimension,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
];

pub static UNITS_TREE: LazyLock<UnitTree> = LazyLock::new(|| build_unit_tree(UNITS));

pub fn build_unit_tree(units: &[UnitDef]) -> UnitTree {
	let mut tree = UnitTree::default();

	// Поддерживаемые разделители для составных единиц
	const SEPARATORS: &[&str] = &["/", "*", "⋅"];

	for unit in units {
		if unit.symbol.is_empty() {
			continue;
		}

		// 1. Всегда вставляем базовый символ (напр. "g/m2")
		tree.insert(unit.symbol);

		// 2. Если есть части (числитель/знаменатель), строим комбинации
		if let Some((n_base, d_base)) = unit.parts {
			// Собираем доступные префиксы для числителя и знаменателя
			// Включаем пустую строку "", чтобы учесть случаи без префикса
			let n_prefixes: Vec<&str> = PREFIXES.iter().filter(|(_, _, g)| *g == unit.numerator_group).map(|(s, _, _)| *s).chain(std::iter::once("")).collect();

			let d_prefixes: Vec<&str> = PREFIXES.iter().filter(|(_, _, g)| *g == unit.denominator_group).map(|(s, _, _)| *s).chain(std::iter::once("")).collect();

			for p_n in &n_prefixes {
				for p_d in &d_prefixes {
					// Пропускаем случай, когда оба префикса пустые (уже вставили unit.symbol)
					if p_n.is_empty() && p_d.is_empty() {
						continue;
					}

					// Генерируем варианты со всеми разделителями
					for separator in SEPARATORS {
						let full_unit = format!("{}{}{}{}{}", p_n, n_base, separator, p_d, d_base);
						tree.insert(&full_unit);
					}
				}
			}
		} else {
			// 3. Логика для атомарных юнитов (как была)
			if unit.numerator_group != PrefixGroup::None {
				for (p_sym, _val, p_group) in PREFIXES {
					if *p_group == unit.numerator_group {
						tree.insert(&format!("{}{}", p_sym, unit.symbol));
					}
				}
			}
		}
	}

	tree
}
