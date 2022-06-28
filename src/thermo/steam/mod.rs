use crate::numerical_methods::*;
use crate::thermo::*;

pub mod iapws97;
mod iapws97_constants;
mod water_constants;

#[derive(Copy, Clone, Debug)]
pub struct PtPoint {
    // Pa
    pub pressure: f64,
    // K
    pub temperature: f64,
}

#[derive(Debug, PartialEq)]
pub enum SteamNonCriticalPhaseRegion {
    // Pressure is less than both the sublimation and vaporization curve and is below the critical temperature
    Vapor,
    // Pressure is above the vaporization curve and the temperature is greater than the fusion curve and less than the critical temperature
    Liquid,
}

#[derive(Debug)]
pub enum SatQuery {
    SatTQuery {
        // K
        temperature: f64,
        phase_region: SteamNonCriticalPhaseRegion,
    },
    SatPQuery {
        // Pa
        pressure: f64,
        phase_region: SteamNonCriticalPhaseRegion,
    },
}

#[derive(Debug)]
pub enum SteamQuery {
    PtQuery(PtPoint),
    SatQuery(SatQuery),
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

#[derive(Debug, PartialEq)]
pub enum SteamQueryErr {
    OutOfRange(OutOfRange),
    CompositePhaseRegionErr(CompositePhaseRegionErr),
    FailedToConverge(RootFinderErr),
}

#[derive(Debug, PartialEq)]
pub enum OutOfRange {
    TemperatureLow,
    TemperatureHigh,
    PressureLow,
    PressureHigh,
    AboveCriticalTemperature,
    BelowCriticalTemperature,
    AboveCriticalPressure,
}
