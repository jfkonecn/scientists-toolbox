pub mod app;
mod shared;
mod thermo;

#[cfg(test)]
mod tests {
    use form_maker::FormMaker;
    use form_maker_derive::FormMaker;

    #[derive(FormMaker)]
    struct Something {}

    #[test]
    fn should_create_correct_form() {
        let html = yew::html! {
            <div></div>
        };
        let html2 = yew::html! {
            <div></div>
        };
        println!("{:?}", html);
        assert_eq!(Something::make_form(), ());
        assert_eq!(html, html2);
        assert_eq!(1, 1);
    }
}
