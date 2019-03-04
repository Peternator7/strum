use proc_macro2::{Span, TokenStream};
use syn;

pub(crate) fn enum_count_inner(ast: &syn::DeriveInput) -> TokenStream {
    let n = match ast.data {
        syn::Data::Enum(ref v) => v.variants.len(),
        _ => panic!("EnumCount can only be used with enums"),
    };

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;
    let const_name = &syn::Ident::new(
        &format!("{}_COUNT", name.to_string().to_uppercase()),
        Span::call_site(),
    );

    // Helper is provided for handling complex generic types correctly and effortlessly
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
            // Implementation
            impl #impl_generics ::strum::EnumCount for #name #ty_generics #where_clause {
                fn count() -> usize {
                    #n
                }
            }

            #[allow(dead_code, missing_docs)]
            pub const #const_name: usize = #n;
    }
}
