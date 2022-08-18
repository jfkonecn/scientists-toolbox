use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(FormMaker)]
pub fn form_maker_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_form_maker_derive(&ast)
}

#[proc_macro_derive(FormMaker2)]
pub fn form_maker_derive2(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_form_maker_derive(&ast)
}

fn impl_form_maker_derive(ast: &syn::DeriveInput) -> TokenStream {
    dbg!(ast);
    let name = &ast.ident;
    let _ = &ast.data;
    let gen = quote! {
        impl FormMaker for #name {
            fn make_form() -> yew::virtual_dom::VNode {
                yew::prelude::html! {
                    <div>
                    </div>
                }
            }
            fn make_inputs() -> yew::virtual_dom::VNode {
                yew::prelude::html! {
                    <div>
                    </div>
                }
            }
        }
    };
    gen.into()
}

fn something() {}
