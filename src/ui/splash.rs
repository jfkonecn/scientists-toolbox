use super::shared::search::*;
use web_sys::{Event, FocusEvent};
use yew::{classes, function_component, html};

#[function_component(Splash)]
pub fn splash() -> Html {
    html! {
        <form
            class={classes!("p-5")}
            onsubmit={|e: FocusEvent| {
                e.prevent_default();
            }}
            >
            <Search/>
        </form>
    }
}
