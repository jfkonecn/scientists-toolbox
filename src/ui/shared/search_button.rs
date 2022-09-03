use super::modal::*;
use super::search::*;
use crate::ui::assets::svg::*;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew::{function_component, use_state_eq, Callback};

#[function_component(SearchButton)]
pub fn search_button() -> Html {
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
            <button
                class={classes!("w-full", "h-full")}
                onclick={toggle_show_modal}>
                <MagnifyingGlass/>
            </button>
            {
                if *show_modal {
                    html! {
                    <Modal
                        title={"Search".to_owned()}
                        on_close_requested={close_modal}>
                        <Search/>
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
