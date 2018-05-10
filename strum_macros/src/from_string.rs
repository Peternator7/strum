
use quote;
use syn;

use helpers::{unique_attr, extract_attrs, is_disabled};

pub fn from_string_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("FromString only works on Enums"),
    };

    let mut has_default = false;
    let mut default =
        quote! { _ => ::std::result::Result::Err(::strum::ParseError::VariantNotFound) };
    let mut arms = Vec::new();
    for variant in variants {
        use syn::Fields::*;
        let ident = &variant.ident;

        // Look at all the serialize attributes.
        let mut attrs = extract_attrs(&variant.attrs, "strum", "serialize");
        attrs.extend(extract_attrs(&variant.attrs, "strum", "to_string"));
        if is_disabled(&variant.attrs) {
            continue;
        }

        if unique_attr(&variant.attrs, "strum", "default").map_or(false, |s| s == "true") {
            if has_default {
                panic!("Can't have multiple default variants");
            }

            if let Unnamed(ref fields) = variant.fields {
                if fields.unnamed.len() != 1 {
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
            attrs.push(ident.to_string());
        }

        let params = match variant.fields {
            Unit => quote!{},
            Unnamed(ref fields) => {
                let defaults = ::std::iter::repeat(quote!(Default::default()))
                    .take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Named(ref fields) => {
                let fields = fields.named.iter().map(|field| field.ident.unwrap());
                quote! { {#(#fields: Default::default()),*} }
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
