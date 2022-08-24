use gloo_events::EventListener;
use uuid::Uuid;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::{Event, HtmlElement, Node};
use yew::{
    classes, create_portal, function_component, html, use_effect, use_effect_with_deps,
    use_node_ref, use_ref, use_state, Callback, Children, Classes, Properties,
};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub class: Option<Classes>,
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
        children,
        class,
        on_close_requested,
    }: &ModalProps,
) -> Html {
    let modal_element_created = use_state(|| false);
    let modal_id_ref = use_ref(|| Uuid::new_v4().to_string());
    let modal_ref = use_node_ref();
    {
        let modal_id_ref = modal_id_ref.clone();
        use_effect_with_deps(
            move |_| {
                let doc = gloo_utils::document();
                let body = doc.body().expect("Failed to get html body");
                let element = doc
                    .create_element("div")
                    .expect("Failed to create modal element");
                element.set_id(modal_id_ref.as_str());
                body.append_child(&element)
                    .expect("Failed to append modal element");
                modal_element_created.set(true);
                move || {
                    get_modal_element((*modal_id_ref).clone())
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

    if let Some(modal_host) = get_modal_element((*modal_id_ref).clone()) {
        create_portal(
            html! {
            <div
                class={classes!("fixed", "top-0", "left-0",
                    "w-full", "h-full", "z-30", "grid", "place-items-center")}
                >
                <div class={classes!("w-full", "h-full", "z-40","grid", "place-items-center")}>
                    <div ref={modal_ref} class={class}>
                        {for children.iter()}
                    </div>
                </div>
            </div>
            },
            modal_host.into(),
        )
    } else {
        html! {
            <></>
        }
    }
}
