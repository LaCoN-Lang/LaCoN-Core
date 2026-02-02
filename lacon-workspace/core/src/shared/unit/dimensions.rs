use super::formulas::{DimensionData, DimensionFormula, DimensionMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
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

impl Dimension {
    pub fn info(&self) -> DimensionData {
        use Dimension::*;
        use DimensionMode::*;

        // Вспомогательная функция для сокращения записи
        let linear = |n: &'static [(Dimension, i8)], d: &'static [(Dimension, i8)]| DimensionData {
            formula: DimensionFormula { num: n, den: d },
            mode: Linear,
        };

        match self {
            // --- Линейные производные ---
            Frequency => linear(&[(Scalar, 1)], &[(Time, 1)]),
            Velocity => linear(&[(Length, 1)], &[(Time, 1)]),
            Force => linear(&[(Mass, 1), (Length, 1)], &[(Time, 2)]),
            Energy => linear(&[(Mass, 1), (Length, 2)], &[(Time, 2)]),

            // --- IT и Битрейт ---
            BitRate => linear(&[(Information, 1)], &[(Time, 1)]),

            // Базовые величины (Length, Mass, Time и т.д.)
            _ => DimensionData::DEFAULT,
        }
    }
}

impl Dimension {
    pub fn formula(&self) -> DimensionFormula {
        use Dimension::*;
        match self {
            // --- Механика и Кинематика ---
            Frequency => DimensionFormula {
                num: &[(Scalar, 1)],
                den: &[(Time, 1)],
            },
            Area => DimensionFormula {
                num: &[(Length, 2)],
                den: &[],
            },
            Volume => DimensionFormula {
                num: &[(Length, 3)],
                den: &[],
            },

            Velocity => DimensionFormula {
                num: &[(Length, 1)],
                den: &[(Time, 1)],
            },
            Acceleration => DimensionFormula {
                num: &[(Length, 1)],
                den: &[(Time, 2)],
            },
            Jerk => DimensionFormula {
                num: &[(Length, 1)],
                den: &[(Time, 3)],
            },
            Snap => DimensionFormula {
                num: &[(Length, 1)],
                den: &[(Time, 4)],
            },
            Crackle => DimensionFormula {
                num: &[(Length, 1)],
                den: &[(Time, 5)],
            },
            Pop => DimensionFormula {
                num: &[(Length, 1)],
                den: &[(Time, 6)],
            },

            Momentum => DimensionFormula {
                num: &[(Mass, 1), (Length, 1)],
                den: &[(Time, 1)],
            },
            Force => DimensionFormula {
                num: &[(Mass, 1), (Length, 1)],
                den: &[(Time, 2)],
            },
            Pressure => DimensionFormula {
                num: &[(Mass, 1)],
                den: &[(Length, 1), (Time, 2)],
            },
            Density => DimensionFormula {
                num: &[(Mass, 1)],
                den: &[(Length, 3)],
            },
            Energy => DimensionFormula {
                num: &[(Mass, 1), (Length, 2)],
                den: &[(Time, 2)],
            },

            // --- Электричество ---
            Power => DimensionFormula {
                num: &[(Mass, 1), (Length, 2)],
                den: &[(Time, 3)],
            },
            ElectricCharge => DimensionFormula {
                num: &[(ElectricCurrent, 1), (Time, 1)],
                den: &[],
            },
            ElectricVoltage => DimensionFormula {
                num: &[(Mass, 1), (Length, 2)],
                den: &[(ElectricCurrent, 1), (Time, 3)],
            },
            ElectricResistance => DimensionFormula {
                num: &[(Mass, 1), (Length, 2)],
                den: &[(ElectricCurrent, 2), (Time, 3)],
            },
            ElectricConductance => DimensionFormula {
                num: &[(ElectricCurrent, 2), (Time, 3)],
                den: &[(Mass, 1), (Length, 2)],
            },
            ElectricCapacitance => DimensionFormula {
                num: &[(ElectricCurrent, 2), (Time, 4)],
                den: &[(Mass, 1), (Length, 2)],
            },

            // --- Свет ---
            LuminousFlux => DimensionFormula {
                num: &[(LuminousIntensity, 1)],
                den: &[],
            },
            Illuminance => DimensionFormula {
                num: &[(LuminousIntensity, 1)],
                den: &[(Length, 2)],
            },

            BitRate => DimensionFormula {
                num: &[(Information, 1)],
                den: &[(Time, 1)],
            },

            DataDensity => DimensionFormula {
                num: &[(Information, 1)],
                den: &[(Length, 2)],
            },

            // Всё остальное (Scalar, Length, Time, Mass и т.д.) не имеет формулы разложения
            _ => DimensionFormula { num: &[], den: &[] },
        }
    }
}
