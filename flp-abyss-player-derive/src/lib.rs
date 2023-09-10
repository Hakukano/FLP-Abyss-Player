mod accessible_model;

use proc_macro::TokenStream;

#[proc_macro_derive(AccessibleModel, attributes(accessible_model))]
pub fn accessible_model(tokens: TokenStream) -> TokenStream {
    accessible_model::handle(tokens)
}
