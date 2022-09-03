use super::shared::search::*;
use yew::{classes, function_component, html};

#[function_component(Splash)]
pub fn splash() -> Html {
    html! {
        <div
            class={classes!("p-5")}
            >
            <Search/>
        </div>
    }
}
