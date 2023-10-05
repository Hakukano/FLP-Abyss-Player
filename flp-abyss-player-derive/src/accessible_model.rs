use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ext::IdentExt, parse_macro_input, Data, DataStruct, Fields, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(accessible_model))]
struct Options {
    singleton: Ident,
    rw_lock: bool,
}

pub fn handle(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token);
    let options = Options::from_derive_input(&input).expect("Wrong options");

    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let singleton = &options.singleton;

    let accessors = fields.iter().map(|field| {
        let field_name = field.ident.clone().expect("No field name found");
        let set_field_name = Ident::new(&format!("set_{}", field_name.unraw()), Span::call_site());
        let field_type = field.ty.clone();

        if options.rw_lock {
            quote! {
                pub fn #field_name() -> #field_type {
                    #singleton.read().#field_name.clone()
                }

                pub fn #set_field_name(value: #field_type) {
                    #singleton.write().#field_name = value;
                }
            }
        } else {
            quote! {
                pub fn #field_name() -> &#field_type {
                    &self.#field_name
                }
            }
        }
    });

    let output = quote! {
        impl #struct_name {
            #(#accessors)*

            pub fn all() -> #struct_name {
                (*#singleton.read()).clone()
            }

            pub fn set_all(value: #struct_name) {
                *#singleton.write() = value;
            }
        }
    };
    output.into()
}
