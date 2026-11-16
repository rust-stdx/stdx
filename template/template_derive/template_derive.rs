//! Derive macros for the `template` crate.
//!
//! `#[derive(ToValue)]` generates a `template::ToValue` implementation for a
//! struct or enum, allowing it to be passed directly to `Engine::render`.
//!
//! Named structs become maps, tuple structs become arrays, and unit structs
//! become `null`. Enums map unit variants to strings, newtype variants to their
//! inner value, tuple variants to arrays, and struct variants to maps.
//!
//! Supported container attributes:
//!
//! - `#[template(rename_all = "...")]`
//! - `#[template(bound = "...")]`
//! - `#[template(crate = "...")]`
//!
//! Supported field and variant attributes:
//!
//! - `#[template(rename = "...")]`
//! - `#[template(skip)]`

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, GenericParam, Generics, Ident, LitStr, Path,
    Token, TypeParamBound, WherePredicate, parse::Parser, parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};

/// Derive `template::ToValue`.
///
/// See the crate documentation for the supported container, field, and variant
/// attributes.
#[proc_macro_derive(ToValue, attributes(template))]
pub fn derive_to_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input).unwrap_or_else(|error| error.to_compile_error()).into()
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
            RenameRule::Pascal => words.iter().map(|w| capitalize(w)).collect(),
            RenameRule::Camel => {
                let mut iter = words.iter();
                let mut out = iter.next().map(|w| w.to_lowercase()).unwrap_or_default();
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
        if !attr.path().is_ident("template") {
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
                return Err(meta.error("unknown template container attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_field(attrs: &[Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("template") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename = Some(value.value());
            } else {
                return Err(meta.error("unknown template field attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let container = parse_container(&input.attrs)?;
    let krate: Path = container.krate.clone().unwrap_or_else(|| syn::parse_quote!(::template));
    let generics = add_bounds(input.generics.clone(), container.bound.as_deref(), &krate)?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = &input.ident;

    let body = match &input.data {
        Data::Struct(data) => expand_struct(data, &container, &krate)?,
        Data::Enum(data) => expand_enum(name, data, &container, &krate)?,
        Data::Union(data) => {
            return Err(syn::Error::new(data.union_token.span(), "ToValue cannot be derived for unions"));
        }
    };

    Ok(quote! {
        impl #impl_generics #krate::ToValue for #name #ty_generics #where_clause {
            fn to_value(&self) -> #krate::__macro_support::Value {
                #body
            }
        }
    })
}

fn add_bounds(mut generics: Generics, bound: Option<&str>, krate: &Path) -> syn::Result<Generics> {
    if let Some(bound) = bound {
        let parser = Punctuated::<WherePredicate, Token![,]>::parse_terminated;
        let predicates = parser
            .parse_str(bound)
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("invalid bound: {e}")))?;
        generics.make_where_clause().predicates.extend(predicates);
    } else {
        for param in &mut generics.params {
            if let GenericParam::Type(param) = param {
                let bound: TypeParamBound = syn::parse_quote!(#krate::ToValue);
                param.bounds.push(bound);
            }
        }
    }
    Ok(generics)
}

fn expand_struct(data: &DataStruct, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    match &data.fields {
        Fields::Named(fields) => {
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
                        __map.insert(
                            #krate::__macro_support::String::from(#key),
                            #krate::ToValue::to_value(&self.#ident),
                        );
                    }))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! {
                let mut __map = #krate::__macro_support::BTreeMap::new();
                #(#entries)*
                #krate::__macro_support::map_value(__map)
            })
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                let field = &fields.unnamed[0];
                let attrs = parse_field(&field.attrs)?;
                if attrs.skip {
                    return Ok(quote!(#krate::__macro_support::Value::Null));
                }
                return Ok(quote!(#krate::ToValue::to_value(&self.0)));
            }
            let values = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| field_value(field, index, krate))
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! {
                #krate::__macro_support::array_value(
                    #krate::__macro_support::Vec::from([#(#values),*])
                )
            })
        }
        Fields::Unit => Ok(quote!(#krate::__macro_support::Value::Null)),
    }
}

