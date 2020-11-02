use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse_quote;
use syn::{Data, DeriveInput};

use crate::helpers::{non_enum_error, HasTypeProperties};

/// Attributes to copy from the main enum's variants to the discriminant enum's variants.
///
/// Attributes not in this list may be for other `proc_macro`s on the main enum, and may cause
/// compilation problems when copied across.
const ATTRIBUTES_TO_COPY: &[&str] = &["doc", "cfg", "allow", "deny"];

pub fn enum_discriminants_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let vis = &ast.vis;

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    // Derives for the generated enum
    let type_properties = ast.get_type_properties()?;

    let derives = type_properties.discriminant_derives;

    let derives = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, #(#derives),*)]
    };

    // Work out the name
    let default_name = syn::Ident::new(
        &format!("{}Discriminants", name.to_string()),
        Span::call_site(),
    );

    let discriminants_name = type_properties.discriminant_name.unwrap_or(default_name);
    let discriminants_vis = type_properties.discriminant_vis.unwrap_or_else(|| vis.clone());

    // Pass through all other attributes
    let pass_though_attributes = type_properties.discriminant_others;

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
            let params = match &variant.fields {
                Unit => quote! {},
                Unnamed(_fields) => {
                    quote! { (..) }
                }
                Named(_fields) => {
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

    Ok(quote! {
        /// Auto-generated discriminant enum variants
        #derives
        #(#[ #pass_though_attributes ])*
        #discriminants_vis enum #discriminants_name {
            #(#discriminants),*
        }

        #impl_from
        #impl_from_ref
    })
}
