use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, Data, DeriveInput, Fields, LitStr, Token};

use crate::helpers::{
    non_enum_error, non_single_field_variant_error, HasStrumVariantProperties, HasTypeProperties,
};

pub fn display_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };

    let type_properties = ast.get_type_properties()?;

    let mut arms = Vec::new();
    let mut interpolates = false;
    for variant in variants {
        let ident = &variant.ident;
        let variant_properties = variant.get_variant_properties()?;

        if variant_properties.disabled.is_some() {
            continue;
        }

        if variant_properties.transparent.is_some() {
            let arm = super::extract_single_field_variant_and_then(name, variant, |tok| {
                quote! { ::core::fmt::Display::fmt(#tok, f) }
            })
            .map_err(|_| non_single_field_variant_error("transparent"))?;

            arms.push(arm);
            continue;
        }

        // Look at all the serialize attributes.
        let output = variant_properties.get_preferred_name(
            type_properties.case_style,
            type_properties.prefix.as_ref(),
            type_properties.suffix.as_ref(),
        );

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(ref unnamed_fields) => {
                // Transform unnamed params '(String, u8)' to '(ref field0, ref field1)'
                let names: Punctuated<_, Token!(,)> = unnamed_fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(index, field)| {
                        assert!(field.ident.is_none());
                        let ident =
                            syn::parse_str::<Ident>(format!("field{}", index).as_str()).unwrap();
                        quote! { ref #ident }
                    })
                    .collect();
                quote! { (#names) }
            }
            Fields::Named(ref field_names) => {
                // Transform named params '{ name: String, age: u8 }' to '{ ref name, ref age }'
                let names: Punctuated<TokenStream, Token!(,)> = field_names
                    .named
                    .iter()
                    .map(|field| {
                        let ident = field.ident.as_ref().unwrap();
                        quote! { ref #ident }
                    })
                    .collect();

                quote! { {#names} }
            }
        };

        if variant_properties.to_string.is_none() && variant_properties.default.is_some() {
            let arm = super::extract_single_field_variant_and_then(name, variant, |tok| {
                quote! { ::core::fmt::Display::fmt(#tok, f)}
            })
            .map_err(|_| {
                syn::Error::new_spanned(
                    variant,
                    "Default only works on newtype structs with a single String field",
                )
            })?;

            arms.push(arm);
            continue;
        }

        let arm = match variant.fields {
            Fields::Named(ref field_names) => {
                let used_vars = capture_format_string_idents(&output)?;
                if used_vars.is_empty() {
                    quote! { #name::#ident #params => ::core::fmt::Display::fmt(#output, f) }
                } else {
                    // Create args like 'name = name, age = age' for format macro
                    let args: Punctuated<_, Token!(,)> = field_names
                        .named
                        .iter()
                        .filter_map(|field| {
                            let ident = field.ident.as_ref().unwrap();
                            // Only contain variables that are used in format string
                            if !used_vars.contains(ident) {
                                None
                            } else {
                                Some(quote! { #ident = #ident })
                            }
                        })
                        .collect();

                    interpolates = true;
                    quote! {
                        #[allow(unused_variables)]
                        #name::#ident #params => __strum_pad(f, format_args!(#output, #args))
                    }
                }
            }
            Fields::Unnamed(ref unnamed_fields) => {
                let used_vars = capture_format_strings(&output)?;
                if used_vars.iter().any(String::is_empty) {
                    return Err(syn::Error::new_spanned(
                        &output,
                        "Empty {} is not allowed; Use manual numbering ({0})",
                    ));
                }
                if used_vars.is_empty() {
                    quote! { #name::#ident #params => ::core::fmt::Display::fmt(#output, f) }
                } else {
                    let args: Punctuated<_, Token!(,)> = unnamed_fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            assert!(field.ident.is_none());
                            syn::parse_str::<Ident>(format!("field{}", index).as_str()).unwrap()
                        })
                        .collect();
                    interpolates = true;
                    quote! {
                        #[allow(unused_variables)]
                        #name::#ident #params => __strum_pad(f, format_args!(#output, #args))
                    }
                }
            }
            Fields::Unit => {
                let used_vars = capture_format_strings(&output)?;
                if !used_vars.is_empty() {
                    return Err(syn::Error::new_spanned(
                        &output,
                        "Unit variants do not support interpolation",
                    ));
                }

                quote! { #name::#ident #params => ::core::fmt::Display::fmt(#output, f) }
            }
        };

        arms.push(arm);
    }

    if arms.len() < variants.len() {
        arms.push(quote! { _ => panic!("fmt() called on disabled variant.") });
    }

    let pad = interpolates.then(pad_fn);

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::result::Result<(), ::core::fmt::Error> {
                #pad
                match *self {
                    #(#arms),*
                }
            }
        }
    })
}

/// Writes `args` honoring the width, fill, alignment and precision of `f`,
/// the same way `Formatter::pad` does for a `&str`, without buffering.
fn pad_fn() -> TokenStream {
    quote! {
        fn __strum_pad(
            f: &mut ::core::fmt::Formatter,
            args: ::core::fmt::Arguments,
        ) -> ::core::fmt::Result {
            struct Sink;

            impl ::core::fmt::Write for Sink {
                fn write_str(&mut self, _: &str) -> ::core::fmt::Result {
                    ::core::result::Result::Ok(())
                }
            }

            struct Truncate<W> {
                out: W,
                chars_left: ::core::primitive::usize,
            }

            impl<W: ::core::fmt::Write> ::core::fmt::Write for Truncate<W> {
                fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
                    let mut end = 0;
                    for c in s.chars() {
                        if self.chars_left == 0 {
                            break;
                        }
                        self.chars_left -= 1;
                        end += c.len_utf8();
                    }
                    self.out.write_str(&s[..end])
                }
            }

            let (width, precision) = (f.width(), f.precision());
            if width.is_none() && precision.is_none() {
                return f.write_fmt(args);
            }

            let chars_max = precision.unwrap_or(::core::primitive::usize::MAX);
            let padding = match width {
                ::core::option::Option::Some(width) => {
                    let mut counter = Truncate { out: Sink, chars_left: chars_max };
                    ::core::fmt::write(&mut counter, args)?;
                    width.saturating_sub(chars_max - counter.chars_left)
                }
                ::core::option::Option::None => 0,
            };
            let (before, after) = match f.align() {
                ::core::option::Option::Some(::core::fmt::Alignment::Right) => (padding, 0),
                ::core::option::Option::Some(::core::fmt::Alignment::Center) => {
                    (padding / 2, (padding + 1) / 2)
                }
                _ => (0, padding),
            };

            let fill = f.fill();
            for _ in 0..before {
                ::core::fmt::Write::write_char(f, fill)?;
            }
            match precision {
                ::core::option::Option::Some(chars_left) => {
                    ::core::fmt::write(&mut Truncate { out: &mut *f, chars_left }, args)?
                }
                ::core::option::Option::None => f.write_fmt(args)?,
            }
            for _ in 0..after {
                ::core::fmt::Write::write_char(f, fill)?;
            }
            ::core::result::Result::Ok(())
        }
    }
}

fn capture_format_string_idents(string_literal: &LitStr) -> syn::Result<Vec<Ident>> {
    capture_format_strings(string_literal)?
        .into_iter()
        .map(|ident| {
            syn::parse_str::<Ident>(ident.as_str()).map_err(|_| {
                syn::Error::new_spanned(
                    string_literal,
                    "Invalid identifier inside format string bracket",
                )
            })
        })
        .collect()
}

fn capture_format_strings(string_literal: &LitStr) -> syn::Result<Vec<String>> {
    // Remove escaped brackets
    let format_str = string_literal.value().replace("{{", "").replace("}}", "");

    let mut new_var_start_index: Option<usize> = None;
    let mut var_used = Vec::new();

    for (i, chr) in format_str.bytes().enumerate() {
        if chr == b'{' {
            if new_var_start_index.is_some() {
                return Err(syn::Error::new_spanned(
                    string_literal,
                    "Bracket opened without closing previous bracket",
                ));
            }
            new_var_start_index = Some(i);
            continue;
        }

        if chr == b'}' {
            let start_index = new_var_start_index.take().ok_or(syn::Error::new_spanned(
                string_literal,
                "Bracket closed without previous opened bracket",
            ))?;

            let inside_brackets = &format_str[start_index + 1..i];
            let ident_str = inside_brackets.split(":").next().unwrap().trim_end();
            var_used.push(ident_str.to_owned());
        }
    }

    Ok(var_used)
}