fn expand_enum(
    enum_ident: &Ident,
    data: &DataEnum,
    container: &ContainerAttrs,
    krate: &Path,
) -> syn::Result<TokenStream2> {
    let arms = data
        .variants
        .iter()
        .map(|variant| {
            let attrs = parse_field(&variant.attrs)?;
            if attrs.skip {
                return Ok(None);
            }
            let variant_ident = &variant.ident;
            let variant_name = attrs.rename.clone().unwrap_or_else(|| {
                container
                    .rename_all
                    .map_or_else(|| variant_ident.to_string(), |rule| rule.apply(&variant_ident.to_string()))
            });

            let body = match &variant.fields {
                Fields::Named(fields) => {
                    let bindings = fields
                        .named
                        .iter()
                        .map(|field| {
                            field
                                .ident
                                .clone()
                                .ok_or_else(|| syn::Error::new(field.span(), "expected a named field"))
                        })
                        .collect::<syn::Result<Vec<_>>>()?;
                    let entries = fields
                        .named
                        .iter()
                        .zip(&bindings)
                        .map(|(field, binding)| {
                            let attrs = parse_field(&field.attrs)?;
                            if attrs.skip {
                                return Ok(None);
                            }
                            let key = field_key(binding, &attrs, None);
                            Ok(Some(quote! {
                                __map.insert(
                                    #krate::__macro_support::String::from(#key),
                                    #krate::ToValue::to_value(#binding),
                                );
                            }))
                        })
                        .collect::<syn::Result<Vec<_>>>()?;
                    quote! {
                        let mut __map = #krate::__macro_support::BTreeMap::new();
                        #(#entries)*
                        #krate::__macro_support::map_value(__map)
                    }
                }
                Fields::Unnamed(fields) => {
                    let bindings = (0..fields.unnamed.len())
                        .map(|index| format_ident!("__field_{index}"))
                        .collect::<Vec<_>>();
                    if bindings.len() == 1 {
                        let binding = &bindings[0];
                        quote!(#krate::ToValue::to_value(#binding))
                    } else {
                        let values = bindings
                            .iter()
                            .map(|binding| quote!(#krate::ToValue::to_value(#binding)));
                        quote! {
                            #krate::__macro_support::array_value(
                                #krate::__macro_support::Vec::from([#(#values),*])
                            )
                        }
                    }
                }
                Fields::Unit => quote!(#krate::__macro_support::str_value(#variant_name)),
            };

            let pattern = match &variant.fields {
                Fields::Named(fields) => {
                    let bindings = fields
                        .named
                        .iter()
                        .map(|field| {
                            field
                                .ident
                                .clone()
                                .ok_or_else(|| syn::Error::new(field.span(), "expected a named field"))
                        })
                        .collect::<syn::Result<Vec<_>>>()?;
                    quote!(#enum_ident::#variant_ident { #(#bindings),* })
                }
                Fields::Unnamed(fields) => {
                    let bindings = (0..fields.unnamed.len())
                        .map(|index| format_ident!("__field_{index}"))
                        .collect::<Vec<_>>();
                    quote!(#enum_ident::#variant_ident ( #(#bindings),* ))
                }
                Fields::Unit => quote!(#enum_ident::#variant_ident),
            };
            Ok(Some(quote! {
                #pattern => { #body }
            }))
        })
        .collect::<syn::Result<Vec<_>>>()?;

    Ok(quote! {
        match self {
            #(#arms),*
        }
    })
}

fn field_key(ident: &Ident, attrs: &FieldAttrs, rule: Option<RenameRule>) -> String {
    attrs
        .rename
        .clone()
        .unwrap_or_else(|| rule.map_or_else(|| ident.to_string(), |rule| rule.apply(&ident.to_string())))
}

fn field_value(field: &Field, index: usize, krate: &Path) -> syn::Result<TokenStream2> {
    let attrs = parse_field(&field.attrs)?;
    if attrs.skip {
        Ok(quote!(#krate::__macro_support::Value::Null))
    } else {
        let member = syn::Member::Unnamed(syn::Index {
            index: index as u32,
            span: field.span(),
        });
        Ok(quote!(#krate::ToValue::to_value(&self.#member)))
    }
}
