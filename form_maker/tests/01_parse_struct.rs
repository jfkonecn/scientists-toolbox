
use form_maker::FormMaker;
use form_maker_derive::FormMaker;

#[derive(FormMaker)]
struct Something {}

#[test]
fn should_create_correct_form() {
    let expected = yew::html! {
        <div></div>
    };
    assert_eq!(expected, Something::make_form());
}
