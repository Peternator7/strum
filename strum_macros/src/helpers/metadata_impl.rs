use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, PathArguments, Type, TypeParen};

use crate::helpers::{non_enum_error, HasStrumVariantProperties, HasTypeProperties};

pub struct MetadataImpl<'a> {
    ast: &'a syn::DeriveInput,
    gen_names: Option<Vec<syn::LitStr>>,
    gen_from_repr: Option<FromReprTokens>,
    pub enum_count: usize,
    pub has_additional_data: bool,
}

pub struct FromReprTokens {
    pub constant_defs: Vec<TokenStream>,
    pub match_arms: Vec<TokenStream>,
}

impl<'a> MetadataImpl<'a> {
    pub fn new(ast: &'a DeriveInput) -> Self {
        MetadataImpl {
            ast,
            enum_count: 0,
            gen_names: None,
            gen_from_repr: None,
            has_additional_data: false,
        }
    }

    pub fn use_name_info(mut self) -> Self {
        self.gen_names = Some(Vec::new());
        self
    }

    pub fn use_from_repr(mut self) -> Self {
        self.gen_from_repr = Some(FromReprTokens {
            constant_defs: Vec::new(),
            match_arms: Vec::new(),
        });
        self
    }

    pub fn discriminant_type(&self) -> Type {
        let mut discriminant_type: Type = syn::parse("usize".parse().unwrap()).unwrap();
        for attr in &self.ast.attrs {
            let path = &attr.path;
            let tokens = &attr.tokens;
            if path.leading_colon.is_some() {
                continue;
            }
            if path.segments.len() != 1 {
                continue;
            }
            let segment = path.segments.first().unwrap();
            if segment.ident != "repr" {
                continue;
            }
            if segment.arguments != PathArguments::None {
                continue;
            }
            let typ_paren = match syn::parse2::<Type>(tokens.clone()) {
                Ok(Type::Paren(TypeParen { elem, .. })) => *elem,
                _ => continue,
            };
            let inner_path = match &typ_paren {
                Type::Path(t) => t,
                _ => continue,
            };
            if let Some(seg) = inner_path.path.segments.last() {
                for t in &[
                    "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize",
                ] {
                    if seg.ident == t {
                        discriminant_type = typ_paren;
                        break;
                    }
                }
            }
        }
        discriminant_type
    }

    fn params_from_fields(fields: &syn::Fields) -> (TokenStream, bool) {
        use syn::Fields::*;
        match &fields {
            Unit => (quote! {}, false),
            Unnamed(fields) => {
                let defaults = ::std::iter::repeat(quote!(::core::default::Default::default()))
                    .take(fields.unnamed.len());
                (quote! { (#(#defaults),*) }, true)
            }
            Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                (
                    quote! { {#(#fields: ::core::default::Default::default()),*} },
                    true,
                )
            }
        }
    }

    fn case_style(&self) -> Option<crate::helpers::case_style::CaseStyle> {
        if let Ok(props) = self.ast.get_type_properties() {
            props.case_style
        } else {
            None
        }
    }

    pub fn generate(&mut self) -> syn::Result<()> {
        let name = &self.ast.ident;
        let discriminant_type = self.discriminant_type();

        let case_style = self.case_style();
        let mut prev_const_var_ident = None;
        let variants = match &self.ast.data {
            Data::Enum(v) => &v.variants,
            _ => return Err(non_enum_error()),
        };
        self.enum_count = variants.len();
        for variant in variants {
            let props = variant.get_variant_properties()?;

            if let Some(variant_names) = &mut self.gen_names {
                variant_names.push(props.get_preferred_name(case_style));
            }

            if let Some(FromReprTokens {
                match_arms,
                constant_defs,
            }) = &mut self.gen_from_repr
            {
                if props.disabled.is_some() {
                    continue;
                }

                let ident = &variant.ident;
                let (params, has_additional_data) = Self::params_from_fields(&variant.fields);
                if has_additional_data {
                    self.has_additional_data = has_additional_data;
                }

                let const_var_ident = {
                    use heck::ToShoutySnakeCase;
                    let const_var_str = format!("{}_DISCRIMINANT", ident).to_shouty_snake_case();
                    format_ident!("{}", const_var_str)
                };

                let const_val_expr = match &variant.discriminant {
                    Some((_, expr)) => quote! { #expr },
                    None => match &prev_const_var_ident {
                        Some(prev) => quote! { #prev + 1 },
                        None => quote! { 0 },
                    },
                };

                constant_defs
                    .push(quote! {const #const_var_ident: #discriminant_type = #const_val_expr;});
                match_arms.push(quote! {v if v == #const_var_ident => ::core::option::Option::Some(#name::#ident #params)});
                prev_const_var_ident = Some(const_var_ident);
            }
        }
        if let Some(FromReprTokens { match_arms, .. }) = &mut self.gen_from_repr {
            match_arms.push(quote! { _ => ::core::option::Option::None });
        }

        Ok(())
    }

    pub fn variant_names(&self) -> &Option<Vec<syn::LitStr>> {
        &self.gen_names
    }

    pub fn from_repr(&self) -> &Option<FromReprTokens> {
        &self.gen_from_repr
    }

    pub fn enum_count(&self) -> usize {
        self.enum_count
    }
}
