use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
       <p class={classes!("bg-red-100")}>{"Test!"}</p>
    }
}
