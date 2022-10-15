use std::fmt;

use crate::units::*;

pub mod steam;

#[derive(Debug, PartialEq)]
pub enum PhaseRegion {
    SupercriticalFluid,
    // Temperature and pressure is above the critical point
    Gas,
    NonCritical(NonCriticalPhaseRegion),
    Composite(CompositePhaseRegion),
}

impl fmt::Display for PhaseRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhaseRegion::SupercriticalFluid => write!(f, "Supercritical Fluid"),
            PhaseRegion::Gas => write!(f, "Gas"),
            PhaseRegion::NonCritical(NonCriticalPhaseRegion::Vapor) => write!(f, "Vapor"),
            PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid) => write!(f, "Liquid"),
            PhaseRegion::NonCritical(NonCriticalPhaseRegion::Solid) => write!(f, "Solid"),
            PhaseRegion::Composite(CompositePhaseRegion::SolidLiquidVapor(x)) => write!(
                f,
                "Solid {:.2}%, Liquid {:.2}%, Vapor {:.2}%",
                x.get_solid_frac(),
                x.get_liquid_frac(),
                x.get_vapor_frac(),
            ),
            PhaseRegion::Composite(CompositePhaseRegion::SolidLiquid(x)) => write!(
                f,
                "Solid {:.2}%, Liquid {:.2}%",
                x.get_solid_frac(),
                x.get_liquid_frac(),
            ),
            PhaseRegion::Composite(CompositePhaseRegion::SolidVapor(x)) => write!(
                f,
                "Solid {:.2}%, Vapor {:.2}%",
                x.get_solid_frac(),
                x.get_vapor_frac()
            ),
            PhaseRegion::Composite(CompositePhaseRegion::LiquidVapor(x)) => write!(
                f,
                "Liquid {:.2}%, Vapor {:.2}%",
                x.get_liquid_frac(),
                x.get_vapor_frac(),
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NonCriticalPhaseRegion {
    // Pressure is less than both the sublimation and vaporization curve and is below the critical temperature
    Vapor,
    // Pressure is above the vaporization curve and the temperature is greater than the fusion curve and less than the critical temperature
    Liquid,
    // Pressure is above the sublimation curve and temperature is less than the fusion curve
    Solid,
}

#[derive(Debug, PartialEq)]
pub enum CompositePhaseRegion {
    SolidLiquid(SolidLiquid),
    LiquidVapor(LiquidVapor),
    SolidVapor(SolidVapor),
    SolidLiquidVapor(SolidLiquidVapor),
}

#[derive(Debug, PartialEq)]
pub enum CompositePhaseRegionErr {
    FractionsDoNotAddUpToOne,
    FractionsMustBePositive,
}

#[derive(Debug, PartialEq)]
pub struct SolidLiquid {
    solid_frac: f64,
    liquid_frac: f64,
}

impl SolidLiquid {
    pub fn new(solid_frac: f64, liquid_frac: f64) -> Result<SolidLiquid, CompositePhaseRegionErr> {
        if solid_frac + liquid_frac == 1.0 {
            Ok(SolidLiquid {
                solid_frac,
                liquid_frac,
            })
        } else {
            Err(CompositePhaseRegionErr::FractionsDoNotAddUpToOne)
        }
    }

    pub fn get_solid_frac(&self) -> f64 {
        self.solid_frac
    }

    pub fn get_liquid_frac(&self) -> f64 {
        self.liquid_frac
    }
}

#[derive(Debug, PartialEq)]
pub struct LiquidVapor {
    liquid_frac: f64,
    vapor_frac: f64,
}

impl LiquidVapor {
    pub fn new(liquid_frac: f64, vapor_frac: f64) -> Result<LiquidVapor, CompositePhaseRegionErr> {
        if liquid_frac + vapor_frac == 1.0 {
            Ok(LiquidVapor {
                liquid_frac,
                vapor_frac,
            })
        } else {
            Err(CompositePhaseRegionErr::FractionsDoNotAddUpToOne)
        }
    }

    pub fn get_liquid_frac(&self) -> f64 {
        self.liquid_frac
    }

    pub fn get_vapor_frac(&self) -> f64 {
        self.vapor_frac
    }
}

#[derive(Debug, PartialEq)]
pub struct SolidVapor {
    solid_frac: f64,
    vapor_frac: f64,
}

impl SolidVapor {
    pub fn new(solid_frac: f64, vapor_frac: f64) -> Result<SolidVapor, CompositePhaseRegionErr> {
        if solid_frac + vapor_frac == 1.0 {
            Ok(SolidVapor {
                solid_frac,
                vapor_frac,
            })
        } else {
            Err(CompositePhaseRegionErr::FractionsDoNotAddUpToOne)
        }
    }

    pub fn get_solid_frac(&self) -> f64 {
        self.solid_frac
    }

    pub fn get_vapor_frac(&self) -> f64 {
        self.vapor_frac
    }
}

#[derive(Debug, PartialEq)]
pub struct SolidLiquidVapor {
    solid_frac: f64,
    liquid_frac: f64,
    vapor_frac: f64,
}

impl SolidLiquidVapor {
    pub fn new(
        solid_frac: f64,
        liquid_frac: f64,
        vapor_frac: f64,
    ) -> Result<SolidLiquidVapor, String> {
        if solid_frac + liquid_frac + vapor_frac == 1.0 {
            Ok(SolidLiquidVapor {
                solid_frac,
                liquid_frac,
                vapor_frac,
            })
        } else {
            Err(String::from("Fractions must add up to 1"))
        }
    }

    pub fn get_solid_frac(&self) -> f64 {
        self.solid_frac
    }

    pub fn get_liquid_frac(&self) -> f64 {
        self.liquid_frac
    }

    pub fn get_vapor_frac(&self) -> f64 {
        self.vapor_frac
    }
}

#[derive(Debug, PartialEq)]
pub struct PtvEntry {
    // in K
    pub temperature: Temperature,
    // in Pa
    pub pressure: Pressure,
    pub phase_region: PhaseRegion,
    // J/kg
    pub internal_energy: EnergyPerMass,
    // J/kg
    pub enthalpy: EnergyPerMass,
    // J/(kg * K)
    pub entropy: EnergyPerMassTemperature,
    /// Isochoric Heat Capacity
    /// Heat Capacity at constant volume (J/(kg*K))
    pub cv: EnergyPerMassTemperature,
    // isobaric heat capacity
    // Heat Capacity at constant pressure (J/(kg*K))
    pub cp: EnergyPerMassTemperature,
    // m/s
    pub speed_of_sound: Velocity,
    // in m3 / kg
    pub specific_volume: SpecificVolume,
}
