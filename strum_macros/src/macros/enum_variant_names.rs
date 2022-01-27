use crate::helpers::metadata_impl::MetadataImpl;
use crate::helpers::HasTypeProperties;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn enum_variant_names_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let mut metadata = MetadataImpl::new(ast)?.use_name_info();
    metadata.generate()?;
    let variant_names = metadata.variant_names().as_ref().unwrap();
    let (impl_generics, ty_generics, where_clause) = &metadata.generics_split();

    // Derives for the generated enum
    let type_properties = ast.get_type_properties()?;
    let strum_module_path = type_properties.crate_module_path();

    Ok(quote! {
        impl #impl_generics #strum_module_path::VariantNames for #name #ty_generics #where_clause {
            const VARIANTS: &'static [&'static str] = &[ #(#variant_names),* ];
        }
    })
}
