use super::OrificePlateQuery;
use crate::fluids::*;

pub fn query_orifice_plate(query: OrificePlateQuery) -> Result<OrificePlateFlow, String> {
    match query {
        OrificePlateQuery::FlowRate {
            pipe_area,
            orifice_area,
            pressure_drop,
            density,
            discharge_coefficient,
        } => {
            let pipe_area_value = pipe_area.into_si_unit().value;
            let orifice_area_value = orifice_area.into_si_unit().value;
            let pressure_drop_value = pressure_drop.into_si_unit().value;
            let density_value = density.into_si_unit().value;
            let flow_rate_value = discharge_coefficient
                * pipe_area_value
                * f64::sqrt(
                    (2f64 * pressure_drop_value)
                        / (density_value
                            * (f64::powi(pipe_area_value, 2) / f64::powi(orifice_area_value, 2)
                                - 1f64)),
                );
            Ok(OrificePlateFlow {
                pipe_area,
                orifice_area,
                pressure_drop,
                density,
                discharge_coefficient,
                flow_rate: VolumetricFlowRate::M3PerSec(M3PerSec::new(flow_rate_value)),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use std::f64::consts::PI;

    #[test]
    fn volumetric_query() {
        let query = OrificePlateQuery::FlowRate {
            pipe_area: Area::M2(M2::new((10f64 * 10f64 * PI) / 4f64)),
            orifice_area: Area::M2(M2::new((8f64 * 8f64 * PI) / 4f64)),
            density: Density::KgPerM3(KgPerM3::new(1000f64)),
            discharge_coefficient: 0.7,
            pressure_drop: Pressure::Pa(Pa::new(10f64)),
        };
        let expected = OrificePlateFlow {
            pipe_area: Area::M2(M2::new((10f64 * 10f64 * PI) / 4f64)),
            orifice_area: Area::M2(M2::new((8f64 * 8f64 * PI) / 4f64)),
            density: Density::KgPerM3(KgPerM3::new(1000f64)),
            pressure_drop: Pressure::Pa(Pa::new(10f64)),
            discharge_coefficient: 0.7,
            flow_rate: VolumetricFlowRate::M3PerSec(M3PerSec::new(6.4760429)),
        };
        let actual = query_orifice_plate(query).unwrap();
        assert_approx_eq!(expected.pipe_area, actual.pipe_area);
        assert_approx_eq!(expected.orifice_area, actual.orifice_area);
        assert_approx_eq!(expected.density, actual.density);
        assert_approx_eq!(expected.pressure_drop, actual.pressure_drop);
        assert_approx_eq!(expected.discharge_coefficient, actual.discharge_coefficient);
        assert_approx_eq!(expected.flow_rate, actual.flow_rate);
    }
}
