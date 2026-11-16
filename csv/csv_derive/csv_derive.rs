//! Derive macros for the `csv` crate.
//!
//! `#[derive(Decode)]` and `#[derive(Encode)]` generate the native
//! `csv::Decode` and `csv::Encode` implementations.
//!
//! Supported container attributes:
//!
//! - `#[csv(rename_all = "...")]`
//! - `#[csv(bound = "...")]`
//! - `#[csv(crate = "...")]`
//!
//! Supported field attributes:
//!
//! - `#[csv(rename = "...")]`
//! - `#[csv(skip)]`

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, GenericParam, Generics, Ident, LitStr, Path, Token, TypeParamBound,
    WherePredicate, parse::Parser, parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};

/// Derive `csv::Decode`.
#[proc_macro_derive(Decode, attributes(csv))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_decode(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

/// Derive `csv::Encode`.
#[proc_macro_derive(Encode, attributes(csv))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_encode(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[derive(Default)]
struct ContainerAttrs {
    rename_all: Option<RenameRule>,
    krate: Option<Path>,
    bound: Option<String>,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    rename: Option<String>,
    skip: bool,
}

#[derive(Clone, Copy)]
enum RenameRule {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

impl RenameRule {
    fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "lowercase" => RenameRule::Lower,
            "UPPERCASE" => RenameRule::Upper,
            "PascalCase" => RenameRule::Pascal,
            "camelCase" => RenameRule::Camel,
            "snake_case" => RenameRule::Snake,
            "SCREAMING_SNAKE_CASE" => RenameRule::ScreamingSnake,
            "kebab-case" => RenameRule::Kebab,
            "SCREAMING-KEBAB-CASE" => RenameRule::ScreamingKebab,
            _ => return None,
        })
    }

    fn apply(self, name: &str) -> String {
        let words = split_words(name);
        match self {
            RenameRule::Lower => words.concat().to_lowercase(),
            RenameRule::Upper => words.concat().to_uppercase(),
            RenameRule::Pascal => words.iter().map(|word| capitalize(word)).collect(),
            RenameRule::Camel => {
                let mut iter = words.iter();
                let mut out = iter.next().map(|word| word.to_lowercase()).unwrap_or_default();
                for word in iter {
                    out.push_str(&capitalize(word));
                }
                out
            }
            RenameRule::Snake => words.join("_").to_lowercase(),
            RenameRule::ScreamingSnake => words.join("_").to_uppercase(),
            RenameRule::Kebab => words.join("-").to_lowercase(),
            RenameRule::ScreamingKebab => words.join("-").to_uppercase(),
        }
    }
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn split_words(name: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut previous_lower = false;
    for c in name.chars() {
        if c == '_' || c == '-' {
            if !current.is_empty() {
                words.push(core::mem::take(&mut current));
            }
            previous_lower = false;
        } else if c.is_uppercase() && previous_lower {
            if !current.is_empty() {
                words.push(core::mem::take(&mut current));
            }
            current.push(c);
            previous_lower = false;
        } else {
            current.push(c);
            previous_lower = c.is_lowercase() || c.is_ascii_digit();
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn parse_container(attrs: &[Attribute]) -> syn::Result<ContainerAttrs> {
    let mut out = ContainerAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("csv") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename_all = Some(
                    RenameRule::parse(&value.value())
                        .ok_or_else(|| syn::Error::new(value.span(), "unknown rename_all rule"))?,
                );
            } else if meta.path.is_ident("crate") {
                let value: LitStr = meta.value()?.parse()?;
                out.krate = Some(
                    syn::parse_str(&value.value())
                        .map_err(|e| syn::Error::new(value.span(), format!("invalid crate path: {e}")))?,
                );
            } else if meta.path.is_ident("bound") {
                let value: LitStr = meta.value()?.parse()?;
                out.bound = Some(value.value());
            } else {
                return Err(meta.error("unknown csv container attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_field(attrs: &[Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("csv") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename = Some(value.value());
            } else {
                return Err(meta.error("unknown csv field attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn named_fields(data: &Data) -> syn::Result<&syn::FieldsNamed> {
    match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => Ok(fields),
            Fields::Unnamed(fields) => Err(syn::Error::new(
                fields.span(),
                "csv derive currently supports only named structs",
            )),
            Fields::Unit => Err(syn::Error::new(
                data.struct_token.span(),
                "csv derive currently supports only named structs",
            )),
        },
        Data::Enum(data) => Err(syn::Error::new(
            data.enum_token.span(),
            "csv derive currently supports only named structs",
        )),
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span(),
            "csv derive currently supports only named structs",
        )),
    }
}

fn add_bounds(
    mut generics: Generics,
    bound: Option<&str>,
    krate: &Path,
    trait_name: TokenStream2,
) -> syn::Result<Generics> {
    if let Some(bound) = bound {
        let parser = Punctuated::<WherePredicate, Token![,]>::parse_terminated;
        let predicates = parser
            .parse_str(bound)
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("invalid bound: {e}")))?;
        generics.make_where_clause().predicates.extend(predicates);
    } else {
        for param in &mut generics.params {
            if let GenericParam::Type(param) = param {
                let bound: TypeParamBound = syn::parse_quote!(#krate::__private::#trait_name);
                param.bounds.push(bound);
            }
        }
    }
    Ok(generics)
}

fn expand_decode(input: DeriveInput) -> syn::Result<TokenStream2> {
    let fields = named_fields(&input.data)?;
    let container = parse_container(&input.attrs)?;
    let krate: Path = container.krate.clone().unwrap_or_else(|| syn::parse_quote!(::csv));
    let generics = add_bounds(input.generics.clone(), container.bound.as_deref(), &krate, quote!(DecodeField))?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = &input.ident;

    let initializers = fields
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field(&field.attrs)?;
            let ident = field
                .ident
                .as_ref()
                .ok_or_else(|| syn::Error::new(field.span(), "expected a named field"))?;
            if attrs.skip {
                return Ok(quote! {
                    #ident: ::core::default::Default::default(),
                });
            }
            let key = field_key(ident, &attrs, container.rename_all);
            Ok(quote! {
                #ident: __decoder.field(#key)?,
            })
        })
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics #krate::Decode for #name #ty_generics #where_clause {
            fn decode(
                __decoder: &#krate::__private::Decoder<'_>,
            ) -> ::core::result::Result<Self, #krate::ReadError> {
                ::core::result::Result::Ok(Self {
                    #(#initializers)*
                })
            }
        }
    })
}

