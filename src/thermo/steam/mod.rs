use std::fmt::Display;

use crate::numerical_methods::*;
use crate::thermo::*;

pub mod iapws97;
mod iapws97_constants;
mod water_constants;

#[derive(Copy, PartialEq, Clone, Debug)]
pub struct PtPoint {
    pub pressure: Pressure,
    pub temperature: Temperature,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SteamNonCriticalPhaseRegion {
    // Pressure is less than both the sublimation and vaporization curve and is below the critical temperature
    Vapor,
    // Pressure is above the vaporization curve and the temperature is greater than the fusion curve and less than the critical temperature
    Liquid,
}

impl TryFrom<String> for SteamNonCriticalPhaseRegion {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Vapor" => Ok(SteamNonCriticalPhaseRegion::Vapor),
            "Liquid" => Ok(SteamNonCriticalPhaseRegion::Liquid),
            _ => Err(format!("Unknown Phase \"{}\"", value)),
        }
    }
}

impl From<SteamNonCriticalPhaseRegion> for String {
    fn from(val: SteamNonCriticalPhaseRegion) -> Self {
        match val {
            SteamNonCriticalPhaseRegion::Vapor => "Vapor".to_owned(),
            SteamNonCriticalPhaseRegion::Liquid => "Liquid".to_owned(),
        }
    }
}

impl Display for SteamNonCriticalPhaseRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SteamNonCriticalPhaseRegion::Vapor => "Vapor",
                SteamNonCriticalPhaseRegion::Liquid => "Liquid",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SatQuery {
    SatTQuery {
        // K
        temperature: Temperature,
        phase_region: SteamNonCriticalPhaseRegion,
    },
    SatPQuery {
        // Pa
        pressure: Pressure,
        phase_region: SteamNonCriticalPhaseRegion,
    },
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SteamQuery {
    Pt(PtPoint),
    Sat(SatQuery),
    EntropyP {
        entropy: EnergyPerMassTemperature,
        pressure: Pressure,
    },
    EnthalpyP {
        enthalpy: EnergyPerMass,
        pressure: Pressure,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SteamQueryErr {
    OutOfRange(OutOfRange),
    CompositePhaseRegionErr(CompositePhaseRegionErr),
    FailedToConverge(RootFinderErr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutOfRange {
    TemperatureLow,
    TemperatureHigh,
    PressureLow,
    PressureHigh,
    AboveCriticalTemperature,
    BelowCriticalTemperature,
    AboveCriticalPressure,
}
