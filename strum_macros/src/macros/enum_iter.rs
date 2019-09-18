use proc_macro2::TokenStream;
use syn;

use helpers::{extract_meta, MetaIteratorHelpers};

pub fn enum_iter_inner(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let vis = &ast.vis;

    if gen.lifetimes().count() > 0 {
        panic!(
            "Enum Iterator isn't supported on Enums with lifetimes. The resulting enums would \
             be unbounded."
        );
    }

    let phantom_data = if gen.type_params().count() > 0 {
        let g = gen.type_params().map(|param| &param.ident);
        quote! { < ( #(#g),* ) > }
    } else {
        quote! { < () > }
    };

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("EnumIter only works on Enums"),
    };

    let mut arms = Vec::new();
    let enabled = variants
        .iter()
        .filter(|variant| !extract_meta(&variant.attrs).is_disabled());

    for (idx, variant) in enabled.enumerate() {
        use syn::Fields::*;
        let ident = &variant.ident;
        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(ref fields) => {
                let defaults = ::std::iter::repeat(quote!(::std::default::Default::default()))
                    .take(fields.unnamed.len());
                quote! { (#(#defaults),*) }
            }
            Named(ref fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! { {#(#fields: ::std::default::Default::default()),*} }
            }
        };

        arms.push(quote! {#idx => ::std::option::Option::Some(#name::#ident #params)});
    }

    let variant_count = arms.len();
    arms.push(quote! { _ => ::std::option::Option::None });
    let iter_name = syn::parse_str::<syn::Ident>(&format!("{}Iter", name)).unwrap();
    quote! {
        #[allow(missing_docs)]
        #vis struct #iter_name #ty_generics {
            idx: usize,
            marker: ::std::marker::PhantomData #phantom_data,
        }

        impl #impl_generics ::strum::IntoEnumIterator for #name #ty_generics #where_clause {
            type Iterator = #iter_name #ty_generics;
            fn iter() -> #iter_name #ty_generics {
                #iter_name {
                    idx:0,
                    marker: ::std::marker::PhantomData,
                }
            }
        }

        impl #impl_generics Iterator for #iter_name #ty_generics #where_clause {
            type Item = #name #ty_generics;

            fn next(&mut self) -> Option<#name #ty_generics> {
                let output = match self.idx {
                    #(#arms),*
                };

                self.idx += 1;
                output
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let t = #variant_count - self.idx;
                (t, Some(t))
            }
        }

        impl #impl_generics ExactSizeIterator for #iter_name #ty_generics #where_clause {
            fn len(&self) -> usize {
                self.size_hint().0
            }
        }

        impl #impl_generics Clone for #iter_name #ty_generics #where_clause {
            fn clone(&self) -> #iter_name #ty_generics {
                #iter_name {
                    idx: self.idx,
                    marker: self.marker.clone(),
                }
            }
        }
    }
}
