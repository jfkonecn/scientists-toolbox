// https://github.com/jfkonecn/thermo/blob/feature/issue-42/thermo/steam_properties.py
use crate::numerical_methods::root_finders::secant_method;
use crate::numerical_methods::root_finders::RootFinderErr;
use crate::thermo::steam::iapws97_constants::*;
use crate::thermo::steam::water_constants::*;
use crate::thermo::types::*;

enum Iapws97Region {
    Region1,
    Region2,
    Region3,
    Region4,
    Region5,
}

#[derive(Copy, Clone)]
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

struct SpecificRegionPoint {
    point: PtPoint,
    tau: f64,
    pi: f64,
    gamma: f64,
    gamma_pi: f64,
    gamma_pi_pi: f64,
    gamma_tau: f64,
    gamma_tau_tau: f64,
    gamma_pi_tau: f64,
}

fn get_sat_pressure(temperature: f64) -> Result<f64, OutOfRange> {
    match temperature {
        t if t > CRITICAL_TEMPERATURE => Result::Err(OutOfRange::AboveCriticalTemperature),
        t => {
            let sat_temp_ratio = t / 1.0;
            let theta = sat_temp_ratio + (region_4[8].n / (sat_temp_ratio - region_4[9].n));
            let a = f64::powi(theta, 2) + region_4[0].n * theta + region_4[1].n;
            let b = region_4[2].n * f64::powi(theta, 2) + region_4[3].n * theta + region_4[4].n;
            let c = region_4[5].n * f64::powi(theta, 2) + region_4[6].n * theta + region_4[7].n;
            let pressure = f64::powi(
                (2.0 * a) / (-b + f64::powf(f64::powi(b, 2) - 4.0 * a * c, 0.5)),
                4,
            ) * 1e6;
            Result::Ok(pressure)
        }
    }
}

fn get_sat_temperature(pressure: f64) -> Result<f64, OutOfRange> {
    match pressure {
        p if p > CRITICAL_PRESSURE => Err(OutOfRange::AboveCriticalPressure),
        p => {
            let beta = f64::powf(p / 1e6, 0.25);
            let e = f64::powi(beta, 2) + region_4[2].n * beta + region_4[5].n;
            let f = region_4[0].n * f64::powi(beta, 2) + region_4[3].n * beta + region_4[6].n;
            let g = region_4[1].n * f64::powi(beta, 2) + region_4[4].n * beta + region_4[7].n;
            let d = (2.0 * g) / (-f - f64::powf(f64::powi(f, 2) - 4.0 * e * g, 0.5));
            let temperature = (region_4[9].n + d
                - f64::powf(
                    f64::powi(region_4[9].n + d, 2) - 4.0 * (region_4[8].n + region_4[9].n * d),
                    0.5,
                ))
                / 2.0;
            Ok(temperature)
        }
    }
}

fn get_boundary_34_pressure(temperature: f64) -> Result<f64, OutOfRange> {
    match temperature {
        t if t < CRITICAL_TEMPERATURE => Err(OutOfRange::BelowCriticalTemperature),
        t => {
            let theta = t / 1.0;
            let pressure = (boundary_34[0].n
                + boundary_34[1].n * theta
                + boundary_34[2].n * f64::powi(theta, 2))
                * 1e6;
            Ok(pressure)
        }
    }
}

fn extract_pressure(query: &SteamQuery) -> Option<f64> {
    match query {
        SteamQuery::PtQuery(PtPoint {
            pressure: p,
            temperature: _,
        })
        | SteamQuery::SatQuery(SatQuery::SatPQuery {
            pressure: p,
            phase_region: _,
        })
        | SteamQuery::EntropyPQuery {
            entropy: _,
            pressure: p,
        }
        | SteamQuery::EnthalpyPQuery {
            enthalpy: _,
            pressure: p,
        } => Some(*p),
        _ => None,
    }
}

