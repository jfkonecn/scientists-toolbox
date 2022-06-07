#[derive(Debug, PartialEq)]
pub enum PhaseRegion {
    SupercriticalFluid,
    // Temperature and pressure is above the critical point
    Gas,
    NonCritical(NonCriticalPhaseRegion),
    Composite(CompositePhaseRegion),
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
pub struct SolidLiquid {
    solid_frac: f64,
    liquid_frac: f64,
}

impl SolidLiquid {
    pub fn new(solid_frac: f64, liquid_frac: f64) -> Result<SolidLiquid, String> {
        if solid_frac + liquid_frac == 1.0 {
            Ok(SolidLiquid {
                solid_frac,
                liquid_frac,
            })
        } else {
            Err(String::from("Fractions must add up to 1"))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LiquidVapor {
    liquid_frac: f64,
    vapor_frac: f64,
}

impl LiquidVapor {
    pub fn new(liquid_frac: f64, vapor_frac: f64) -> Result<LiquidVapor, String> {
        if liquid_frac + vapor_frac == 1.0 {
            Ok(LiquidVapor {
                liquid_frac,
                vapor_frac,
            })
        } else {
            Err(String::from("Fractions must add up to 1"))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SolidVapor {
    solid_frac: f64,
    vapor_frac: f64,
}

impl SolidVapor {
    pub fn new(solid_frac: f64, vapor_frac: f64) -> Result<SolidVapor, String> {
        if solid_frac + vapor_frac == 1.0 {
            Ok(SolidVapor {
                solid_frac,
                vapor_frac,
            })
        } else {
            Err(String::from("Fractions must add up to 1"))
        }
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
}

#[derive(Debug, PartialEq)]
pub struct PtvEntry {
    // in K
    pub temperature: f64,
    // in Pa
    pub pressure: f64,
    pub phase_region: PhaseRegion,
    // J/kg
    pub internal_energy: f64,
    // J/kg
    pub enthalpy: f64,
    // J/(kg * K)
    pub entropy: f64,
    // isochoric heat capacity
    // Heat Capacity at constant volume (J/(kg*K))
    pub cv: f64,
    // isobaric heat capacity
    // Heat Capacity at constant pressure (J/(kg*K))
    pub cp: f64,
    // m/s
    pub speed_of_sound: f64,
    // in m3 / kg
    pub specific_volume: f64,
}
