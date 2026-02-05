use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum UnitKind {
	None,
	// Безразмерные величины
	Scalar,           // 1
	Fraction,         // /   \\ FractionUnit
	Percent,          // %   \\ Percentage
	Permille,         // ‰  \\ Permile
	PerTenThousand,   // ‱   \\ PerTenThousand
	PartPerMillion,   // ppm \\ PartPerMillion
	PartPerBillion,   // ppb \\ PartPerBillion
	PartPerTrillion,  // ppt \\ PartPerTrillion
	LogarithmicRatio, // ln \\ LogarithmicRatio
	Acidity,          // pH \\ Acidity

	// Пространственные величины
	Length,         // m   \\ LengthUnit
	Area,           // m2 \\ AreaUnit
	Volume,         // m3 \\ VolumeUnit
	SpaceDimension, // DimensionalUnit
	AreaDensity,    // kg/m2 \\ AreaDensity
	Density,        // kg/m3 \\ Density
	SpecificVolume, // m3/kg \\ SpecificVolume

	// Время и производные
	Time,         // s   \\ TimeUnit
	Frequency,    // Hz  \\ FrequencyUnit
	Velocity,     // m/s \\ SpeedUnit
	Acceleration, // m/s2 \\ AccelerationUnit
	Jerk,         // m/s3 \\ JerkUnit
	Snap,         // m/s4 \\ SnapUnit
	Crackle,      // m/s5 \\ CrackleUnit
	Pop,          // m/s6 \\ PopUnit

	// Масса и вещество
	Mass,               // g  \\ MassUnit
	AmountOfSubstance,  // mol \\ Amount
	MolarConcentration, // mol/l \\ Concentration
	MolarVolume,        // m3/mol \\ MolarVolume
	MolarMass,          // g/mol \\ MolarMass
	MolarEnergy,        // J/mol \\ MolarEnergy
	MolarEntropy,       // J/mol·K \\ MolarEntropy
	Momentum,           // kg*m/s \\ Momentum

	// Энергия, работа, сила
	Energy,   // J  \\ EnergyUnit
	Force,    // N  \\ ForceUnit
	Pressure, // Pa  \\ PressureUnit
	Power,    // W \\ ElectricPower

	// Температура и тепло
	Temperature, // K \\ TemperatureUnit
	Entropy,     // J/K \\ Entropy

	// Электрические величины
	ElectricVoltage,     // V \\ ElectricVoltage
	ElectricCurrent,     // A \\ ElectricCurrent
	ElectricCharge,      // C \\ ElectricCharge
	ElectricResistance,  // Ω \\ ElectricResistance
	ElectricConductance, // S \\ ElectricConductance
	ElectricCapacitance, // F \\ ElectricCapacitance

	// Световые величины
	LuminousIntensity, // cd \\ LuminousIntensity
	LuminousFlux,      // lm \\ LuminousFlux
	Illuminance,       // lx \\ Illuminance

	// Угловые величины
	Degree, // deg \\ AngleDegree
	Radian, // rad \\ AngleRadian

	// Информация
	Information,        // B   \\ InformationSizeUnit
	InformationEntropy, // B/1 \\ InformationEntropy
	DataDensity,        // B/m2 \\ DataDensity
	BitRate,            // bps \\ BitRate
}
