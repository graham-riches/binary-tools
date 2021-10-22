extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HtmlElement)]
pub fn html_element_derive(input: TokenStream) -> TokenStream {
    // Parse the AST and manipulate it to generate code
    let ast = syn::parse(input).expect("Could not parse token stream to AST");
    impl_html_element_macro(&ast)
}

fn impl_html_element_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HtmlElement for #name {
            fn start_tag() -> String {
                format!("<{}>", stringify!(#name).to_lowercase())
            }

            fn end_tag() -> String {
                format!("</{}>", stringify!(#name).to_lowercase())
            }
        }
    };
    gen.into()
}