fn expand_encode(input: DeriveInput) -> syn::Result<TokenStream2> {
    let fields = named_fields(&input.data)?;
    let container = parse_container(&input.attrs)?;
    let krate: Path = container.krate.clone().unwrap_or_else(|| syn::parse_quote!(::csv));
    let generics = add_bounds(input.generics.clone(), container.bound.as_deref(), &krate, quote!(EncodeField))?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = &input.ident;

    let entries = fields
        .named
        .iter()
        .map(|field| {
            let attrs = parse_field(&field.attrs)?;
            if attrs.skip {
                return Ok(None);
            }
            let ident = field
                .ident
                .as_ref()
                .ok_or_else(|| syn::Error::new(field.span(), "expected a named field"))?;
            let key = field_key(ident, &attrs, container.rename_all);
            Ok(Some(quote! {
                __encoder.field(#key, &self.#ident)?;
            }))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
            fn encode(
                &self,
                __encoder: &mut #krate::__private::Encoder<'_>,
            ) -> ::core::result::Result<(), #krate::WriteError> {
                #(#entries)*
                ::core::result::Result::Ok(())
            }
        }
    })
}

fn field_key(ident: &Ident, attrs: &FieldAttrs, rule: Option<RenameRule>) -> String {
    attrs
        .rename
        .clone()
        .unwrap_or_else(|| rule.map_or_else(|| ident.to_string(), |rule| rule.apply(&ident.to_string())))
}
