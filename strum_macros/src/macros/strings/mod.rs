use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Ident, Variant};

pub mod as_ref_str;
pub mod display;
pub mod from_string;
pub mod to_string;

struct NonSingleFieldEnum;

fn extract_single_field_variant_and_then<F>(
    name: &Ident,
    variant: &Variant,
    return_val_fn: F,
) -> Result<TokenStream, NonSingleFieldEnum>
where
    F: Fn(&TokenStream) -> TokenStream,
{
    let variant_ident = &variant.ident;

    let pattern_and_return = match &variant.fields {
        Fields::Unnamed(f) if f.unnamed.len() == 1 => {
            let pat = &quote! { field0 };
            let ret_val = return_val_fn(pat);
            quote! { (ref #pat) => #ret_val }
        }
        Fields::Named(f) if f.named.len() == 1 => {
            let ident = &quote! { f.named.last().unwrap().ident.as_ref().unwrap() };
            let ret_val = return_val_fn(ident);
            quote! { {ref #ident} => #ret_val }
        }
        _ => return Err(NonSingleFieldEnum),
    };

    Ok(quote! { #name::#variant_ident #pattern_and_return })
}
