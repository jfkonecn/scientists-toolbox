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
    pipe_area: Area,
    orifice_area: Area,
    density: Density,
    pressure_drop: Pressure,
    discharge_coefficient: f64,
    flow_rate: VolumetricFlowRate,
}
