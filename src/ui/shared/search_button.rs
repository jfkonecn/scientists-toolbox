use super::modal::*;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{function_component, use_state_eq, Callback, Properties};
use yew_router::prelude::*;

#[derive(PartialEq)]
pub enum SearchButtonType {
    Compact,
    Wide,
}

#[derive(Properties, PartialEq)]
pub struct SearchButtonProps {
    pub button_type: SearchButtonType,
}

#[function_component(SearchButton)]
pub fn search_button(SearchButtonProps { button_type }: &SearchButtonProps) -> Html {
    let show_modal = use_state_eq(|| false);
    let toggle_show_modal = {
        let show_modal = show_modal.clone();
        let show_modal_value = !*show_modal;
        Callback::from(move |_: MouseEvent| {
            show_modal.set(show_modal_value);
        })
    };
    let close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_: Event| {
            show_modal.set(false);
        })
    };

    html! {
        <>
            <button onclick={toggle_show_modal}>{"toggle modal"}</button>
            {
                if *show_modal {
                    html! {
                    <Modal
                        class={classes!("bg-red-700", "w-44", "h-52")} on_close_requested={close_modal}>
                    <h1>{"test2"}</h1>
                    </Modal>

                    }
                } else {
                    html! {
                        <></>
                    }
                }
            }
        </>
    }
}
