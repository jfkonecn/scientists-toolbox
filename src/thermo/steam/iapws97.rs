// https://github.com/jfkonecn/thermo/blob/feature/issue-42/thermo/steam_properties.py
use crate::thermo::types::*;

enum Iapws97Region {
    OutOfRange,
    Region1,
    Region2,
    Region3,
    Region4,
    Region5,
}

pub enum SteamQuery {
    PtQuery {
        // Pa
        pressure: f64,
        // K
        temperature: f64,
    },
    SatTQuery {
        // K
        temperature: f64,
        phase_region: NonCriticalPhaseRegion,
    },
    SatPQuery {
        // Pa
        pressure: f64,
        phase_region: NonCriticalPhaseRegion,
    },
    EntropyPQuery {
        // J/(kg * K)
        entropy: f64,
        // Pa
        pressure: f64,
    },
    EnthalpyPQuery {
        // J/kg
        enthalpy: f64,
        // Pa
        pressure: f64,
    },
}

pub fn get_steam_table_entry(query: SteamQuery) -> PtvEntry {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! get_steam_table_entry_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_steam_table_entry(input));
            }
        )*
        }
    }
    get_steam_table_entry_tests! {
        steam_table_1: (
            SteamQuery::PtQuery {
                temperature: 750.0,
                pressure: 78.309563916917e6,
            },
            PtvEntry {
                temperature: 750.0,
                pressure: 78.309563916917e6,
                phase_region: PhaseRegion::SupercriticalFluid,
                internal_energy: 2102.069317626429e3,
                enthalpy: 2258.688445460262e3,
                entropy: 4.469719056217e3,
                cv: 2.71701677121e3,
                cp: 6.341653594791e3,
                speed_of_sound: 760.696040876798,
                specific_volume: 1.0 / 500.0,
            }
        ),
        steam_table_2: (
            SteamQuery::PtQuery {
                temperature: 473.15,
                pressure: 40e6,
            },
            PtvEntry {
                temperature: 473.15,
                pressure: 40e6,
                phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid),
                internal_energy: 825.228016170348e3,
                enthalpy: 870.124259682489e3,
                entropy: 2.275752861241e3,
                cv: 3.292858637199e3,
                cp: 4.315767590903e3,
                speed_of_sound: 1457.418351596083,
                specific_volume: 0.001122406088,
            }
        ),
        steam_table_3: (
            SteamQuery::PtQuery {
                temperature: 2000.0,
                pressure: 30e6,
            },
            PtvEntry {
                temperature: 2000.0,
                pressure: 30e6,
                phase_region: PhaseRegion::SupercriticalFluid,
                internal_energy: 5637.070382521894e3,
                enthalpy: 6571.226038618478e3,
                entropy: 8.536405231138e3,
                cv: 2.395894362358e3,
                cp: 2.885698818781e3,
                speed_of_sound: 1067.369478777425,
                specific_volume: 0.03113852187,
            }
        ),
        steam_table_4: (
            SteamQuery::PtQuery {
                temperature: 823.15,
                pressure: 14e6,
            },
            PtvEntry {
                temperature: 823.15,
                pressure: 14e6,
                phase_region: PhaseRegion::Gas,
                internal_energy: 3114.302136294585e3,
                enthalpy: 3460.987255128561e3,
                entropy: 6.564768889364e3,
                cv: 1.892708832325e3,
                cp: 2.666558503968e3,
                speed_of_sound: 666.050616844223,
                specific_volume: 0.024763222774,
            }
        ),
        steam_table_5: (
            SteamQuery::SatPQuery {
                pressure: 0.2e6,
                phase_region: NonCriticalPhaseRegion::Liquid,
            },
            PtvEntry {
                temperature: 393.361545936488,
                pressure: 0.2e6,
                phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid),
                internal_energy: 504471.741847973,
                enthalpy: 504683.84552926,
                entropy: 1530.0982011075,
                cv: 3666.99397284121,
                cp: 4246.73524917536,
                speed_of_sound: 1520.69128792808,
                specific_volume: 0.00106051840643552,
            }
        ),
        steam_table_6: (
            SteamQuery::SatPQuery {
                pressure: 0.2e6,
                phase_region: NonCriticalPhaseRegion::Vapor,
            },
            PtvEntry {
                temperature: 393.361545936488,
                pressure: 0.2e6,
                phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Vapor),
                internal_energy: 2529094.32835793,
                enthalpy: 2706241.34137425,
                entropy: 7126.8563914686,
                cv: 1615.96336473298,
                cp: 2175.22318865273,
                speed_of_sound: 481.883535821489,
                specific_volume: 0.885735065081644,
            }
        ),
        steam_table_7: (
            SteamQuery::SatTQuery {
                temperature: 393.361545936488,
                phase_region: NonCriticalPhaseRegion::Liquid,
            },
            PtvEntry {
                temperature: 393.361545936488,
                pressure: 0.2e6,
                phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid),
                internal_energy: 504471.741847973,
                enthalpy: 504683.84552926,
                entropy: 1530.0982011075,
                cv: 3666.99397284121,
                cp: 4246.73524917536,
                speed_of_sound: 1520.69128792808,
                specific_volume: 0.00106051840643552,
            }
        ),
        steam_table_8: (
            SteamQuery::SatTQuery {
                temperature: 393.361545936488,
                phase_region: NonCriticalPhaseRegion::Vapor,
            },
            PtvEntry {
                temperature: 393.361545936488,
                pressure: 0.2e6,
                phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Vapor),
                internal_energy: 2529094.32835793,
                enthalpy: 2706241.34137425,
                entropy: 7126.8563914686,
                cv: 1615.96336473298,
                cp: 2175.22318865273,
                speed_of_sound: 481.883535821489,
                specific_volume: 0.885735065081644,
            }
        ),
        steam_table_9: (
            SteamQuery::EntropyPQuery {
                entropy: 4.469719056217e3,
                pressure: 78.309563916917e6,
            },
            PtvEntry {
                temperature: 750.0,
                pressure: 78.309563916917e6,
                phase_region: PhaseRegion::SupercriticalFluid,
                internal_energy: 2102.069317626429e3,
                enthalpy: 2258.688445460262e3,
                entropy: 4.469719056217e3,
                cv: 2.71701677121e3,
                cp: 6.341653594791e3,
                speed_of_sound: 760.696040876798,
                specific_volume: 1.0 / 500.0,
            }
        ),
        steam_table_10: (
            SteamQuery::EntropyPQuery {
                entropy: 2.275752861241e3,
                pressure: 40e6,
            },
            PtvEntry {
                temperature: 473.15,
                pressure: 40e6,
                phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid),
                internal_energy: 825.228016170348e3,
                enthalpy: 870.124259682489e3,
                entropy: 2.275752861241e3,
                cv: 3.292858637199e3,
                cp: 4.315767590903e3,
                speed_of_sound: 1457.418351596083,
                specific_volume: 0.001122406088,
            }
        ),
        steam_table_11: (
            SteamQuery::EntropyPQuery {
                entropy: 8.536405231138e3,
                pressure: 30e6,
            },
            PtvEntry {
                temperature: 2000.0,
                pressure: 30e6,
                phase_region: PhaseRegion::SupercriticalFluid,
                internal_energy: 5637.070382521894e3,
                enthalpy: 6571.226038618478e3,
                entropy: 8.536405231138e3,
                cv: 2.395894362358e3,
                cp: 2.885698818781e3,
                speed_of_sound: 1067.369478777425,
                specific_volume: 0.03113852187,
            }
        ),
        steam_table_12: (
            SteamQuery::EntropyPQuery {
                entropy: 6.564768889364e3,
                pressure: 14e6,
            },
            PtvEntry {
                temperature: 823.15,
                pressure: 14e6,
                phase_region: PhaseRegion::Gas,
                internal_energy: 3114.302136294585e3,
                enthalpy: 3460.987255128561e3,
                entropy: 6.564768889364e3,
                cv: 1.892708832325e3,
                cp: 2.666558503968e3,
                speed_of_sound: 666.050616844223,
                specific_volume: 0.024763222774,
            }
        ),
        steam_table_13: (
            SteamQuery::EntropyPQuery {
                entropy: 6.6858e3,
                pressure: 10e3,
            },
            PtvEntry {
                temperature: 318.957548207023,
                pressure: 10e3,
                phase_region: PhaseRegion::Composite(
                    CompositePhaseRegion::LiquidVapor(
                        LiquidVapor::new(0.8049124470781327, 1.0 - 0.8049124470781327).unwrap()
                    )
                ),
                internal_energy: 1999135.82661328,
                enthalpy: 2117222.94886314,
                entropy: 6.6858e3,
                cv: 1966.28009225455,
                cp: 2377.86300751001,
                speed_of_sound: 655.005141924186,
                specific_volume: 1.0 / 193.16103883,
            }
        ),
        steam_table_14: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 2258.688445460262e3,
                pressure: 78.309563916917e6,
            },
            PtvEntry {
                temperature: 750.0,
                pressure: 78.309563916917e6,
                phase_region: PhaseRegion::SupercriticalFluid,
                internal_energy: 2102.069317626429e3,
                enthalpy: 2258.688445460262e3,
                entropy: 4.469719056217e3,
                cv: 2.71701677121e3,
                cp: 6.341653594791e3,
                speed_of_sound: 760.696040876798,
                specific_volume: 1.0 / 500.0,
            }
        ),
        steam_table_15: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 870.124259682489e3,
                pressure: 40e6,
            },
            PtvEntry {
                temperature: 473.15,
                pressure: 40e6,
                phase_region: PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid),
                internal_energy: 825.228016170348e3,
                enthalpy: 870.124259682489e3,
                entropy: 2.275752861241e3,
                cv: 3.292858637199e3,
                cp: 4.315767590903e3,
                speed_of_sound: 1457.418351596083,
                specific_volume: 0.001122406088,
            }
        ),
        steam_table_16: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 6571.226038618478e3,
                pressure: 30e6,
            },
            PtvEntry {
                temperature: 2000.0,
                pressure: 30e6,
                phase_region: PhaseRegion::SupercriticalFluid,
                internal_energy: 5637.070382521894e3,
                enthalpy: 6571.226038618478e3,
                entropy: 8.536405231138e3,
                cv: 2.395894362358e3,
                cp: 2.885698818781e3,
                speed_of_sound: 1067.369478777425,
                specific_volume: 0.03113852187,
            }
        ),
        steam_table_17: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 3460.987255128561e3,
                pressure: 14e6,
            },
            PtvEntry {
                temperature: 823.15,
                pressure: 14e6,
                phase_region: PhaseRegion::Gas,
                internal_energy: 3114.302136294585e3,
                enthalpy: 3460.987255128561e3,
                entropy: 6.564768889364e3,
                cv: 1.892708832325e3,
                cp: 2.666558503968e3,
                speed_of_sound: 666.050616844223,
                specific_volume: 0.024763222774,
            }
        ),
        steam_table_18: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 2117222.94886314,
                pressure: 10e3,
            },
            PtvEntry {
                temperature: 318.957548207023,
                pressure: 10e3,
                phase_region: PhaseRegion::Composite(
                    CompositePhaseRegion::LiquidVapor(
                        LiquidVapor::new(0.8049124470781327, 1.0 - 0.8049124470781327).unwrap()
                    )
                ),
                internal_energy: 1999135.82661328,
                enthalpy: 2117222.94886314,
                entropy: 6.6858e3,
                cv: 1966.28009225455,
                cp: 2377.86300751001,
                speed_of_sound: 655.005141924186,
                specific_volume: 1.0 / 193.16103883,
            }
        ),
    }
}
