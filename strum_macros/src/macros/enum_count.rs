use crate::helpers::metadata_impl::MetadataImpl;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::helpers::HasTypeProperties;

pub(crate) fn enum_count_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let n = MetadataImpl::new(ast)?.enum_count;
    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    Ok(quote! {
        // Implementation
        impl #impl_generics #strum_module_path::EnumCount for #name #ty_generics #where_clause {
            const COUNT: usize = #n;
        }
    })
}