fn extract_temperature(query: &SteamQuery) -> Option<f64> {
    match query {
        SteamQuery::PtQuery(PtPoint {
            pressure: _,
            temperature: t,
        })
        | SteamQuery::SatQuery(SatQuery::SatTQuery {
            temperature: t,
            phase_region: _,
        }) => Some(*t),
        _ => None,
    }
}

fn check_if_out_of_range(query: &SteamQuery) -> Result<(), OutOfRange> {
    let opt_p = extract_pressure(&query);
    let opt_t = extract_temperature(&query);
    match (opt_p, opt_t) {
        (_, Some(t)) if t < 273.15 => Err(OutOfRange::TemperatureLow),
        (_, Some(t)) if t > 2000.0 + 273.15 => Err(OutOfRange::TemperatureHigh),
        (Some(p), Some(t)) if p > 50e6 && t > 800.0 + 273.15 => Err(OutOfRange::TemperatureHigh),
        (Some(p), _) if p < 0.0 => Err(OutOfRange::PressureLow),
        (Some(p), _) if p > 100e6 => Err(OutOfRange::PressureHigh),
        _ => Ok(()),
    }
}

fn get_region_from_pt_point(pt_point: &PtPoint) -> Result<Iapws97Region, OutOfRange> {
    let t = pt_point.temperature;
    let p = pt_point.pressure;
    let opt_sat_p_result = get_sat_pressure(t);
    let opt_boundary_result = get_boundary_34_pressure(t);
    match (opt_sat_p_result, opt_boundary_result) {
        (_, _) if t > 273.15 + 800.0 => Ok(Iapws97Region::Region5),
        (_, _) if t > 273.15 + 600.0 => Ok(Iapws97Region::Region2),
        (Ok(sat_p), _) if p == sat_p => Ok(Iapws97Region::Region4),
        (Ok(sat_p), _) if p > sat_p => Ok(Iapws97Region::Region2),
        (Ok(_), _) => Ok(Iapws97Region::Region1),
        (_, Ok(boundary)) if p > boundary => Ok(Iapws97Region::Region2),
        (_, Ok(_)) => Ok(Iapws97Region::Region3),
        (Err(err), _) => Err(err),
    }
}

fn get_region_from_sat_query(sat_query: &SatQuery) -> Result<(PtPoint, Iapws97Region), OutOfRange> {
    let region = match sat_query {
        SatQuery::SatTQuery {
            temperature: _,
            phase_region: r,
        }
        | SatQuery::SatPQuery {
            pressure: _,
            phase_region: r,
        } => match *r {
            SteamNonCriticalPhaseRegion::Liquid => Iapws97Region::Region1,
            SteamNonCriticalPhaseRegion::Vapor => Iapws97Region::Region2,
        },
    };
    let pt_result = match sat_query {
        SatQuery::SatTQuery {
            temperature: t,
            phase_region: _,
        } => get_sat_pressure(*t).map(|p| PtPoint {
            pressure: p,
            temperature: *t,
        }),
        SatQuery::SatPQuery {
            pressure: p,
            phase_region: _,
        } => get_sat_temperature(*p).map(|t| PtPoint {
            pressure: *p,
            temperature: t,
        }),
    };
    pt_result.map(|pt| (pt, region))
}

