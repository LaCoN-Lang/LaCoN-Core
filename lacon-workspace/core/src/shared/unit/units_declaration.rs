use super::UnitKind;
use super::definition::{PrefixGroup, UnitDef, UnitTree};
use super::prefixes::PREFIXES;
use super::{CalcMode, UnitProps};

use std::collections::BTreeMap;
use std::sync::LazyLock;

pub static UNITS: &[UnitDef] = units_array![
				[]
				UnitDef::new(
								"Hz",
								UnitKind::Frequency,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"g",
								UnitKind::Mass,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m",
								UnitKind::Length,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"Å",
								UnitKind::Length,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1e-10, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"m2",
								UnitKind::Area,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m3",
								UnitKind::Volume,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"L",
								UnitKind::Volume,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 0.001, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"s",
								UnitKind::Time,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"sec",
								UnitKind::Time,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"mol",
								UnitKind::AmountOfSubstance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"mol/m3",
								UnitKind::MolarConcentration,
								Some(("mol", "m3")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"mol/L",
								UnitKind::MolarConcentration,
								Some(("mol", "L")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 0.001, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"g/mol",
								UnitKind::MolarConcentration,
								Some(("g", "mol")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m3/g",
								UnitKind::SpecificVolume,
								Some(("m3", "g")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"m3/mol",
								UnitKind::MolarVolume,
								Some(("m3", "mol")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"Da", // Атомная единица массы
								UnitKind::Mass,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1.66053906660e-24, ..UnitProps::DEFAULT },
				),
				UnitDef::new(
								"J/mol",
								UnitKind::MolarEnergy,
								Some(("J", "mol")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"J/mol⋅K",
								UnitKind::MolarEntropy,
								Some(("J", "mol⋅K")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"J/mol*K",
								UnitKind::MolarEntropy,
								Some(("J", "mol*K")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"g⋅m/s",
								UnitKind::Momentum,
								Some(("g⋅m", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"g*m/s",
								UnitKind::Momentum,
								Some(("g*m", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT,
				),
				//
				//
				UnitDef::new(
								"g/m2",
								UnitKind::AreaDensity,
								Some(("g", "m2")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"g/m3",
								UnitKind::Density,
								Some(("g", "m3")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"m/s",
								UnitKind::Velocity,
								Some(("m", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/h",
								UnitKind::Velocity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s2",
								UnitKind::Acceleration,
								Some(("m", "s2")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s3",
								UnitKind::Jerk,
								Some(("m", "s3")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s4",
								UnitKind::Snap,
								Some(("m", "s4")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s5",
								UnitKind::Crackle,
								Some(("m", "s5")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"m/s6",
								UnitKind::Pop,
								Some(("m", "s6")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"bit",
								UnitKind::Information,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Byte",
								UnitKind::Information,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 8.0, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"bit/s",
								UnitKind::BitRate,
								Some(("bit", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Byte/s",
								UnitKind::BitRate,
								Some(("B", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 8.0, ..UnitProps::DEFAULT }
				),
				//
				UnitDef::new(
								"Bel",
								UnitKind::LogarithmicRatio,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"pH",
								UnitKind::Acidity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"t",
								UnitKind::Mass,
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
								UnitKind::Length,
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
								UnitKind::Length,
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
								UnitKind::Length,
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
								UnitKind::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"rem",
								UnitKind::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"pt",
								UnitKind::Length,
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
								UnitKind::Length,
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
								UnitKind::Length,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				//
				UnitDef::new(
								"min",
								UnitKind::Time,
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
								UnitKind::Time,
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
								UnitKind::Time,
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
								UnitKind::Time,
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
								UnitKind::Time,
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
								UnitKind::Time,
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
								UnitKind::Velocity,
								Some(("m", "s")),
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps { scale: 0.3048, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"mi/h",
								UnitKind::Velocity,
								Some(("m", "s")),
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps { scale: 0.44704, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"kn",
								UnitKind::Velocity,
								Some(("m", "s")),
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps { scale: 0.514444, ..UnitProps::DEFAULT }
				),
				//
				UnitDef::new(
								"K",
								UnitKind::Temperature,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				@multi ["deg", "\u{00B0}"] "C", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								offset: 273.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "F", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 5.0 / 9.0,
								offset: 255.3722222222222,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "De", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: -2.0 / 3.0,
								offset: 373.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Ra", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 5.0 / 9.0,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "N", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 100.0 / 33.0,
								offset: 273.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "D", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: -2.0 / 3.0,
								offset: 373.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Re", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 1.25,
								offset: 273.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Ro", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 40.0 / 21.0,
								offset: 258.864286,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "L", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								offset: 20.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "W", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 24.857191,
								offset: 542.15,
								..UnitProps::DEFAULT
				},
				@multi ["deg", "\u{00B0}"] "Da", UnitKind::Temperature, (PrefixGroup::None, PrefixGroup::None), UnitProps {
								scale: 373.15,
								offset: 273.15,
								mode: CalcMode::Exponential,
								..UnitProps::DEFAULT
				},
				//
				UnitDef::new(
								"V",
								UnitKind::ElectricVoltage,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"A",
								UnitKind::ElectricCurrent,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W",
								UnitKind::Power,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"J/s",
								UnitKind::Power,
								Some(("J", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Ω",
								UnitKind::ElectricResistance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"ohm",
								UnitKind::ElectricResistance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"S",
								UnitKind::ElectricConductance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"siemens",
								UnitKind::ElectricConductance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"C",
								UnitKind::ElectricCharge,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"F",
								UnitKind::ElectricCapacitance,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"J",
								UnitKind::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W*s",
								UnitKind::Energy,
								Some(("W", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W⋅s",
								UnitKind::Energy,
								Some(("W", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"W*s",
								UnitKind::Energy,
								Some(("W", "s")),
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"Wh",
								UnitKind::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"eV",
								UnitKind::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1.602176634e-19, ..UnitProps::DEFAULT }
				),
				UnitDef::new(
								"erg",
								UnitKind::Energy,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps { scale: 1e-7, ..UnitProps::DEFAULT }
				),
				//
				UnitDef::new(
								"Pa",
								UnitKind::Pressure,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"N",
								UnitKind::Force,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"lm",
								UnitKind::LuminousIntensity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"lx",
								UnitKind::LuminousIntensity,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				UnitDef::new(
								"cd",
								UnitKind::LuminousFlux,
								None,
								PrefixGroup::SI,
								PrefixGroup::SI,
								UnitProps::DEFAULT
				),
				//
				UnitDef::new(
								"%",
								UnitKind::Percent,
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
								UnitKind::Permille,
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
								UnitKind::PerTenThousand,
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
								UnitKind::PartPerMillion,
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
								UnitKind::PartPerBillion,
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
								UnitKind::PartPerTrillion,
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
								UnitKind::Fraction,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"deg",
								UnitKind::Degree,
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
								UnitKind::Degree,
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
								UnitKind::Radian,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
				UnitDef::new(
								"D",
								UnitKind::SpaceDimension,
								None,
								PrefixGroup::None,
								PrefixGroup::None,
								UnitProps::DEFAULT,
				),
];

pub static UNITS_TREE: LazyLock<UnitTree> = LazyLock::new(|| build_unit_tree(UNITS));
pub static UNIT_LOOKUP: LazyLock<BTreeMap<String, UnitKind>> = LazyLock::new(|| {
	let mut map = BTreeMap::new();

	// Предварительно готовим префиксы, чтобы не фильтровать PREFIXES в каждом цикле
	let mut grouped_p: BTreeMap<PrefixGroup, Vec<&str>> = BTreeMap::new();
	for (p_sym, _, p_group) in PREFIXES {
		grouped_p.entry(*p_group).or_default().push(*p_sym);
	}

	for unit in UNITS {
		// Базовый символ
		map.insert(unit.symbol.to_string(), unit.dimension);

		// Символы с префиксами
		if let Some(prefixes) = grouped_p.get(&unit.numerator_group) {
			for p_sym in prefixes {
				let mut full_sym = String::with_capacity(p_sym.len() + unit.symbol.len());
				full_sym.push_str(p_sym);
				full_sym.push_str(unit.symbol);
				map.insert(full_sym, unit.dimension);
			}
		}
	}
	map
});

pub fn build_unit_tree(units: &[UnitDef]) -> UnitTree {
	let mut tree = UnitTree::default();
	const SEPARATORS: &[&str] = &["/", "*", "⋅"];

	let mut gp: BTreeMap<PrefixGroup, Vec<&str>> = BTreeMap::new();
	for (s, _, g) in PREFIXES {
		gp.entry(*g).or_default().push(*s);
	}

	for list in gp.values_mut() {
		list.push("");
	}

	let default_prefix = vec![""];

	let mut buf = String::with_capacity(64);

	for unit in units {
		if unit.symbol.is_empty() {
			continue;
		}

		tree.insert(unit.symbol);

		if let Some((n_base, d_base)) = unit.parts {
			let n_prefixes = gp.get(&unit.numerator_group).unwrap_or(&default_prefix);
			let d_prefixes = gp.get(&unit.denominator_group).unwrap_or(&default_prefix);

			for p_n in n_prefixes {
				for p_d in d_prefixes {
					if p_n.is_empty() && p_d.is_empty() {
						continue;
					}

					for sep in SEPARATORS {
						buf.clear();
						buf.push_str(p_n);
						buf.push_str(n_base);
						buf.push_str(sep);
						buf.push_str(p_d);
						buf.push_str(d_base);
						tree.insert(&buf);
					}
				}
			}
		} else {
			if let Some(prefixes) = gp.get(&unit.numerator_group) {
				for p_sym in prefixes {
					if p_sym.is_empty() {
						continue;
					}
					buf.clear();
					buf.push_str(p_sym);
					buf.push_str(unit.symbol);
					tree.insert(&buf);
				}
			}
		}
	}
	tree
}
