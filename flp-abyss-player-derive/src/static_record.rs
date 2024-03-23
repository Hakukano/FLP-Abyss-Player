use convert_case::{Case, Casing};
use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ext::IdentExt, parse_macro_input, Ident, LitStr, Type};

#[derive(FromField)]
struct Field {
    ident: Option<Ident>,
    ty: Type,

    #[darling(default)]
    findable: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(static_record), forward_attrs(allow, doc, cfg))]
struct Options {
    ident: Ident,
    data: Data<Ignored, Field>,

    singleton: Ident,
    #[darling(default)]
    belongs_to: Vec<LitStr>,
    #[darling(default)]
    has_many: Vec<LitStr>,
}

pub fn handle(token: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token);
    let options = Options::from_derive_input(&input).expect("Wrong options");

    let fields = options
        .data
        .take_struct()
        .expect("Only struct is supported");

    let struct_name = options.ident;

    let singleton = options.singleton;

    let find_by = fields.iter().filter_map(|field| {
        let field_name = field.ident.clone().expect("No field name found");
        let find_by_field_name = Ident::new(
            &format!("find_by_{}", field_name.unraw()),
            Span::call_site(),
        );
        let field_type = field.ty.clone();

        if field.findable {
            Some(quote! {
                pub fn #find_by_field_name(value: &#field_type) -> Vec<#struct_name> {
                    #struct_name::all()
                        .iter()
                        .filter(|a| &a.#field_name == value)
                }
            })
        } else {
            None
        }
    });

    let belongs_to = options.belongs_to.iter().map(|a| {
        let value = a.value();
        let function_name = Ident::new(value.to_case(Case::Snake).as_str(), Span::call_site());
        let type_name = Ident::new(value.to_case(Case::Pascal).as_str(), Span::call_site());
        let id_name = Ident::new(
            format!("{}_id", value.to_case(Case::Snake)).as_str(),
            Span::call_site(),
        );
        quote! {
            pub fn #function_name(&self) -> Option<#type_name> {
                #type_name::find(&self.#id_name)
            }
        }
    });

    let has_many = options.has_many.iter().map(|a| {
        let value = a.value();
        let function_name = Ident::new(
            format!("{}_list", value.to_case(Case::Snake)).as_str(),
            Span::call_site(),
        );
        let type_name = Ident::new(value.to_case(Case::Pascal).as_str(), Span::call_site());
        let find_by_id_name = Ident::new(
            format!(
                "find_by_{}_id",
                struct_name.to_string().to_case(Case::Snake)
            )
            .as_str(),
            Span::call_site(),
        );
        quote! {
            pub fn #function_name(&self) -> Option<#type_name> {
                #type_name::#find_by_id_name(&self.id)
            }
        }
    });

    let output = quote! {
        impl #struct_name {
            #(#find_by)*
            #(#belongs_to)*
            #(#has_many)*

            pub fn all() -> HashMap<String, #struct_name> {
                (*#singleton.read()).clone()
            }

            pub fn find(id: &str) -> Option<#struct_name> {
                #singleton.read().get(id).cloned()
            }

            pub fn save(self) {
                #singleton.write().insert(self.id.clone(), self);
            }
        }
    };
    output.into()
}
