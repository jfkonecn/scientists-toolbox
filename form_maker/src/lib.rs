use form_maker_derive::FormMaker;

pub trait FormMaker {
    fn make_form() -> yew::virtual_dom::VNode;
    fn make_inputs() -> yew::virtual_dom::VNode;
}

#[derive(FormMaker)]
struct Something {}
