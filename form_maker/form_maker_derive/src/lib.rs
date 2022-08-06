use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(FormMaker)]
pub fn form_maker_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_form_maker_derive(&ast)
}

fn impl_form_maker_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl FormMaker for #name {
            fn make_form() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
