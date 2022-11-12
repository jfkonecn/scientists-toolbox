use crate::ui::app::MainRoute;
use crate::ui::assets::svg::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(SearchButton)]
pub fn search_button() -> Html {
    html! {
    <Link<MainRoute>
        classes={classes!("w-full", "h-full")}
        to={MainRoute::Home}>
        <MagnifyingGlass/>
    </Link<MainRoute>>
    }
}
