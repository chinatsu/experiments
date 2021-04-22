use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn protected_with_claims(_attr: TokenStream, code: TokenStream) -> TokenStream {
    let input = parse_macro_input!(code as DeriveInput);

    let expanded = quote! {

    };

    TokenStream::from(expanded)
}
