use proc_macro2::{TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Data, DeriveInput};
use crate::helpers::{HasStrumVariantProperties, HasTypeProperties, non_enum_error};

pub fn enum_assign_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &ast.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(non_enum_error()),
    };
   // let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let type_properties = ast.get_type_properties()?;

    let derives = type_properties.discriminant_derives;

    let derives = quote! {
        #[derive(Debug, PartialEq, Eq, #(#derives),*)]
    };

    let enum_name = &ast.ident;
    let enum_name_struct = format_ident!("{}Struct", enum_name);
    let enum_name_structs = format_ident!("{}Structs", enum_name);
    let enum_name_structs_all = format_ident!("{}StructsAll", enum_name);

    let mut variant_enums: Vec<_> = Vec::new();
    let mut variant_structs: Vec<_> = Vec::new();
    let mut variant_inner_structs: Vec<_> = Vec::new();

    let _variants: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            if variant.get_variant_properties().ok()?.disabled.is_some() {
                return None; //TODO: When Disabled use regular enum args instead of nothing.
            }

            let variant_ident = variant.ident.clone();

            let enum_name_variant_struct = format_ident!("{}_{}_STRUCT", enum_name.to_string().to_uppercase(), variant_ident.to_string().to_uppercase());
            let enum_name_variant_struct_inner = format_ident!("{}_{}_struct", enum_name.to_string().to_lowercase(), variant_ident.to_string().to_lowercase());

            let mut variant_args: Vec<_> = Vec::new();

            let mut args: Vec<_> = Vec::new();
            let mut types: Vec<_> = Vec::new();
            for (i, field) in variant.fields.iter().enumerate() {
                let var;
                if let Some(_var) = field.clone().ident {
                    var = _var
                } else {
                    var = format_ident!("_{}", i);
                }
                let fd = field.ty.to_token_stream();
                args.push(quote! {
                    pub #var: #fd,
                });
                types.push(quote! {
                    #fd
                })
            }
            if args.len() > 0 {
                variant_args.push(quote! {
                    #(#args)*
                });
            }

            /*
            let _fields = match &variant.fields {
                Fields::Named(named) => {
                    let mut args: Vec<_> = Vec::new();
                    for field in named.named.iter() {
                        let var = field.clone().ident.unwrap();
                        let fd = field.ty.to_token_stream();
                        args.push(quote! {
                            pub #var: #fd,
                        })
                    }
                    types.push(quote! {
                            #(#args)*
                    });
                    quote! { (..) }
                }
                Fields::Unnamed(unamed) => {
                    let mut args: Vec<_> = Vec::new();
                    for (i, field) in unamed.unnamed.iter().enumerate() {
                        let var = format_ident!("var_{}", i);
                        let fd = field.ty.to_token_stream();
                        args.push( quote! {
                            pub #var: #fd,
                        } )
                    }
                    types.push(quote! {
                        #(#args)*
                    });
                    quote! { (..) }
                }
                Fields::Unit => {
                    quote! {}
                }
            };
            */

            if variant_args.len() > 0 {
                variant_enums.push(quote! {
                    #variant_ident(#enum_name_structs::#enum_name_variant_struct),
                });

                variant_inner_structs.push(quote! {
                    #enum_name_variant_struct_inner: Option<(#(#types),*)>
                });

                variant_structs.push(quote! {
                    #derives
                    pub struct #enum_name_variant_struct {
                       #(#variant_args)*
                    }
                });
            } else {
                variant_enums.push(quote! {
                    #variant_ident(),
                });
            }

            Some(0)
        })
                .collect();

    Ok(quote! {
        #derives
        pub enum #enum_name_struct {
            #(#variant_enums)*
        }

        #derives
        pub struct #enum_name_structs_all {
            #(#variant_inner_structs)*
        }

        pub mod #enum_name_structs {
            #(#variant_structs)*
        }
    }.into())
}