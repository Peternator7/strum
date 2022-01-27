use crate::helpers::metadata_impl::{FromReprTokens, MetadataImpl};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn enum_metadata_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let mut metadata = MetadataImpl::new(ast)?.use_name_info().use_from_repr()?;
    let discriminant_type = metadata.discriminant_type();
    metadata.generate()?;
    let enum_count = metadata.enum_count();
    let variant_names = metadata.variant_names().as_ref().unwrap();
    let (impl_generics, ty_generics, where_clause) = &metadata.generics_split();

    let FromReprTokens {
        constant_defs,
        match_arms,
    } = &metadata.from_repr().as_ref().unwrap();

    Ok(quote! {
        impl #impl_generics EnumMetadata for #name #ty_generics #where_clause {
            #[doc = "The Repr type."]
            type Repr = #discriminant_type;
            #[doc = "The Enum type, typically Self unless implementing EnumMetadata for another enum type."]
            type EnumT = Self;

            const VARIANTS: &'static [&'static str] = &[ #(#variant_names),* ];
            const COUNT: usize = #enum_count;
            const REPR_SIZE: usize = ::core::mem::size_of::<Self::Repr>();

            fn to_repr(self) -> #discriminant_type {
               self as #discriminant_type
            }

            // Note: synchronize changes with `FromRepr::from_repr`,
            // it duplicates this logic in an inherent impl.
            // Making it possible to have both impls on the same type;
            // so their behavior must be kept the same.
            fn from_repr(discriminant: #discriminant_type) -> Option<Self> {
                #(#constant_defs)*
                match discriminant {
                    #(#match_arms),*
                }
            }
        }
    })
}
