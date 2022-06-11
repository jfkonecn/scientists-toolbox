use crate::thermo::steam::iapws97::*;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let _ = get_steam_table_entry(SteamQuery::PtQuery(PtPoint {
        temperature: 750.0,
        pressure: 78.309563916917e6,
    }));
    html! {
       <p class={classes!("bg-red-100")}>{"Test!"}</p>
    }
}