fn create_entry_from_region_point(
    specific_region_point: SpecificRegionPoint,
    phase_region: PhaseRegion,
) -> PtvEntry {
    let temperature = specific_region_point.point.temperature;
    let pressure = specific_region_point.point.pressure;
    let pi = specific_region_point.pi;
    let tau = specific_region_point.tau;
    let gamma = specific_region_point.gamma;
    let gamma_pi = specific_region_point.gamma_pi;
    let gamma_pi_pi = specific_region_point.gamma_pi_pi;
    let gamma_tau = specific_region_point.gamma_tau;
    let gamma_tau_tau = specific_region_point.gamma_tau_tau;
    let gamma_pi_tau = specific_region_point.gamma_pi_tau;

    PtvEntry {
        temperature,
        pressure,
        phase_region,
        internal_energy: GAS_CONSTANT * temperature * (tau * gamma_tau - pi * gamma_pi),
        enthalpy: GAS_CONSTANT * temperature * tau * gamma_tau,
        entropy: GAS_CONSTANT * (tau * gamma_tau - gamma),
        cv: GAS_CONSTANT
            * (-f64::powi(-tau, 2) * gamma_tau_tau
                + f64::powi(gamma_pi - tau * gamma_pi_tau, 2) / gamma_pi_pi),
        cp: GAS_CONSTANT * -f64::powi(-tau, 2) * gamma_tau_tau,
        speed_of_sound: f64::sqrt(
            GAS_CONSTANT
                * temperature
                * ((f64::powi(gamma_pi, 2))
                    / ((f64::powi(gamma_pi - tau * gamma_pi_tau, 2)
                        / (f64::powi(tau, 2) * gamma_tau_tau))
                        - gamma_pi_pi)),
        ),
        specific_volume: pi * (gamma_pi * GAS_CONSTANT * temperature) / pressure,
    }
}

fn gibbs_method(point: &PtPoint) -> PtvEntry {
    let pi = point.pressure / 16.53e6;
    let tau = 1386.0 / point.temperature;
    let mut gamma = 0f64;
    let mut gamma_pi = 0f64;
    let mut gamma_pi_pi = 0f64;
    let mut gamma_tau = 0f64;
    let mut gamma_tau_tau = 0f64;
    let mut gamma_pi_tau = 0f64;
    let phase_info = PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid);
    for region_point in region_1_and_4.iter() {
        let n = region_point.n;
        let i = region_point.i;
        let j = region_point.j;
        gamma += n * f64::powf(7.1 - pi, i) * f64::powf(tau - 1.222, j);
        gamma_pi += -n * i * f64::powf(7.1 - pi, i - 1f64) * f64::powf(tau - 1.222, j);
        gamma_pi_pi +=
            n * i * (i - 1f64) * f64::powf(7.1 - pi, i - 2f64) * f64::powf(tau - 1.222, j);
        gamma_tau += n * j * f64::powf(7.1 - pi, i) * f64::powf(tau - 1.222, j - 1f64);
        gamma_tau_tau +=
            n * j * (j - 1f64) * f64::powf(7.1 - pi, i) * f64::powf(tau - 1.222, j - 2f64);
        gamma_pi_tau +=
            -n * i * j * f64::powf(7.1 - pi, i - 1f64) * f64::powf(tau - 1.222, j - 1f64);
    }

    let specific_region_point = SpecificRegionPoint {
        point: *point,
        tau,
        pi,
        gamma,
        gamma_pi,
        gamma_pi_pi,
        gamma_tau,
        gamma_tau_tau,
        gamma_pi_tau,
    };

    create_entry_from_region_point(specific_region_point, phase_info)
}

