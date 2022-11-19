pub mod orifice_plate;
use crate::units::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum OrificePlateQuery {
    FlowRate {
        pipe_area: Area,
        orifice_area: Area,
        pressure_drop: Pressure,
        density: Density,
        discharge_coefficient: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct OrificePlateFlow {
    pub pipe_area: Area,
    pub orifice_area: Area,
    pub density: Density,
    pub pressure_drop: Pressure,
    pub discharge_coefficient: f64,
    pub flow_rate: VolumetricFlowRate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrificePlateQueryErr {
    DischargeCoefficientLow,
    DischargeCoefficientHigh,
}
