use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CalculationFormProps {
    pub id: Option<String>,
    pub children: Children,
}

#[function_component(CalculationForm)]
pub fn calculation_form(CalculationFormProps { id, children }: &CalculationFormProps) -> Html {
    html! {
        <form id={id.clone()} class={classes!(
                "w-full",
                "h-full",
                "grid",
                "place-items-center",
                "[&>*]:w-full",
                "[&>*]:p-8",
                "[&>*]:grid",
                "[&>*]:place-items-center",
            )}>
            {children.clone()}
        </form>
    }
}