fn vapor_method(
    tau: f64,
    tau_shift: f64,
    point: &PtPoint,
    ideal_points: &[JnRegionPoint],
    residual_points: &[IjnRegionPoint],
) -> PtvEntry {
    let pi = point.pressure / 1.0e6;
    let mut gamma = f64::log(pi, 10f64);
    let mut gamma_pi = 1.0 / pi;
    let mut gamma_pi_pi = -1.0 / f64::powi(pi, 2);
    let mut gamma_tau = 0f64;
    let mut gamma_tau_tau = 0f64;
    let mut gamma_pi_tau = 0f64;
    let phase_info = match (
        point.temperature > CRITICAL_TEMPERATURE,
        point.pressure > CRITICAL_PRESSURE,
    ) {
        (true, true) => PhaseRegion::SupercriticalFluid,
        (true, false) => PhaseRegion::Gas,
        _ => PhaseRegion::NonCritical(NonCriticalPhaseRegion::Vapor),
    };

    for region_point in ideal_points.iter() {
        let n = region_point.n;
        let j = region_point.j;
        gamma += n * f64::powf(tau, j);
        gamma_tau += n * j * f64::powf(tau, j - 1f64);
        gamma_tau_tau += n * j * (j - 1f64) * f64::powf(tau, j - 2f64);
    }
    for region_point in residual_points.iter() {
        let n = region_point.n;
        let i = region_point.i;
        let j = region_point.j;
        gamma += n * f64::powf(pi, i) * f64::powf(tau - tau_shift, j);
        gamma_pi += n * i * f64::powf(pi, i - 1f64) * f64::powf(tau - tau_shift, j);
        gamma_pi_pi += n * i * (i - 1f64) * f64::powf(pi, i - 2f64) * f64::powf(tau - tau_shift, j);
        gamma_tau += n * f64::powf(pi, i) * j * f64::powf(tau - tau_shift, j - 1f64);
        gamma_tau_tau +=
            n * f64::powf(pi, i) * j * (j - 1f64) * f64::powf(tau - tau_shift, j - 2f64);
        gamma_pi_tau += n * i * f64::powf(pi, i - 1f64) * j * f64::powf(tau - tau_shift, j - 1f64);
    }

    let specific_region_point = SpecificRegionPoint {
        point: *point,
        tau,
        pi,
        gamma,
        gamma_pi,
        gamma_pi_pi,
        gamma_tau,
        gamma_tau_tau,
        gamma_pi_tau,
    };

    create_entry_from_region_point(specific_region_point, phase_info)
}

fn region3_method(point: &PtPoint) -> Result<PtvEntry, SteamQueryErr> {
    unimplemented!()
}

fn get_entry_from_pt_point(
    point: &PtPoint,
    region: Iapws97Region,
) -> Result<PtvEntry, SteamQueryErr> {
    match region {
        Iapws97Region::Region1 | Iapws97Region::Region4 => Ok(gibbs_method(&point)),
        Iapws97Region::Region2 => Ok(vapor_method(
            540f64 / point.temperature,
            0.5,
            &point,
            region_2_ideal,
            region_2_residual,
        )),
        Iapws97Region::Region3 => region3_method(&point),
        Iapws97Region::Region5 => Ok(vapor_method(
            100f64 / point.temperature,
            0f64,
            &point,
            region_5_ideal,
            region_5_residual,
        )),
    }
}

fn interpolate_entry(
    liquid_entry: &PtvEntry,
    vapor_entry: &PtvEntry,
    liq_frac: f64,
) -> Result<PtvEntry, SteamQueryErr> {
    let vap_frac = 1.0 - liq_frac;
    let interpolate_entry_property =
        |f: fn(e: &PtvEntry) -> f64| (f(liquid_entry) * liq_frac) + (f(vapor_entry) * vap_frac);
    let phase_info_result = LiquidVapor::new(liq_frac, vap_frac)
        .map(|x| PhaseRegion::Composite(CompositePhaseRegion::LiquidVapor(x)))
        .map_err(|err| SteamQueryErr::CompositePhaseRegionErr(err));
    let temperature = interpolate_entry_property(|x| x.temperature);
    let pressure = interpolate_entry_property(|x| x.pressure);
    let internal_energy = interpolate_entry_property(|x| x.internal_energy);
    let enthalpy = interpolate_entry_property(|x| x.enthalpy);
    let entropy = interpolate_entry_property(|x| x.entropy);
    let cv = interpolate_entry_property(|x| x.cv);
    let cp = interpolate_entry_property(|x| x.cp);
    let speed_of_sound = interpolate_entry_property(|x| x.speed_of_sound);
    let specific_volume = interpolate_entry_property(|x| x.specific_volume);
    phase_info_result.map(|phase_region| PtvEntry {
        temperature,
        pressure,
        phase_region,
        internal_energy,
        enthalpy,
        entropy,
        cv,
        cp,
        speed_of_sound,
        specific_volume,
    })
}

