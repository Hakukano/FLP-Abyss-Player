mod accessible_model;
mod differ;

use proc_macro::TokenStream;

#[proc_macro_derive(AccessibleModel, attributes(accessible_model))]
pub fn accessible_model(tokens: TokenStream) -> TokenStream {
    accessible_model::handle(tokens)
}

#[proc_macro_derive(Differ)]
pub fn differ(tokens: TokenStream) -> TokenStream {
    differ::handle(tokens)
}
