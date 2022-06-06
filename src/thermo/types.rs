pub enum PhaseRegion {
    SupercriticalFluid,
    // Temperature and pressure is above the critical point
    Gas,
    // Pressure is less than both the sublimation and vaporization curve and is below the critical temperature
    Vapor,
    // Pressure is above the vaporization curve and the temperature is greater than the fusion curve and less than the critical temperature
    Liquid,
    // Pressure is above the sublimation curve and temperature is less than the fusion curve
    Solid,
    SolidLiquid {
        solid_frac: f64,
        liquid_frac: f64,
    },
    LiquidVapor {
        liquid_frac: f64,
        vapor_frac: f64,
    },
    SolidVapor {
        solid_frac: f64,
        vapor_frac: f64,
    },
    SolidLiquidVapor {
        solid_frac: f64,
        liquid_frac: f64,
        vapor_frac: f64,
    },
}

pub struct PtvEntry {
    // in K
    temperature: f64,
    // in Pa
    pressure: f64,
    phase_region: PhaseRegion,
    // in m3 / kg
    specific_volume: f64,
    // J/kg
    internal_energy: f64,
    // J/kg
    enthalpy: f64,
    // J/(kg * K)
    entropy: f64,
    // isochoric heat capacity
    // Heat Capacity at constant volume (J/(kg*K))
    cv: f64,
    // isobaric heat capacity
    // Heat Capacity at constant pressure (J/(kg*K))
    cp: f64,
    // m/s
    speed_of_sound: f64,
    // kg/m3
    density: f64,
}