fn iterate_pt_entry_solution(
    pressure: f64,
    target_value: f64,
    get_prop_value: fn(entry: &PtvEntry) -> f64,
) -> Result<PtvEntry, SteamQueryErr> {
    let liquid_entry_result = get_steam_table_entry(SteamQuery::SatQuery(SatQuery::SatPQuery {
        pressure,
        phase_region: SteamNonCriticalPhaseRegion::Liquid,
    }));
    let vapor_entry_result = get_steam_table_entry(SteamQuery::SatQuery(SatQuery::SatPQuery {
        pressure,
        phase_region: SteamNonCriticalPhaseRegion::Vapor,
    }));

    match (liquid_entry_result, vapor_entry_result) {
        (Ok(liquid_entry), Ok(vapor_entry))
            if get_prop_value(&liquid_entry) <= target_value
                && get_prop_value(&vapor_entry) >= target_value =>
        {
            let liq_frac = (get_prop_value(&vapor_entry) - target_value)
                / (get_prop_value(&vapor_entry) - get_prop_value(&liquid_entry));
            interpolate_entry(&liquid_entry, &vapor_entry, liq_frac)
        }
        _ => {
            let f = |temperature| {
                let query_result = get_steam_table_entry(SteamQuery::PtQuery(PtPoint {
                    pressure,
                    temperature,
                }));
                if let Ok(entry) = query_result {
                    get_prop_value(&entry) - target_value
                } else {
                    f64::NAN
                }
            };
            secant_method(f, 300f64, 310f64, 1e-15)
                .map_err(|err| SteamQueryErr::FailedToConverge(err))
                .and_then(|temperature| {
                    get_steam_table_entry(SteamQuery::PtQuery(PtPoint {
                        pressure,
                        temperature,
                    }))
                })
        }
    }
}

pub fn get_steam_table_entry(query: SteamQuery) -> Result<PtvEntry, SteamQueryErr> {
    let f = |err| SteamQueryErr::OutOfRange(err);

    check_if_out_of_range(&query)
        .map_err(f)
        .and_then(|_| match query {
            SteamQuery::PtQuery(point) => get_region_from_pt_point(&point)
                .map_err(f)
                .and_then(|r| get_entry_from_pt_point(&point, r)),
            SteamQuery::SatQuery(sat_query) => get_region_from_sat_query(&sat_query)
                .map_err(f)
                .and_then(|(p, r)| get_entry_from_pt_point(&p, r)),
            SteamQuery::EntropyPQuery {
                pressure: p,
                entropy: e,
            } => iterate_pt_entry_solution(p, e, |point| point.entropy),
            SteamQuery::EnthalpyPQuery {
                pressure: p,
                enthalpy: e,
            } => iterate_pt_entry_solution(p, e, |point| point.enthalpy),
        })
}

#[cfg(test)]
mod tests {

    use super::*;
    use assert_approx_eq::assert_approx_eq;

