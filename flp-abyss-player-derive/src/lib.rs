mod differ;
mod static_record;

use proc_macro::TokenStream;

#[proc_macro_derive(StaticRecord, attributes(static_record))]
pub fn static_record(tokens: TokenStream) -> TokenStream {
    static_record::handle(tokens)
}

#[proc_macro_derive(Differ)]
pub fn differ(tokens: TokenStream) -> TokenStream {
    differ::handle(tokens)
}
