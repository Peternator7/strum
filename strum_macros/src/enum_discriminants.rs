use proc_macro2::{Span, TokenStream};
use syn;

use helpers::{
    extract_list_metas, extract_meta, filter_metas, get_meta_ident, get_meta_list, unique_meta_list,
};

pub fn enum_discriminants_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let vis = &ast.vis;

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumDiscriminants only works on Enums"),
    };

    // Derives for the generated enum
    let type_meta = extract_meta(&ast.attrs);
    let discriminant_attrs = get_meta_list(type_meta.iter(), "strum_discriminants")
        .flat_map(|meta| extract_list_metas(meta).collect::<Vec<_>>())
        .collect::<Vec<&syn::Meta>>();
    let derives = get_meta_list(discriminant_attrs.iter().map(|&m| m), "derive")
        .flat_map(extract_list_metas)
        .filter_map(get_meta_ident)
        .collect::<Vec<_>>();

    let derives = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, #(#derives),*)]
    };

    // Work out the name
    let default_name = syn::Ident::new(
        &format!("{}Discriminants", name.to_string()),
        Span::call_site(),
    );

    let discriminants_name = unique_meta_list(discriminant_attrs.iter().map(|&m| m), "name")
        .map(extract_list_metas)
        .and_then(|metas| metas.filter_map(get_meta_ident).next())
        .unwrap_or(&default_name);

    // Pass through all other attributes
    let pass_though_attributes =
        filter_metas(discriminant_attrs.iter().map(|&m| m), |meta| match meta {
            syn::Meta::List(ref metalist) => metalist.ident != "derive" && metalist.ident != "name",
            _ => true,
        }).map(|meta| quote! { #[ #meta ] })
        .collect::<Vec<_>>();

    // Add the variants without fields, but exclude the `strum` meta item
    let mut discriminants = Vec::new();
    for variant in variants {
        let ident = &variant.ident;

        // Don't copy across the "strum" meta attribute.
        let attrs = variant.attrs.iter().filter(|attr| {
            attr.interpret_meta().map_or(true, |meta| match meta {
                syn::Meta::List(ref metalist) => metalist.ident != "strum",
                _ => true,
            })
        });

        discriminants.push(quote!{ #(#attrs)* #ident });
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
                Unit => quote!{},
                Unnamed(ref _fields) => {
                    quote! { (..) }
                }
                Named(ref _fields) => {
                    quote! { { .. } }
                }
            };

            quote! { #name::#ident #params => #discriminants_name::#ident }
        }).collect::<Vec<_>>();
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

    quote!{
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
