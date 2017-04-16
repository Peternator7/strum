use quote;
use syn;

use helpers::is_disabled;

fn extract_properties(ast: &syn::Variant) -> Vec<(&syn::Ident, &syn::Lit)> {
    use syn::*;
    ast.attrs
        .iter()
        .filter_map(|attr| {
            // Look for all the strum attributes
            if let &Attribute { value: MetaItem::List(ref ident, ref nested), .. } = attr {
                if ident == "strum" {
                    return Option::Some(nested);
                }
            }

            Option::None
        })
        .flat_map(|prop| prop)
        .filter_map(|prop| {
            // Look for all the recursive property attributes
            if let &NestedMetaItem::MetaItem(MetaItem::List(ref ident, ref nested)) = prop {
                if ident == "props" {
                    return Option::Some(nested);
                }
            }

            Option::None
        })
        .flat_map(|prop| prop)
        .filter_map(|prop| {
            // Only look at key value pairs
            if let &NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, ref value)) = prop {
                return Option::Some((ident, value));
            }

            Option::None
        })
        .collect()
}

pub fn enum_properties_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("EnumProp only works on Enums"),
    };

    let mut arms = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let mut string_arms = Vec::new();
        let mut bool_arms = Vec::new();
        let mut num_arms = Vec::new();
        // But you can disable the messages.
        if is_disabled(&variant.attrs) {
            continue;
        }

        use syn::VariantData::*;
        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(..) => quote::Ident::from("(..)"),
            Struct(..) => quote::Ident::from("{..}"),
        };

        for (key, value) in extract_properties(&variant) {
            use syn::Lit::*;
            let key = key.as_ref();
            match *value {
                Str(ref s, ..) => {
                    string_arms.push(quote!{ #key => ::std::option::Option::Some( #s )})
                }
                Bool(b) => bool_arms.push(quote!{ #key => ::std::option::Option::Some( #b )}),
                Int(i, ..) => num_arms.push(quote!{ #key => ::std::option::Option::Some( #i )}),
                _ => {}
            }
        }

        string_arms.push(quote!{ _ => ::std::option::Option::None });
        bool_arms.push(quote!{ _ => ::std::option::Option::None });
        num_arms.push(quote!{ _ => ::std::option::Option::None });

        arms.push(quote!{
            &#name::#ident #params => {
                match prop {
                    #(#string_arms),*
                }
            }
        });
    }

    if arms.len() < variants.len() {
        arms.push(quote!{ _ => ::std::option::Option::None });
    }

    quote!{
        impl #impl_generics ::strum::EnumProperty for #name #ty_generics #where_clause {
            fn get_str(&self, prop: &str) -> ::std::option::Option<&'static str> {
                match self {
                    #(#arms),*
                }
            }
        }
    }
}
