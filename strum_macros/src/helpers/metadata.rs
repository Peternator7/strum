use proc_macro2::{Span, TokenStream};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, DeriveInput, Ident, LitStr, Path, Token, Variant,
};

use super::case_style::CaseStyle;

mod kw {
    use syn::custom_keyword;

    // enum metadata
    custom_keyword!(serialize_all);

    // enum discriminant metadata
    custom_keyword!(derive);
    custom_keyword!(name);

    // variant metadata
    custom_keyword!(message);
    custom_keyword!(detailed_message);
    custom_keyword!(serialize);
    custom_keyword!(to_string);
    custom_keyword!(disabled);
    custom_keyword!(default);
    custom_keyword!(props);
}

pub enum EnumMeta {
    SerializeAll {
        serialize_all_kw: kw::serialize_all,
        case_style: CaseStyle,
    },
}

impl Parse for EnumMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let serialize_all_kw = input.parse::<kw::serialize_all>()?;
        input.parse::<Token![=]>()?;
        let case_style = input.parse()?;
        Ok(Self::SerializeAll {
            serialize_all_kw,
            case_style,
        })
    }
}

impl Spanned for EnumMeta {
    fn span(&self) -> Span {
        match self {
            Self::SerializeAll {
                serialize_all_kw, ..
            } => serialize_all_kw.span(),
        }
    }
}

pub enum EnumDiscriminantsMeta {
    Derive { kw: kw::derive, paths: Vec<Path> },
    Name { kw: kw::name, name: Ident },
    Other { path: Path, nested: TokenStream },
}

impl Parse for EnumDiscriminantsMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::derive) {
            let kw = input.parse()?;
            let content;
            parenthesized!(content in input);
            let paths = content.parse_terminated::<_, Token![,]>(Path::parse)?;
            Ok(Self::Derive {
                kw,
                paths: paths.into_iter().collect(),
            })
        } else if input.peek(kw::name) {
            let kw = input.parse()?;
            let content;
            parenthesized!(content in input);
            let name = content.parse()?;
            Ok(Self::Name { kw, name })
        } else {
            let path = input.parse()?;
            let content;
            parenthesized!(content in input);
            let nested = content.parse()?;
            Ok(Self::Other { path, nested })
        }
    }
}

impl Spanned for EnumDiscriminantsMeta {
    fn span(&self) -> Span {
        match self {
            Self::Derive { kw, .. } => kw.span,
            Self::Name { kw, .. } => kw.span,
            Self::Other { path, .. } => path.span(),
        }
    }
}

pub trait DeriveInputExt {
    /// Get all the strum metadata associated with an enum.
    fn get_metadata(&self) -> syn::Result<Vec<EnumMeta>>;

    /// Get all the strum_discriminants metadata associated with an enum.
    fn get_discriminants_metadata(&self) -> syn::Result<Vec<EnumDiscriminantsMeta>>;
}

impl DeriveInputExt for DeriveInput {
    fn get_metadata(&self) -> syn::Result<Vec<EnumMeta>> {
        get_metadata_inner("strum", &self.attrs)
    }

    fn get_discriminants_metadata(&self) -> syn::Result<Vec<EnumDiscriminantsMeta>> {
        get_metadata_inner("strum_discriminants", &self.attrs)
    }
}

pub enum VariantMeta {
    Message {
        kw: kw::message,
        value: LitStr,
    },
    DetailedMessage {
        kw: kw::detailed_message,
        value: LitStr,
    },
    Serialize {
        kw: kw::serialize,
        value: LitStr,
    },
    ToString {
        kw: kw::to_string,
        value: LitStr,
    },
    Disabled(kw::disabled),
    Default(kw::default),
    Props {
        kw: kw::props,
        props: Vec<(LitStr, LitStr)>,
    },
}

impl Parse for VariantMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::message) {
            let kw = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value = input.parse()?;
            Ok(Self::Message { kw, value })
        } else if lookahead.peek(kw::detailed_message) {
            let kw = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value = input.parse()?;
            Ok(Self::DetailedMessage { kw, value })
        } else if lookahead.peek(kw::serialize) {
            let kw = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value = input.parse()?;
            Ok(Self::Serialize { kw, value })
        } else if lookahead.peek(kw::to_string) {
            let kw = input.parse()?;
            let _: Token![=] = input.parse()?;
            let value = input.parse()?;
            Ok(Self::ToString { kw, value })
        } else if lookahead.peek(kw::disabled) {
            Ok(Self::Disabled(input.parse()?))
        } else if lookahead.peek(kw::default) {
            Ok(Self::Default(input.parse()?))
        } else if lookahead.peek(kw::props) {
            let kw = input.parse()?;
            let content;
            parenthesized!(content in input);
            let props = content.parse_terminated::<_, Token![,]>(Prop::parse)?;
            Ok(Self::Props {
                kw,
                props: props
                    .into_iter()
                    .map(|Prop(k, v)| (LitStr::new(&k.to_string(), k.span()), v))
                    .collect(),
            })
        } else {
            Err(lookahead.error())
        }
    }
}

struct Prop(Ident, LitStr);

impl Parse for Prop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        use syn::ext::IdentExt;

        let k = Ident::parse_any(&input)?;
        let _: Token![=] = input.parse()?;
        let v = input.parse()?;

        Ok(Self(k, v))
    }
}

impl Spanned for VariantMeta {
    fn span(&self) -> Span {
        match self {
            Self::Message { kw, .. } => kw.span,
            Self::DetailedMessage { kw, .. } => kw.span,
            Self::Serialize { kw, .. } => kw.span,
            Self::ToString { kw, .. } => kw.span,
            Self::Disabled(kw) => kw.span,
            Self::Default(kw) => kw.span,
            Self::Props { kw, .. } => kw.span,
        }
    }
}

pub trait VariantExt {
    /// Get all the metadata associated with an enum variant.
    fn get_metadata(&self) -> syn::Result<Vec<VariantMeta>>;
}

impl VariantExt for Variant {
    fn get_metadata(&self) -> syn::Result<Vec<VariantMeta>> {
        get_metadata_inner("strum", &self.attrs)
    }
}

fn get_metadata_inner<'a, T: Parse + Spanned>(
    ident: &str,
    it: impl IntoIterator<Item = &'a Attribute>,
) -> syn::Result<Vec<T>> {
    it.into_iter()
        .filter(|attr| attr.path.is_ident(ident))
        .try_fold(Vec::new(), |mut vec, attr| {
            vec.extend(attr.parse_args_with(Punctuated::<T, Token![,]>::parse_terminated)?);
            Ok(vec)
        })
}
