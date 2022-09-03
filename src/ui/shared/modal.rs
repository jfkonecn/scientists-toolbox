use crate::ui::assets::svg::*;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Event, HtmlElement, MouseEvent, Node};
use yew::{
    classes, create_portal, function_component, html, use_effect, use_effect_with_deps,
    use_node_ref, use_state, Callback, Children, Properties,
};

use super::hooks::use_random_id;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_else(Callback::noop)]
    pub on_close_requested: Callback<Event>,
}

fn get_modal_element(modal_id: String) -> Option<web_sys::Element> {
    gloo_utils::document().get_element_by_id(modal_id.as_str())
}

#[function_component(Modal)]
pub fn modal(
    ModalProps {
        title,
        children,
        on_close_requested,
    }: &ModalProps,
) -> Html {
    let modal_element_created = use_state(|| false);
    let modal_id = use_random_id();
    let x_button_id = use_random_id();
    let modal_ref = use_node_ref();
    {
        let modal_id = modal_id.clone();
        use_effect_with_deps(
            move |_| {
                let doc = gloo_utils::document();
                let body = doc.body().expect("Failed to get html body");
                let element = doc
                    .create_element("div")
                    .expect("Failed to create modal element");
                element.set_id(modal_id.as_str());
                body.append_child(&element)
                    .expect("Failed to append modal element");
                modal_element_created.set(true);
                move || {
                    get_modal_element(modal_id)
                        .ok_or(JsValue::from("_"))
                        .and_then(|ele| body.remove_child(&ele))
                        .expect_throw("Failed to remove modal element");
                }
            },
            (),
        );
    }
    {
        let modal_ref = modal_ref.clone();
        let on_close_requested = on_close_requested.clone();
        use_effect(move || {
            let on_mouse_up = Callback::from(move |e: Event| {
                let element_opt = modal_ref.cast::<HtmlElement>();
                let target_opt = e.target().and_then(|x| x.dyn_into::<Node>().ok());

                if let Some(element) = element_opt {
                    if !element.contains(target_opt.as_ref()) {
                        on_close_requested.emit(e);
                    }
                }
            });
            let doc = gloo_utils::document();
            let listener = EventListener::new(&doc, "mouseup", move |e| {
                on_mouse_up.emit(e.clone());
            });

            move || drop(listener)
        });
    }

    let on_x_clicked = {
        let on_close_requested = on_close_requested.clone();
        Callback::from(move |e: MouseEvent| {
            on_close_requested.emit(e.into());
        })
    };
    let on_close_button_clicked = {
        let on_close_requested = on_close_requested.clone();
        Callback::from(move |e: MouseEvent| {
            on_close_requested.emit(e.into());
        })
    };

    if let Some(modal_host) = get_modal_element(modal_id) {
        create_portal(
            html! {
            <form
                class={classes!("fixed", "flex", "items-start", "justify-center", "flex-row", "content-start", "top-0", "left-0",
                    "w-screen", "h-screen", "z-30", "p-5" )}
                >
                <div ref={modal_ref} class={classes!(
                    "bg-white", "border-2", "border-gray-200",
                    "max-w-lg", "h-4/5", "h-full", "w-full",
                    "rounded-lg")}>
                        <form class={classes!("flex", "flex-col", "divide-y-2", "[&>*]:py-2", "[&>*]:px-4", "h-full", "w-full")}>
                            <div class={classes!("flex", "flex-row")}>
                                <h2 class={classes!("block")}>{title}</h2>
                                <div class={classes!("flex-grow")}></div>
                                <label class={classes!("block", "relative", "h-6", "w-6")} for={x_button_id.clone()}>
                                    <input
                                        type={"button"}
                                        class={classes!("absolute", "left-0", "top-0", "w-full", "h-full", "block")}
                                        id={x_button_id.clone()}
                                        aria-label={"Close Button"}
                                        onclick={on_x_clicked}
                                    />
                                    <XMark/>
                                </label>
                            </div>
                            <div class={classes!("overflow-auto", "flex-grow")}>
                                {for children.iter()}
                            </div>
                            <div class={classes!("w-full", "flex", "flex-row")}>
                                <div class={classes!("flex-grow")}></div>
                                <input
                                    class={classes!("h-12", "p-2", "border-2", "border-gray-200", "rounded-lg")}
                                    type={"button"}
                                    value={"Close"}
                                    aria-label={"Close Button"}
                                    onclick={on_close_button_clicked}
                                />
                            </div>
                        </form>
                </div>
            </form>
            },
            modal_host.into(),
        )
    } else {
        html! {
            <></>
        }
    }
}
