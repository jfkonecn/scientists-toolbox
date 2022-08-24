use super::shared::search_button::*;
use yew::{function_component, html};

#[function_component(Splash)]
pub fn splash() -> Html {
    html! {
        <SearchButton button_type={SearchButtonType::Wide} />
    }
}
