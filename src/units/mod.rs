use core::fmt;

pub struct RawUnit {
    value: f64,
    unit_display: String,
}

pub enum ParseUnitError {
    UnknownUnit(String),
}

pub trait Unit<Si>: fmt::Debug + TryFrom<RawUnit> + Into<RawUnit> {
    fn into_si_unit(&self) -> Si;
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
        pub type $si_unit_name = f64;
        $(
        pub type $unit_name = f64;
        )*

        #[derive(Debug, PartialEq)]
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
                    $si_str  => Ok($type_name::$si_unit_name(value)),
                    $(
                    $unit_str  => Ok($type_name::$unit_name(value)),
                    )*
                    _ => Err(ParseUnitError::UnknownUnit(unit_display)),
                }
            }
        }

        impl Into<RawUnit> for $type_name {
            fn into(self) -> RawUnit {
                match self {
                    $type_name::$si_unit_name(value) => RawUnit {
                        value,
                        unit_display: $si_str.to_owned(),
                    },
                    $(
                    $type_name::$unit_name(value) => RawUnit {
                        value,
                        unit_display: $unit_str.to_owned(),
                    },
                    )*
                }
            }
        }

        impl Unit<$si_unit_name> for $type_name {
            fn into_si_unit(&self) -> $si_unit_name {
                match *self {
                    $type_name::$si_unit_name(x) => x,
                    $(
                    $type_name::$unit_name(x) => {
                        let f = $unit_function;
                        f(x)
                    },
                    )*
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
        assert_approx_eq!(Length::M(1f64).into_si_unit(), 1f64);
        assert_approx_eq!(Length::Ft(1f64).into_si_unit(), 1f64 / 3.28084);
        assert_approx_eq!(
            Length::Inches(1f64).into_si_unit(),
            1f64 / (3.28084 * 12f64)
        );
    }

    #[test]
    fn mass_conversion() {
        assert_approx_eq!(Mass::Kg(1f64).into_si_unit(), 1f64);
        assert_approx_eq!(Mass::G(1f64).into_si_unit(), 1000f64);
        assert_approx_eq!(Mass::Lbsm(1f64).into_si_unit(), 2.20462f64);
    }
}
