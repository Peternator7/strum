use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

use crate::helpers::{non_enum_error, snakify, HasStrumVariantProperties, HasTypeProperties};

pub fn enum_iter_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let vis = &ast.vis;
    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();
    let doc_comment = format!("An iterator over the variants of [{}]", name);

    if gen.lifetimes().count() > 0 {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro doesn't support enums with lifetimes. \
             The resulting enums would be unbounded.",
        ));
    }

    let phantom_data = if gen.type_params().count() > 0 {
        let g = gen.type_params().map(|param| &param.ident);
        quote! { < fn() -> ( #(#g),* ) > }
    } else {
        quote! { < fn() -> () > }
    };

    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let mut get_arms = Vec::new();
    let mut idx = 0usize;
    let mut nested_iters = Vec::new();
    let mut nested_iter_inits = Vec::new();
    let mut nested_iter_clones = Vec::new();
    let mut nested_iter_size_hints = Vec::new();
    let iter_res_name = syn::parse_str::<Ident>(&format!("{}IterRes", name)).unwrap();

    for variant in variants {
        if variant.get_variant_properties()?.disabled.is_some() {
            continue;
        }

        let ident = &variant.ident;
        let get_res = match &variant.fields {
            Fields::Unit => quote! {
                #iter_res_name::Done(#name::#ident)
            },
            Fields::Unnamed(fields) => {
                if variant.get_variant_properties()?.flatten.is_some() {
                    if fields.unnamed.len() != 1 {
                        return Err(syn::Error::new_spanned(
                            variant,
                            "Flatten only works on newtype structs with a single field",
                        ));
                    }
                    let field = &fields.unnamed[0];

                    let nested_iter =
                        syn::parse_str::<Ident>(&format!("{}_iter", snakify(&ident.to_string())))
                            .unwrap();

                    nested_iters.push(quote! {
                        // We're using Option<> here so that we can disable them
                        // when they are exhausted.
                        #nested_iter:  ::core::option::Option<
                            <#field as #strum_module_path::IntoEnumIterator>
                                ::Iterator
                        >,
                    });
                    nested_iter_inits.push(quote! {
                        #nested_iter: Some(<#field>::iter()),
                    });
                    nested_iter_clones.push(quote! {
                        #nested_iter: self.#nested_iter.clone(),
                    });
                    nested_iter_size_hints.push(quote! {
                        // For each nested iterator, we want to replace its
                        // trivial length with its real length.
                        //
                        // For that we first add its t.len(), and then decrement
                        // it by one, because we already accounted for it.
                        // Except when iterator is disabled, in such case we
                        // don't want to remove anything, because it's already
                        // zero.
                        + self.#nested_iter.as_ref().map_or(0, |t| {
                            t.len()
                        })
                        - if self.#nested_iter.is_some() { 1 } else { 0 }
                    });
                    // Delegate .get() call to .next()/.next_back() on nested
                    // iterator.
                    quote! {{
                        let next_inner = if forward {
                            self.#nested_iter.as_mut().and_then(|t| t.next())
                        } else {
                            self.#nested_iter.as_mut().and_then(|t| t.next_back())
                        };

                        if let Some(it) = next_inner {
                            #iter_res_name::DoneStep(#name::#ident(it))
                        } else {
                            // Disable the nested iterator if exhausted
                            self.#nested_iter.take();
                            #iter_res_name::EndStep
                        }
                    }}
                } else {
                    let defaults =
                        ::core::iter::repeat(quote!(::core::default::Default::default()))
                            .take(fields.unnamed.len());

                    quote! { #iter_res_name::Done(#name::#ident (#(#defaults),*)) }
                }
            }
            Fields::Named(fields) => {
                if variant.get_variant_properties()?.flatten.is_some() {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "Flatten only works on newtype structs with a single field",
                    ));
                }

                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                quote! {
                    #iter_res_name::Done(
                        #name::#ident {#(#fields: ::core::default::Default::default()),*}
                    )
                }
            }
        };

        get_arms.push(quote! {#idx => #get_res});
        idx += 1;
    }

    let variant_count = get_arms.len();
    get_arms.push(quote! { _ => #iter_res_name::End });

    let iter_name = syn::parse_str::<Ident>(&format!("{}Iter", name)).unwrap();

    // Create a string literal "MyEnumIter" to use in the debug impl.
    let iter_name_debug_struct =
        syn::parse_str::<syn::LitStr>(&format!("\"{}\"", iter_name)).unwrap();

    let iter_struct = quote! {
        enum #iter_res_name #impl_generics {
            Done(#name #ty_generics),
            DoneStep(#name #ty_generics),
            EndStep,
            End,
        }

        #[doc = #doc_comment]
        #[allow(
            missing_copy_implementations,
        )]
        #vis struct #iter_name #impl_generics {
            idx: usize,
            back_idx: usize,
            #(#nested_iters)*
            marker: ::core::marker::PhantomData #phantom_data,
        }
    };

    let debug_impl = quote! {
        impl #impl_generics ::core::fmt::Debug for #iter_name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                // We don't know if the variants implement debug themselves so the only thing we
                // can really show is how many elements are left.
                f.debug_struct(#iter_name_debug_struct)
                    .field("len", &self.len())
                    .finish()
            }
        }
    };

    let iter_struct_impl = quote! {
        impl #impl_generics #iter_name #ty_generics #where_clause {
            fn get(&mut self, idx: usize, forward: bool) -> #iter_res_name #ty_generics {
                match idx {
                    #(#get_arms),*
                }
            }
        }
    };

    let into_enum_iter_impl = quote! {

        impl #impl_generics #strum_module_path::IntoEnumIterator for #name #ty_generics #where_clause {
            type Iterator = #iter_name #ty_generics;

            #[inline]
            fn iter() -> #iter_name #ty_generics {
                #iter_name {
                    idx: 0,
                    back_idx: 0,
                    #(#nested_iter_inits)*
                    marker: ::core::marker::PhantomData,
                }
            }
        }
    };

    let iter_impl = quote! {
        impl #impl_generics Iterator for #iter_name #ty_generics #where_clause {
            type Item = #name #ty_generics;

            #[inline]
            fn next(&mut self) -> ::core::option::Option<<Self as Iterator>::Item> {
                self.nth(0)
            }

            #[inline]
            fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
                let t = if self.idx + self.back_idx >= #variant_count {
                    0
                } else {
                    #variant_count
                        #(#nested_iter_size_hints)*
                        - self.idx
                        - self.back_idx
                };
                (t, Some(t))
            }

            #[inline]
            fn nth(&mut self, n: usize) -> ::core::option::Option<<Self as Iterator>::Item> {
                let idx = self.idx + n + 1;
                if idx + self.back_idx > #variant_count {
                    // We went past the end of the iterator. Freeze idx at #variant_count
                    // so that it doesn't overflow if the user calls this repeatedly.
                    // See PR #76 for context.
                    self.idx = #variant_count;
                    return ::core::option::Option::None
                }

                match #iter_name::get(self, self.idx + n, true) {
                    #iter_res_name::Done(x) => {
                        // move to requested, and past it
                        self.idx += n + 1;
                        Some(x)
                    }
                    #iter_res_name::DoneStep(x) => {
                        // move to requested, but not past it
                        self.idx += n;
                        Some(x)
                    }
                    #iter_res_name::EndStep => {
                        // ok, this one failed, move past it and request again
                        self.idx += 1;
                        self.nth(0)
                    }
                    #iter_res_name::End => None,
                }
            }
        }
    };

    let exact_size_iter_impl = quote! {

        impl #impl_generics ExactSizeIterator for #iter_name #ty_generics #where_clause {
            #[inline]
            fn len(&self) -> usize {
                self.size_hint().0
            }
        }
    };

    let double_ended_iter_impl = quote! {
            impl #impl_generics #iter_name #ty_generics #where_clause {
                fn nth_back(&mut self, back_n: usize) ->
                    ::core::option::Option<<Self as Iterator>::Item>
                {
                    if self.back_idx + self.idx >= #variant_count {
                        return None;
                    }

                    let res = match #iter_name::get(
                        self,
                        #variant_count - self.back_idx - back_n - 1,
                        false,
                    ) {
                        #iter_res_name::Done(x) => {
                            // move to requested, and past it
                            self.back_idx += 1;
                            Some(x)
                        }
                        #iter_res_name::DoneStep(x) => {
                            // move to requested, but not past it
                            Some(x)
                        }
                        #iter_res_name::EndStep => {
                            // ok, this one failed, try the next one
                            self.back_idx += 1;
                            self.nth_back(0)
                        }
                        #iter_res_name::End => None,
                    };
                    res
                }
    }

            impl #impl_generics DoubleEndedIterator for #iter_name #ty_generics #where_clause {
                #[inline]
                fn next_back(&mut self) -> ::core::option::Option<<Self as Iterator>::Item> {
                    let back_idx = self.back_idx + 1;

                    if self.idx + back_idx > #variant_count {
                        // We went past the end of the iterator. Freeze back_idx at #variant_count
                        // so that it doesn't overflow if the user calls this repeatedly.
                        // See PR #76 for context.
                        self.back_idx = #variant_count;
                        ::core::option::Option::None
                    } else {
                        self.nth_back(0)
                    }
                }
            }
        };

    let fused_iter_impl = quote! {

        impl #impl_generics ::core::iter::FusedIterator for #iter_name #ty_generics #where_clause { }

    };

    let clone_impl = quote! {
        impl #impl_generics Clone for #iter_name #ty_generics #where_clause {
            #[inline]
            fn clone(&self) -> #iter_name #ty_generics {
                #iter_name {
                    idx: self.idx,
                    back_idx: self.back_idx,
                    #(#nested_iter_clones)*
                    marker: self.marker.clone(),
                }
            }
        }
    };

    let fragments = vec![
        // iter type
        iter_struct,
        iter_struct_impl,
        // iterator traits
        into_enum_iter_impl,
        iter_impl,
        double_ended_iter_impl,
        exact_size_iter_impl,
        fused_iter_impl,
        // misc traits for iter type
        debug_impl,
        clone_impl,
    ];

    Ok(quote! {
        #(#fragments)*
    })
}
