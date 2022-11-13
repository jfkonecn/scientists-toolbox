use core::fmt;

pub struct RawUnit {
    pub value: f64,
    pub unit_display: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseUnitError {
    UnknownUnit(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnitLabel {
    pub abbreviation: String,
    pub plural: String,
}

pub trait Unit: fmt::Debug + TryFrom<RawUnit> + Into<RawUnit> {
    type Si;
    fn into_si_unit(&self) -> Self::Si;
    fn list_unit_labels() -> Vec<UnitLabel>;
    fn get_si_unit_label() -> UnitLabel;
    fn get_value(&self) -> f64;
    fn try_convert(&self, abbreviation_label: String) -> Result<Self, ParseUnitError>;
}

macro_rules! units {
    ($($type_name:ident {
        $si_unit_name:ident {
            $si_abbreviation:literal,
            $si_plural:literal,
        }
        $(,$unit_name:ident {
            $unit_abbreviation:literal,
            $unit_plural:literal,
            $unit_function:expr,
            $from_si_function:expr,
        })*
    })*) => {
        $(
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub struct $si_unit_name {
            pub value: f64
        }

        #[allow(dead_code)]
        impl $si_unit_name {
            pub fn new(value: f64) -> $si_unit_name {
                $si_unit_name {
                    value
                }
            }

            pub fn abs(self) -> f64 {
                self.value
            }
        }
        impl std::ops::Sub for $si_unit_name {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self {
                    value: self.value - other.value,
                }
            }
        }

        $(
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub struct $unit_name {
            pub value: f64
        }

        #[allow(dead_code)]
        impl $unit_name {
            pub fn new(value: f64) -> $unit_name {
                $unit_name {
                    value
                }
            }

            pub fn abs(self) -> f64 {
                self.value.abs()
            }
        }

        impl std::ops::Sub for $unit_name {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self {
                    value: self.value - other.value,
                }
            }
        }
        )*

        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $type_name {
            $si_unit_name($si_unit_name),
            $(
            $unit_name($unit_name),
            )*
        }

        #[allow(dead_code)]
        impl $type_name {
            pub fn abs(self) -> f64 {
                self.into_si_unit().value.abs()
            }
        }

        impl std::ops::Sub for $type_name {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                let diff = self.into_si_unit() - other.into_si_unit();
                $type_name::$si_unit_name(diff)
            }
        }

        impl TryFrom<RawUnit> for $type_name {
            type Error = ParseUnitError;

            fn try_from(
                RawUnit {
                    value,
                    unit_display,
                }: RawUnit,
            ) -> Result<Self, Self::Error> {
                match unit_display.as_str() {
                    $si_abbreviation  => Ok($type_name::$si_unit_name($si_unit_name::new(value))),
                    $(
                    $unit_abbreviation  => Ok($type_name::$unit_name($unit_name::new(value))),
                    )*
                    _ => Err(ParseUnitError::UnknownUnit(unit_display)),
                }
            }
        }

        impl Into<RawUnit> for $type_name {
            fn into(self) -> RawUnit {
                match self {
                    $type_name::$si_unit_name($si_unit_name {value}) => RawUnit {
                        value,
                        unit_display: $si_abbreviation.to_owned(),
                    },
                    $(
                    $type_name::$unit_name($unit_name {value}) => RawUnit {
                        value,
                        unit_display: $unit_abbreviation.to_owned(),
                    },
                    )*
                }
            }
        }

        impl Unit for $type_name {
            type Si = $si_unit_name;
            fn into_si_unit(&self) -> $si_unit_name {
                match self {
                    $type_name::$si_unit_name(x) => (*x).clone(),
                    $(
                    $type_name::$unit_name($unit_name {value}) => {
                        let f = $unit_function;
                        $si_unit_name::new(f(value))
                    },
                    )*
                }
            }

            fn list_unit_labels() -> Vec<UnitLabel> {
                vec![
                    UnitLabel {
                        abbreviation: $si_abbreviation.to_owned(),
                        plural: $si_plural.to_owned(),
                    },
                    $(
                        UnitLabel {
                            abbreviation: $unit_abbreviation.to_owned(),
                            plural: $unit_plural.to_owned(),
                        },
                    )*
                ]
            }

            fn get_si_unit_label() -> UnitLabel {
                UnitLabel {
                    abbreviation: $si_abbreviation.to_owned(),
                    plural: $si_plural.to_owned(),
                }
            }

            fn get_value(&self) -> f64 {
                match self {
                    $type_name::$si_unit_name(x) => (*x).value,
                    $(
                    $type_name::$unit_name($unit_name {value}) => {
                        *value
                    },
                    )*
                }
            }

            fn try_convert(&self, unit_display: String) -> Result<Self, ParseUnitError> {
                let si_unit = self.into_si_unit();
                let value = si_unit.value;

                match unit_display.as_str() {
                    $si_abbreviation  => Ok($type_name::$si_unit_name($si_unit_name::new(value))),
                    $(
                    $unit_abbreviation  =>  {
                        let f = $from_si_function;
                        Ok($type_name::$unit_name($unit_name::new(f(value))))
                    },
                    )*
                    _ => Err(ParseUnitError::UnknownUnit(unit_display)),
                }
            }
        }
        )*

    };
}

