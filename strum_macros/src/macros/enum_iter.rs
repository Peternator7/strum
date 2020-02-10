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
            back_idx: usize,
            marker: ::std::marker::PhantomData #phantom_data,
        }

        impl #impl_generics #iter_name #ty_generics #where_clause {
            fn get(&self, idx: usize) -> Option<#name #ty_generics> {
                match idx {
                    #(#arms),*
                }
            }
        }

        impl #impl_generics ::strum::IntoEnumIterator for #name #ty_generics #where_clause {
            type Iterator = #iter_name #ty_generics;
            fn iter() -> #iter_name #ty_generics {
                #iter_name {
                    idx: 0,
                    back_idx: 0,
                    marker: ::std::marker::PhantomData,
                }
            }
        }

        impl #impl_generics Iterator for #iter_name #ty_generics #where_clause {
            type Item = #name #ty_generics;

            fn next(&mut self) -> Option<Self::Item> {
                self.nth(0)
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let t = if self.idx + self.back_idx >= #variant_count { 0 } else { #variant_count - self.idx - self.back_idx };
                (t, Some(t))
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                let idx = self.idx + n + 1;
                if idx + self.back_idx > #variant_count {
                    // We went past the end of the iterator. Freeze idx at #variant_count
                    // so that it doesn't overflow if the user calls this repeatedly.
                    // See PR #76 for context.
                    self.idx = #variant_count;
                    None
                } else {
                    self.idx = idx;
                    self.get(idx - 1)
                }
            }
        }

        impl #impl_generics ExactSizeIterator for #iter_name #ty_generics #where_clause {
            fn len(&self) -> usize {
                self.size_hint().0
            }
        }

        impl #impl_generics DoubleEndedIterator for #iter_name #ty_generics #where_clause {
            fn next_back(&mut self) -> Option<Self::Item> {
                let back_idx = self.back_idx + 1;

                if self.idx + back_idx > #variant_count {
                    // We went past the end of the iterator. Freeze back_idx at #variant_count
                    // so that it doesn't overflow if the user calls this repeatedly.
                    // See PR #76 for context.
                    self.back_idx = #variant_count;
                    None
                } else {
                    self.back_idx = back_idx;
                    self.get(#variant_count - self.back_idx)
                }
            }
        }

        impl #impl_generics Clone for #iter_name #ty_generics #where_clause {
            fn clone(&self) -> #iter_name #ty_generics {
                #iter_name {
                    idx: self.idx,
                    back_idx: self.back_idx,
                    marker: self.marker.clone(),
                }
            }
        }
    }
}
