use std::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromTags)]
pub fn derive_from_tags(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let expanded = match data {
        syn::Data::Struct(s) => {
            let fields = match s.fields {
                syn::Fields::Named(named) => named,
                _ => panic!("expecting named structure"),
            };

            let fields_idt: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();

            let fields_str: Vec<_> = fields
                .named
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    format!("{}", quote! { #ident })
                })
                .collect();

            quote! {
                impl crate::FromTags for #ident {
                    fn from_tags(tags: &[(String, String)]) -> Result<Self, crate::MissingTags> {
                        #(let mut #fields_idt = None;)*

                        for (k, v) in tags.iter() {
                            match k.as_str() {
                                #(#fields_str => #fields_idt = Some(v.clone()),)*
                                _ => {},
                            }
                        }

                        let missing_tags: Vec<_> = [
                            #((#fields_str, &#fields_idt),)*
                        ]
                        .iter()
                        .filter(|(_, v)| v.is_none())
                        .map(|(k, _)| k.to_string())
                        .collect();

                        if missing_tags.is_empty() {
                            Ok(Self {
                                #(#fields_idt: #fields_idt.unwrap(),)*
                            })
                        } else {
                            Err(crate::MissingTags { missing_tags })
                        }
                    }
                }
            }
        }
        _ => panic!("expecting "),
    };

    // panic!("{}", expanded.to_string());

    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
