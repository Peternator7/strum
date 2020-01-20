use crate::helpers::{MetaHelpers, MetaIteratorHelpers, MetaListHelpers};
use proc_macro2::{Span, TokenStream};
use syn;

use helpers::extract_meta;

/// Attributes to copy from the main enum's variants to the discriminant enum's variants.
///
/// Attributes not in this list may be for other `proc_macro`s on the main enum, and may cause
/// compilation problems when copied across.
const ATTRIBUTES_TO_COPY: &[&str] = &["doc", "cfg", "allow", "deny"];

pub fn enum_discriminants_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let vis = &ast.vis;

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumDiscriminants only works on Enums"),
    };

    // Derives for the generated enum
    let type_meta = extract_meta(&ast.attrs);
    let discriminant_attrs = type_meta
        .find_attribute("strum_discriminants")
        .collect::<Vec<&syn::Meta>>();

    let derives = discriminant_attrs
        .find_attribute("derive")
        .map(|meta| meta.path())
        .collect::<Vec<_>>();

    let derives = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, #(#derives),*)]
    };

    // Work out the name
    let default_name = syn::Path::from(syn::Ident::new(
        &format!("{}Discriminants", name.to_string()),
        Span::call_site(),
    ));

    let discriminants_name = discriminant_attrs
        .iter()
        .filter_map(|meta| meta.try_metalist())
        .filter(|list| list.path.is_ident("name"))
        // We want exactly zero or one items. Start with the assumption we have zero, i.e. None
        // Then set our output to the first value we see. If fold is called again and we already
        // have a value, panic.
        .fold(None, |acc, val| match acc {
            Some(_) => panic!("Expecting a single attribute 'name' in EnumDiscriminants."),
            None => Some(val),
        })
        .map(|meta| meta.expand_inner())
        .and_then(|metas| metas.into_iter().map(|meta| meta.path()).next())
        .unwrap_or(&default_name);

    // Pass through all other attributes
    let pass_though_attributes = discriminant_attrs
        .iter()
        .filter(|meta| {
            let path = meta.path();
            !path.is_ident("derive") && !path.is_ident("name")
        })
        .map(|meta| quote! { #[ #meta ] })
        .collect::<Vec<_>>();

    // Add the variants without fields, but exclude the `strum` meta item
    let mut discriminants = Vec::new();
    for variant in variants {
        let ident = &variant.ident;

        // Don't copy across the "strum" meta attribute.
        let attrs = variant.attrs.iter().filter(|attr| {
            ATTRIBUTES_TO_COPY
                .iter()
                .any(|attr_whitelisted| attr.path.is_ident(attr_whitelisted))
        });

        discriminants.push(quote! { #(#attrs)* #ident });
    }

    // Ideally:
    //
    // * For `Copy` types, we `impl From<TheEnum> for TheEnumDiscriminants`
    // * For `!Copy` types, we `impl<'enum> From<&'enum TheEnum> for TheEnumDiscriminants`
    //
    // That way we ensure users are not able to pass a `Copy` type by reference. However, the
    // `#[derive(..)]` attributes are not in the parsed tokens, so we are not able to check if a
    // type is `Copy`, so we just implement both.
    //
    // See <https://github.com/dtolnay/syn/issues/433>
    // ---
    // let is_copy = unique_meta_list(type_meta.iter(), "derive")
    //     .map(extract_list_metas)
    //     .map(|metas| {
    //         metas
    //             .filter_map(get_meta_ident)
    //             .any(|derive| derive.to_string() == "Copy")
    //     }).unwrap_or(false);

    let arms = variants
        .iter()
        .map(|variant| {
            let ident = &variant.ident;

            use syn::Fields::*;
            let params = match variant.fields {
                Unit => quote! {},
                Unnamed(ref _fields) => {
                    quote! { (..) }
                }
                Named(ref _fields) => {
                    quote! { { .. } }
                }
            };

            quote! { #name::#ident #params => #discriminants_name::#ident }
        })
        .collect::<Vec<_>>();

    let from_fn_body = quote! { match val { #(#arms),* } };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let impl_from = quote! {
        impl #impl_generics ::std::convert::From< #name #ty_generics > for #discriminants_name #where_clause {
            fn from(val: #name #ty_generics) -> #discriminants_name {
                #from_fn_body
            }
        }
    };
    let impl_from_ref = {
        let mut generics = ast.generics.clone();

        let lifetime = parse_quote!('_enum);
        let enum_life = quote! { & #lifetime };
        generics.params.push(lifetime);

        // Shadows the earlier `impl_generics`
        let (impl_generics, _, _) = generics.split_for_impl();

        quote! {
            impl #impl_generics ::std::convert::From< #enum_life #name #ty_generics > for #discriminants_name #where_clause {
                fn from(val: #enum_life #name #ty_generics) -> #discriminants_name {
                    #from_fn_body
                }
            }
        }
    };

    quote! {
        /// Auto-generated discriminant enum variants
        #derives
        #(#pass_though_attributes)*
        #vis enum #discriminants_name {
            #(#discriminants),*
        }

        #impl_from
        #impl_from_ref
    }
}
