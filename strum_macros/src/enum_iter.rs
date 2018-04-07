use quote;
use syn;

use helpers::is_disabled;

pub fn enum_iter_inner(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();
    let vis = &ast.vis;

    if gen.lifetimes.len() > 0 {
        panic!("Enum Iterator isn't supported on Enums with lifetimes. The resulting enums would \
                 be unbounded.");
    }

    let phantom_data = if gen.ty_params.len() > 0 {
        let g = gen.ty_params
            .iter()
            .map(|param| &param.ident)
            .collect::<Vec<_>>();
        quote!{ < ( #(#g),* ) > }
    } else {
        quote! { < () > }
    };

    let variants = match ast.body {
        syn::Body::Enum(ref v) => v,
        _ => panic!("EnumIter only works on Enums"),
    };

    let mut arms = Vec::new();
    let enabled = variants
        .iter()
        .filter(|variant| !is_disabled(&variant.attrs));

    for (idx, variant) in enabled.enumerate() {
        use syn::VariantData::*;
        let ident = &variant.ident;
        let params = match variant.data {
            Unit => quote::Ident::from(""),
            Tuple(ref fields) => {
                let default = fields
                    .iter()
                    .map(|_| "::std::default::Default::default()")
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("({})", default))
            }
            Struct(ref fields) => {
                let default = fields
                    .iter()
                    .map(|field| {
                             format!("{}: {}",
                                     field.ident.as_ref().unwrap(),
                                     "::std::default::Default::default()")
                         })
                    .collect::<Vec<_>>()
                    .join(", ");

                quote::Ident::from(&*format!("{{{}}}", default))
            }
        };

        arms.push(quote!{#idx => ::std::option::Option::Some(#name::#ident #params)});
    }

    let variant_count = arms.len();
    arms.push(quote! { _ => ::std::option::Option::None });
    let iter_name = quote::Ident::from(&*format!("{}Iter", name));
    quote!{
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
