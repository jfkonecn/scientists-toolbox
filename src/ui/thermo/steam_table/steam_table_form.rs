use super::super::super::shared::forms::unit_input::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SteamTableFormProps {}

#[function_component(SteamTableForm)]
pub fn steam_table_form(SteamTableFormProps {}: &SteamTableFormProps) -> Html {
    html! {
        <form class={classes!("w-full", "h-full", "grid", "place-items-center")}>
            <fieldset>
            </fieldset>
            <fieldset class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
                <UnitInput id={"pressure"} label={"Pressure"} unit={"Pa"} />
                <UnitInput id={"temperature"} label={"Temperature"} unit={"K"} />
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
                onclick={|e: MouseEvent| {
                    e.prevent_default();
                }}/>
        </form>
    }
}