units! {
    Length {
        M {
            "m",
            "meters",
        },
        Km {
            "km",
            "kilometers",
            |x| x * 1000f64,
            |x| x / 1000f64,
        },
        Ft {
            "ft",
            "feet",
            |x| x / 3.28084,
            |x| x * 3.28084,
        },
        Inches {
            "in",
            "inches",
            |x| x / (3.28084 * 12f64),
            |x| x * (3.28084 * 12f64),
        }
    }
    Area {
        M2 {
            "m²",
            "meters squared",
        },
        Km2 {
            "km²",
            "kilometers squared",
            |x| x * (1000f64 * 1000f64),
            |x| x / (1000f64 * 1000f64),
        },
        Ft2 {
            "ft²",
            "feet squared",
            |x| x / (3.28084 * 3.28084),
            |x| x * (3.28084 * 3.28084),
        },
        Inches2 {
            "in²",
            "inches squared",
            |x| x / (3.28084 * 12f64 * 3.28084 * 12f64),
            |x| x * (3.28084 * 12f64 * 3.28084 * 12f64),
        }
    }
    Volume {
        M3 {
            "m³",
            "meters cubed",
        },
        Km3 {
            "km³",
            "kilometers cubed",
            |x| x * (1000f64 * 1000f64 * 1000f64),
            |x| x / (1000f64 * 1000f64 * 1000f64),
        },
        Ft3 {
            "ft³",
            "feet cubed",
            |x| x / (3.28084 * 3.28084 * 3.28084),
            |x| x * (3.28084 * 3.28084 * 3.28084),
        },
        Inches3 {
            "in³",
            "inches cubed",
            |x| x / (3.28084 * 12f64 * 3.28084 * 12f64 * 3.28084 * 12f64),
            |x| x * (3.28084 * 12f64 * 3.28084 * 12f64 * 3.28084 * 12f64),
        }
    }
    VolumetricFlowRate {
        M3PerSec {
            "m³/sec",
            "meters cubed per second",
        },
        M3PerMin {
            "m³/min",
            "meters cubed per minute",
            |x| x * (60f64),
            |x| x / (60f64),
        },
        Ft3PerSec {
            "ft³/sec",
            "feet cubed per second",
            |x| x / (3.28084 * 3.28084 * 3.28084),
            |x| x * (3.28084 * 3.28084 * 3.28084),
        },
        Ft3PerMin {
            "ft³/min",
            "feet cubed per minute",
            |x| x * (60f64 / (3.28084 * 3.28084 * 3.28084)),
            |x| x * ((3.28084 * 3.28084 * 3.28084) / 60f64),
        }
    }
    Mass {
        Kg {
            "kg",
            "kilograms",
        },
        G {
            "g",
            "grams",
            |x| x * 1000f64,
            |x| x / 1000f64,
        },
        Lbsm {
            "Lbsₘ",
            "pounds mass",
            |x| x * 2.20462f64,
            |x| x / 2.20462f64,
        }
    }
    Temperature {
        K {
            "K",
            "kelvin",
        },
        C {
            "°C",
            "degrees celsius",
            |x| x + 273.15,
            |x| x - 273.15,
        },
        F {
            "°F",
            "degrees fahrenheit",
            |x| ((x - 32f64) * 5f64 / 9f64) + 273.15,
            |x| ((x - 273.15f64) * 9f64 / 5f64) + 32f64,
        },
        R {
            "°R",
            "degrees rankine",
            |x| x * 5f64 / 9f64,
            |x| x * 9f64 / 5f64,
        }
    }
    Pressure {
        Pa {
            "Pa",
            "pascals",
        },
        KPa {
            "kPa",
            "kilopascals",
            |x| x * 1000f64,
            |x| x / 1000f64,
        },
        Lbf {
            "lbf/in²",
            "pounds-force per square inch",
            |x| x * 6894.76,
            |x| x / 6894.76,
        }
    }
    EnergyPerMass {
        JPerKg {
            "J/kg",
            "joules pr kilogram",
        },
        BtuPerLbsm {
            "BTU/Lbsₘ",
            "british thermal units per pounds mass",
            |x| x * (2.2 * 1055.06),
            |x| x / (2.2 * 1055.06),
        }
    }
    EnergyPerMassTemperature {
        JPerKgK {
            "J/(kg · K)",
            "joules per kilogram kelvin",
        },
        BtuPerLbsmR {
            "BTU/(Lbsₘ · °R)",
            "british thermal units per pounds mass degrees rankine",
            |x| x * (2.2 * 1055.06 * 5f64 / 9f64),
            |x| x / (2.2 * 1055.06 * 5f64 / 9f64),
        }
    }
    Velocity {
        MPerSec {
            "m/s",
            "meters per second",
        },
        FtPerSec {
            "ft/s",
            "feet per second",
            |x| x / 3.28084,
            |x| x * 3.28084,
        }
    }
    SpecificVolume {
        M3PerKg {
            "m³/kg",
            "cubic meters per kilogram",
        },
        Ft3PerLbsm {
            "ft³/Lbsₘ",
            "cubic feet per pounds mass",
            |x| x * (2.20462 / 35.3147),
            |x| x / (2.20462 / 35.3147),
        }
    }
    Density {
        KgPerM3 {
            "kg/m³",
            "cubic meters per kilogram",
        },
        LbsmPerFt3 {
            "Lbsₘ/ft³",
            "pounds mass per cubic feet",
            |x| x * (35.3147 / 2.20462),
            |x| x / (35.3147 / 2.20462),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn length_conversion() {
        assert_approx_eq!(Length::M(M::new(1f64)).into_si_unit(), M::new(1f64));
        assert_approx_eq!(Length::Ft(Ft::new(1f64)), Length::M(M::new(1f64 / 3.28084)));
        assert_approx_eq!(
            Length::Inches(Inches::new(1f64)),
            Length::M(M::new(1f64 / (3.28084 * 12f64)))
        );
    }

    #[test]
    fn area_conversion() {
        assert_approx_eq!(Area::M2(M2::new(1f64)).into_si_unit(), M2::new(1f64));
        assert_approx_eq!(
            Area::Ft2(Ft2::new(1f64)),
            Area::M2(M2::new(1f64 / (3.28084 * 3.28084)))
        );
        assert_approx_eq!(
            Area::Inches2(Inches2::new(1f64)),
            Area::M2(M2::new(1f64 / (3.28084 * 12f64 * 3.28084 * 12f64)))
        );
    }

    #[test]
    fn volume_conversion() {
        assert_approx_eq!(Volume::M3(M3::new(1f64)).into_si_unit(), M3::new(1f64));
        assert_approx_eq!(
            Volume::Ft3(Ft3::new(1f64)),
            Volume::M3(M3::new(1f64 / (3.28084 * 3.28084 * 3.28084)))
        );
        assert_approx_eq!(
            Volume::Inches3(Inches3::new(1f64)),
            Volume::M3(M3::new(
                1f64 / (3.28084 * 12f64 * 3.28084 * 12f64 * 3.28084 * 12f64)
            ))
        );
    }

    #[test]
    fn volumetric_flow_rate_conversion() {
        assert_approx_eq!(
            VolumetricFlowRate::M3PerSec(M3PerSec::new(1f64)).into_si_unit(),
            M3PerSec::new(1f64)
        );
        assert_approx_eq!(
            VolumetricFlowRate::M3PerSec(M3PerSec::new(1f64 * 60f64)),
            VolumetricFlowRate::M3PerMin(M3PerMin::new(1f64))
        );
        assert_approx_eq!(
            VolumetricFlowRate::Ft3PerSec(Ft3PerSec::new(1f64)),
            VolumetricFlowRate::M3PerSec(M3PerSec::new(1f64 / (3.28084 * 3.28084 * 3.28084)))
        );
        assert_approx_eq!(
            VolumetricFlowRate::Ft3PerMin(Ft3PerMin::new(1f64)),
            VolumetricFlowRate::M3PerSec(M3PerSec::new(60f64 / (3.28084 * 3.28084 * 3.28084)))
        );
    }

    #[test]
    fn mass_conversion() {
        assert_approx_eq!(Mass::Kg(Kg::new(1f64)).into_si_unit(), Kg::new(1f64));
        assert_approx_eq!(Mass::G(G::new(1f64)).into_si_unit(), Kg::new(1000f64));
        assert_approx_eq!(
            Mass::Lbsm(Lbsm::new(1f64)).into_si_unit(),
            Kg::new(2.20462f64)
        );
    }

    #[test]
    fn temperature_conversion() {
        assert_approx_eq!(Temperature::K(K::new(1f64)).into_si_unit(), K::new(1f64));
        assert_approx_eq!(Temperature::C(C::new(1f64)), Temperature::K(K::new(274.15)));
        assert_approx_eq!(
            Temperature::F(F::new(1f64)),
            Temperature::K(K::new(255.92777777777775))
        );
        assert_approx_eq!(
            Temperature::R(R::new(200f64)),
            Temperature::K(K::new(111.11111111111111))
        );
    }

    #[test]
    fn pressure_conversion() {
        assert_approx_eq!(Pressure::Pa(Pa::new(1f64)).into_si_unit(), Pa::new(1f64));
        assert_approx_eq!(
            Pressure::KPa(KPa::new(1f64)).into_si_unit(),
            Pa::new(1000f64)
        );
        assert_approx_eq!(
            Pressure::KPa(KPa::new(1f64)),
            Pressure::Pa(Pa::new(1000f64))
        );
        assert_approx_eq!(
            Pressure::Lbf(Lbf::new(1f64)),
            Pressure::Pa(Pa::new(6894.76))
        );
    }

    #[test]
    fn energy_per_mass_conversion() {
        assert_approx_eq!(
            EnergyPerMass::JPerKg(JPerKg::new(1f64)).into_si_unit(),
            JPerKg::new(1f64)
        );
        assert_approx_eq!(
            EnergyPerMass::BtuPerLbsm(BtuPerLbsm::new(1f64)),
            EnergyPerMass::JPerKg(JPerKg::new(2.2 * 1055.06))
        );
    }

    #[test]
    fn energy_per_mass_temperature_conversion() {
        assert_approx_eq!(
            EnergyPerMassTemperature::JPerKgK(JPerKgK::new(1f64)).into_si_unit(),
            JPerKgK::new(1f64)
        );
        assert_approx_eq!(
            EnergyPerMassTemperature::BtuPerLbsmR(BtuPerLbsmR::new(1f64)),
            EnergyPerMassTemperature::JPerKgK(JPerKgK::new(2.2 * 1055.06 * 5f64 / 9f64))
        );
    }

    #[test]
    fn velocity_conversion() {
        assert_approx_eq!(
            Velocity::MPerSec(MPerSec::new(1f64)).into_si_unit(),
            MPerSec::new(1f64)
        );
        assert_approx_eq!(
            Velocity::FtPerSec(FtPerSec::new(1f64)),
            Velocity::MPerSec(MPerSec::new(1f64 / 3.28084))
        );
    }

    #[test]
    fn specific_volume_conversion() {
        assert_approx_eq!(
            SpecificVolume::M3PerKg(M3PerKg::new(1f64)).into_si_unit(),
            M3PerKg::new(1f64)
        );
        assert_approx_eq!(
            SpecificVolume::Ft3PerLbsm(Ft3PerLbsm::new(1f64)),
            SpecificVolume::M3PerKg(M3PerKg::new(2.20462 / 35.3147))
        );
    }

    #[test]
    fn density_conversion() {
        assert_approx_eq!(
            Density::KgPerM3(KgPerM3::new(1f64)).into_si_unit(),
            KgPerM3::new(1f64)
        );
        assert_approx_eq!(
            Density::LbsmPerFt3(LbsmPerFt3::new(1f64)),
            Density::KgPerM3(KgPerM3::new(35.3147 / 2.20462))
        );
    }
}
