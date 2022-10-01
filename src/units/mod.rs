use core::fmt;

pub struct RawUnit {
    pub value: f64,
    pub unit_display: String,
}

pub enum ParseUnitError {
    UnknownUnit(String),
}

pub trait Unit: fmt::Debug + TryFrom<RawUnit> + Into<RawUnit> {
    type Si;
    fn into_si_unit(&self) -> Self::Si;
    fn list_unit_strings() -> Vec<String>;
    fn get_si_unit_string() -> String;
}

macro_rules! units {
    ($($type_name:ident {
        $si_unit_name:ident {
            $si_str:literal,
        }
        $(,$unit_name:ident {
            $unit_str:literal,
            $unit_function:expr
        })*
    })*) => {
        $(
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub struct $si_unit_name {
            value: f64
        }

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

        #[derive(Debug, PartialEq, Clone)]
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
                    $si_str  => Ok($type_name::$si_unit_name($si_unit_name::new(value))),
                    $(
                    $unit_str  => Ok($type_name::$unit_name($unit_name::new(value))),
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
                        unit_display: $si_str.to_owned(),
                    },
                    $(
                    $type_name::$unit_name($unit_name {value}) => RawUnit {
                        value,
                        unit_display: $unit_str.to_owned(),
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

            fn list_unit_strings() -> Vec<String> {
                vec![$si_str.to_owned(), $($unit_str.to_owned(),)*]
            }

            fn get_si_unit_string() -> String {
                $si_str.to_owned()
            }
        }
        )*
    };
}

units! {
    Length {
        M {
            "m",
        },
        Ft {
            "ft",
            |x| x / 3.28084
        },
        Inches {
            "in",
            |x| x / (3.28084 * 12f64)
        }
    }
    Mass {
        Kg {
            "kg",
        },
        G {
            "g",
            |x| x * 1000f64
        },
        Lbsm {
            "Lbsₘ",
            |x| x * 2.20462f64
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
}
