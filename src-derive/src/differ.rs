use proc_macro::TokenStream;
use quote::quote;
use syn::{ext::IdentExt, parse_macro_input, Data, DataStruct, DeriveInput, Fields};

pub fn handle(token: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(token);

    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    let cmps = fields.iter().map(|field| {
        let field_name = field.ident.clone().expect("No field name found");
        let field_name_str = field_name.unraw().to_string();

        quote! {
            if let Some(diff) = self.#field_name.diff(&other.#field_name) {
                map.insert(#field_name_str.to_string(), diff);
            }
        }
    });

    let apply = fields.iter().map(|field| {
        let field_name = field.ident.clone().expect("No field name found");
        let field_name_str = field_name.unraw().to_string();

        quote! {
            if let Some(value) = diff.get(#field_name_str) {
                self.#field_name.apply_diff(value.clone());
            }
        }
    });

    let output = quote! {
        impl crate::library::differ::Differ for #struct_name {
            fn diff(&self, other: &Self) -> Option<serde_json::Value> {
                let mut map = serde_json::Map::new();
                #(#cmps)*
                if map.is_empty() {
                    None
                } else {
                    Some(serde_json::Value::Object(map))
                }
            }

            fn apply_diff(&mut self, diff: serde_json::Value) {
                #(#apply)*
            }
        }
    };
    output.into()
}
