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
        phase_region: PhaseRegion,
    },
    SatPQuery {
        // Pa
        pressure: f64,
        phase_region: PhaseRegion,
    },
}

pub fn get_steam_table_entry(query: SteamQuery) -> PtvEntry {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
