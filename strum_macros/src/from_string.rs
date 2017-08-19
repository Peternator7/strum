
use quote;
use syn;

use helpers::{unique_attr, extract_attrs, is_disabled};

pub fn from_string_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("FromString only works on Enums"),
    };

    let mut has_default = false;
    let mut default =
        quote! { _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound) };
    let mut arms = Vec::new();
    for variant in variants {
        use syn::VariantData::*;
        let ident = &variant.ident;

        // Look at all the serialize attributes.
        let mut attrs = extract_attrs(&variant.attrs, "strum", "serialize");
        attrs.extend(extract_attrs(&variant.attrs, "strum", "to_string"));
        attrs.extend(extract_attrs(&variant.attrs, "strum", "as_str"));
        if is_disabled(&variant.attrs) {
            continue;
        }

        if let Some("true") = unique_attr(&variant.attrs, "strum", "default") {
            if has_default {
                panic!("Can't have multiple default variants");
            }

            if let Tuple(ref fields) = variant.data {
                if fields.len() != 1 {
                    panic!("Default only works on unit structs with a single String parameter");
                }

                default = quote!{
                    default => ::std::result::Result::Ok(#name::#ident (default.into()))
                };
            } else {
                panic!("Default only works on unit structs with a single String parameter");
            }

            has_default = true;
            continue;
        }

        // If we don't have any custom variants, add the default name.
        if attrs.len() == 0 {
            attrs.push(ident.as_ref());
        }

        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(ref fields) => {
                let default = fields
                    .iter()
                    .map(|_| "Default::default()")
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("({})", default))
            }
            Struct(ref fields) => {
                let default = fields
                    .iter()
                    .map(|field| {
                             format!("{}:{}", field.ident.as_ref().unwrap(), "Default::default()")
                         })
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("{{{}}}", default))
            }
        };

        arms.push(quote!{ #(#attrs)|* => ::std::result::Result::Ok(#name::#ident #params) });
    }

    arms.push(default);

    quote!{
        impl #impl_generics ::std::str::FromStr for #name #ty_generics #where_clause {
            type Err = ::strum::ParseError;
            fn from_str(s: &str) -> ::std::result::Result< #name #ty_generics , Self::Err> {
                match s {
                    #(#arms),*
                }
            }
        }
    }
}
