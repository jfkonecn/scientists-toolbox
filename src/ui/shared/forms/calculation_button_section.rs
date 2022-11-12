use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CalculationButtonSectionProps {
    pub id: Option<String>,
    #[prop_or_else(Callback::noop)]
    pub on_click: Callback<Event>,
}

#[function_component(CalculationButtonSection)]
pub fn calculation_button_section(
    CalculationButtonSectionProps { id, on_click }: &CalculationButtonSectionProps,
) -> Html {
    let on_click = {
        let on_click = on_click.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_click.emit(e.into());
        })
    };
    html! {
            <fieldset>
                <input
                    id={id.clone()}
                    value={"Calculate"}
                    type="submit"
                    class={classes!(
                        "hover:cursor-pointer",
                        "border-2",
                        "rounded-md",
                        "border-gray-200",
                        "p-2",
                        "w-64"
                    )}
                    onclick={on_click}/>
            </fieldset>
    }
}
