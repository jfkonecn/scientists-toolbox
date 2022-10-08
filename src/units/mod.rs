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
            value: f64
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
            value: f64
        }

        #[allow(dead_code)]
        impl $unit_name {
            pub fn new(value: f64) -> $unit_name {
                $unit_name {
                    value
                }
            }

            pub fn abs(self) -> f64 {
                self.value
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
            |x| ((x - 273.15f64) * 9f64 / 5f64) + 32f64,
            |x| ((x - 32f64) * 5f64 / 9f64) + 273.15,
        },
        R {
            "°R",
            "degrees rankine",
            |x| x * 5f64 / 9f64,
            |x| x * 9f64 / 5f64,
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
        assert_approx_eq!(
            Length::Ft(Ft::new(1f64)).into_si_unit(),
            M::new(1f64 / 3.28084)
        );
        assert_approx_eq!(
            Length::Inches(Inches::new(1f64)).into_si_unit(),
            M::new(1f64 / (3.28084 * 12f64))
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
        assert_approx_eq!(Temperature::C(C::new(1f64)).into_si_unit(), K::new(274.15));
        assert_approx_eq!(Temperature::F(F::new(1f64)).into_si_unit(), K::new(255.928));
        assert_approx_eq!(Temperature::R(R::new(1f64)).into_si_unit(), K::new(0.556));
    }
}