    macro_rules! get_steam_table_valid_entry_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected_result): (SteamQuery, Result<PtvEntry, SteamQueryErr>) = $value;
                let actual_result = get_steam_table_entry(input);
                match (expected_result, actual_result) {
                    (Ok(expected), Ok(actual)) => {
                        println!("{:?}", expected);
                        println!("{:?}", actual);
                        assert_approx_eq!(expected.pressure, actual.pressure);
                        assert_approx_eq!(expected.temperature, actual.temperature);
                        assert_eq!(expected.phase_region, actual.phase_region);
                        assert_approx_eq!(expected.internal_energy, actual.internal_energy);
                        assert_approx_eq!(expected.enthalpy, actual.enthalpy);
                        assert_approx_eq!(expected.entropy, actual.entropy);
                        assert_approx_eq!(expected.cv, actual.cv);
                        assert_approx_eq!(expected.cp, actual.cp);
                        assert_approx_eq!(expected.speed_of_sound, actual.speed_of_sound);
                        assert_approx_eq!(expected.specific_volume, actual.specific_volume);
                    },
                    (x, y) => assert_eq!(x, y),
                };
            }
        )*
        }
    }
    get_steam_table_valid_entry_tests! {
        steam_table_01: (
            SteamQuery::PtQuery(PtPoint {
                temperature: 750.0,
                pressure: 78.309563916917e6,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_02: (
            SteamQuery::PtQuery(PtPoint {
                temperature: 473.15,
                pressure: 40e6,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_03: (
            SteamQuery::PtQuery(PtPoint {
                temperature: 2000.0,
                pressure: 30e6,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_04: (
            SteamQuery::PtQuery(PtPoint {
                temperature: 823.15,
                pressure: 14e6,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_05: (
            SteamQuery::SatQuery(SatQuery::SatPQuery {
                pressure: 0.2e6,
                phase_region: SteamNonCriticalPhaseRegion::Liquid,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_06: (
            SteamQuery::SatQuery(SatQuery::SatPQuery {
                pressure: 0.2e6,
                phase_region: SteamNonCriticalPhaseRegion::Vapor,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_07: (
            SteamQuery::SatQuery(SatQuery::SatTQuery {
                temperature: 393.361545936488,
                phase_region: SteamNonCriticalPhaseRegion::Liquid,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_08: (
            SteamQuery::SatQuery(SatQuery::SatTQuery {
                temperature: 393.361545936488,
                phase_region: SteamNonCriticalPhaseRegion::Vapor,
            }),
            Ok(PtvEntry {
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
            })
        ),
        steam_table_09: (
            SteamQuery::EntropyPQuery {
                entropy: 4.469719056217e3,
                pressure: 78.309563916917e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_10: (
            SteamQuery::EntropyPQuery {
                entropy: 2.275752861241e3,
                pressure: 40e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_11: (
            SteamQuery::EntropyPQuery {
                entropy: 8.536405231138e3,
                pressure: 30e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_12: (
            SteamQuery::EntropyPQuery {
                entropy: 6.564768889364e3,
                pressure: 14e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_13: (
            SteamQuery::EntropyPQuery {
                entropy: 6.6858e3,
                pressure: 10e3,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_14: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 2258.688445460262e3,
                pressure: 78.309563916917e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_15: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 870.124259682489e3,
                pressure: 40e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_16: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 6571.226038618478e3,
                pressure: 30e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_17: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 3460.987255128561e3,
                pressure: 14e6,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_18: (
            SteamQuery::EnthalpyPQuery {
                enthalpy: 2117222.94886314,
                pressure: 10e3,
            },
            Ok(PtvEntry {
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
            })
        ),
        steam_table_19: (
            SteamQuery::PtQuery(PtPoint {
                temperature:273.0,
                pressure: 40e6,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::TemperatureLow))
        ),
        steam_table_20: (
            SteamQuery::PtQuery(PtPoint {
                temperature:273.0,
                pressure: 60e6,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::TemperatureLow))
        ),
        steam_table_21: (
            SteamQuery::PtQuery(PtPoint {
                temperature:2001.0 + 273.15,
                pressure: 40e6,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::TemperatureHigh))
        ),
        steam_table_22: (
            SteamQuery::PtQuery(PtPoint {
                temperature:801.0 + 273.0,
                pressure: 60e6,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::TemperatureHigh))
        ),
        steam_table_23: (
            SteamQuery::PtQuery(PtPoint {
                temperature:799.0 + 273.15,
                pressure: -1.0,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::PressureLow))
        ),
        steam_table_24: (
            SteamQuery::PtQuery(PtPoint {
                temperature:801.0 + 273.15,
                pressure: -1.0,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::PressureLow))
        ),
        steam_table_25: (
            SteamQuery::PtQuery(PtPoint {
                temperature:801.0 + 273.15,
                pressure: 51e6,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::TemperatureHigh))
        ),
        steam_table_26: (
            SteamQuery::PtQuery(PtPoint {
                temperature:799.0 + 273.15,
                pressure: 101e6,
            }),
            Err(SteamQueryErr::OutOfRange(OutOfRange::PressureHigh))
        ),
    }
}
