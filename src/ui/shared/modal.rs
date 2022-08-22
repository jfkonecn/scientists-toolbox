use uuid::Uuid;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use yew::{
    classes, create_portal, function_component, html, use_effect_with_deps, use_ref, use_state,
    Children, Classes, Properties,
};

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub class: Option<Classes>,
    #[prop_or_default]
    pub children: Children,
}

fn get_modal_element(modal_id: String) -> Option<web_sys::Element> {
    gloo_utils::document().get_element_by_id(modal_id.as_str())
}

#[function_component(Modal)]
pub fn modal(ModalProps { children, class }: &ModalProps) -> Html {
    let modal_element_created = use_state(|| false);
    let modal_id = use_ref(|| Uuid::new_v4().to_string());
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
                    get_modal_element((*modal_id).clone())
                        .ok_or(JsValue::from("_"))
                        .and_then(|ele| body.remove_child(&ele))
                        .expect_throw("Failed to remove modal element");
                }
            },
            (),
        );
    }

    if let Some(modal_host) = get_modal_element((*modal_id).clone()) {
        create_portal(
            html! {
            <div class={classes!("fixed", "top-0", "left-0", "w-full", "h-full", "z-30")}>
                <div class={class}>
             {for children.iter()}
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
