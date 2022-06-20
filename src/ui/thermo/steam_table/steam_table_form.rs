use super::super::super::shared::forms::str_output::*;
use super::super::super::shared::forms::unit_input::*;
use super::super::super::shared::forms::unit_output::*;
use crate::thermo::steam::iapws97::*;
use crate::thermo::types::*;
use yew::prelude::*;

impl PhaseRegion {
    fn to_display_str(&self) -> String {
        match self {
            PhaseRegion::SupercriticalFluid => String::from("Supercritical Fluid"),
            PhaseRegion::Gas => String::from("Gas"),
            PhaseRegion::NonCritical(NonCriticalPhaseRegion::Vapor) => String::from("Vapor"),
            PhaseRegion::NonCritical(NonCriticalPhaseRegion::Liquid) => String::from("Liquid"),
            PhaseRegion::NonCritical(NonCriticalPhaseRegion::Solid) => String::from("Solid"),
            PhaseRegion::Composite(CompositePhaseRegion::SolidLiquidVapor(x)) => format!(
                "Solid {}%, Liquid {}%, Vapor {}%",
                x.get_solid_frac(),
                x.get_liquid_frac(),
                x.get_vapor_frac(),
            ),
            PhaseRegion::Composite(CompositePhaseRegion::SolidLiquid(x)) => format!(
                "Solid {}%, Liquid {}%",
                x.get_solid_frac(),
                x.get_liquid_frac(),
            ),
            PhaseRegion::Composite(CompositePhaseRegion::SolidVapor(x)) => format!(
                "Solid {}%, Vapor {}%",
                x.get_solid_frac(),
                x.get_vapor_frac()
            ),
            PhaseRegion::Composite(CompositePhaseRegion::LiquidVapor(x)) => format!(
                "Liquid {}%, Vapor {}%",
                x.get_liquid_frac(),
                x.get_vapor_frac(),
            ),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct SteamTableFormProps {}

#[function_component(SteamTableForm)]
pub fn steam_table_form(SteamTableFormProps {}: &SteamTableFormProps) -> Html {
    let entry_opt = use_state(|| -> Option<Result<PtvEntry, SteamQueryErr>> {
        Some(Ok(PtvEntry {
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
        }))
    });

    let on_pressure_change = Callback::from(move |val| {
        log::info!("pressure is {:?} Pa", val);
    });

    let on_temperature_change = Callback::from(move |val| {
        log::info!("temperature is  {:?} K", val);
    });

    let output: Html = if let Some(Ok(entry)) = &*entry_opt {
        html! {
            <fieldset class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
                <UnitOutput
                    id={"pressure_output"}
                    label={"Pressure"}
                    unit={"Pa"}
                    value={entry.pressure}
                />
                <UnitOutput
                    id={"temperature_output"}
                    label={"Temperature"}
                    unit={"K"}
                    value={entry.temperature}
                />
                <StrOutput
                    id={"temperature_output"}
                    label={"Phase"}
                    value={entry.phase_region.to_display_str()}
                />
                <UnitOutput
                    id={"internal_energy_output"}
                    label={"Internal Energy"}
                    unit={"J/kg"}
                    value={entry.internal_energy}
                />
                <UnitOutput
                    id={"enthalpy_output"}
                    label={"Enthalpy"}
                    unit={"J/kg"}
                    value={entry.enthalpy}
                />
                <UnitOutput
                    id={"entropy_output"}
                    label={"Entropy"}
                    unit={"J/(kg * K)"}
                    value={entry.entropy}
                />
                <UnitOutput
                    id={"cv_output"}
                    label={"Isochoric Heat Capacity"}
                    unit={"J/(kg * K)"}
                    value={entry.cv}
                />
                <UnitOutput
                    id={"cp_output"}
                    label={"Isobaric Heat Capacity"}
                    unit={"J/(kg * K)"}
                    value={entry.cp}
                />
                <UnitOutput
                    id={"speed_of_sound_output"}
                    label={"Speed of Sound"}
                    unit={"m/s"}
                    value={entry.speed_of_sound}
                />
                <UnitOutput
                    id={"specific_volume_output"}
                    label={"Specific Volume"}
                    unit={"m^3/kg"}
                    value={entry.specific_volume}
                />
            </fieldset>
        }
    } else {
        html! {}
    };

    html! {
        <form class={classes!("w-full", "h-full", "grid", "place-items-center")}>
            <fieldset>
            </fieldset>
            <fieldset class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
                <UnitInput id={"pressure"} label={"Pressure"} unit={"Pa"} onchange={on_pressure_change} />
                <UnitInput id={"temperature"} label={"Temperature"} unit={"K"} onchange={on_temperature_change}/>
            </fieldset>
            <input
                value={"Calculate"}
                type="submit"
                class={classes!(
                    "hover:cursor-pointer",
                    "border-2",
                    "rounded-md",
                    "border-gray-200",
                    "p-2",
                    "w-64"
                )}
                onclick={move |e: MouseEvent| {
                    e.prevent_default();
                    let result = get_steam_table_entry(SteamQuery::PtQuery(PtPoint {
                        pressure: 10e6,
                        temperature: 300f64,
                    }));
                   entry_opt.set(Some(result));
                }}/>
                {output}
        </form>
    }
}
