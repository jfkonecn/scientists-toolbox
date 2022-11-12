use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CalculationSectionProps {
    pub id: Option<String>,
    pub children: Children,
}

#[function_component(CalculationSection)]
pub fn calculation_section(
    CalculationSectionProps { id, children }: &CalculationSectionProps,
) -> Html {
    html! {
        <fieldset id={id.clone()} class={classes!("grid", "grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3")}>
            {children.clone()}
        </fieldset>
    }
}
