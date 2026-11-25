//! Derive macros for the `yaml` crate.
//!
//! `#[derive(Encode)]` generates `yaml::Encode` and `#[derive(Decode)]`
//! generates `yaml::Decode`. Structs are represented as string-keyed mappings,
//! tuple structs as sequences (or transparently when they have one field), and
//! enums externally tagged: unit variants become strings and data variants
//! become `{"Variant": payload}`.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, LitStr, Path, Type, parse_macro_input};

/// Derive `yaml::Encode`.
#[proc_macro_derive(Encode, attributes(yaml))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_encode(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Derive `yaml::Decode`.
#[proc_macro_derive(Decode, attributes(yaml))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_decode(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[derive(Default)]
struct Container {
    rename_all: Option<RenameRule>,
    transparent: bool,
}

#[derive(Default)]
struct Field {
    rename: Option<String>,
    skip: bool,
    default: bool,
    with: Option<Path>,
}

fn expand_encode(input: DeriveInput) -> syn::Result<TokenStream2> {
    let container = parse_container(&input.attrs)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let body = match &input.data {
        Data::Struct(data) => encode_struct(data, &container)?,
        Data::Enum(data) => encode_enum(data, &container, ident)?,
        Data::Union(_) => return Err(syn::Error::new_spanned(ident, "unions are not supported")),
    };
    Ok(quote! {
        impl #impl_generics ::yaml::Encode for #ident #ty_generics #where_clause {
            fn encode(&self) -> ::yaml::Value {
                #body
            }
        }
    })
}

fn encode_struct(data: &DataStruct, container: &Container) -> syn::Result<TokenStream2> {
    if container.transparent {
        let field = single_field(&data.fields)?;
        let attrs = parse_field(&field.attrs)?;
        let member = member(field, 0);
        return Ok(encode_value(&attrs, 0, quote! { self.#member }));
    }
    match &data.fields {
        Fields::Named(fields) => {
            let mut inserts = Vec::new();
            for field in &fields.named {
                let attrs = parse_field(&field.attrs)?;
                if attrs.skip {
                    continue;
                }
                let name = renamed(&attrs, field.ident.as_ref().expect("named field"), container);
                let member = member(field, 0);
                let value = encode_value(&attrs, 0, quote! { self.#member });
                inserts.push(quote! {
                    mapping.insert(::yaml::Value::String(::yaml::alloc::string::String::from(#name)), #value);
                });
            }
            Ok(quote! {
                let mut mapping = ::yaml::Mapping::new();
                #(#inserts)*
                ::yaml::Value::Mapping(mapping)
            })
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                let field = fields.unnamed.first().expect("one field");
                let attrs = parse_field(&field.attrs)?;
                return Ok(encode_value(&attrs, 0, quote! { self.0 }));
            }
            let values = fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let attrs = parse_field(&field.attrs)?;
                    let member = syn::Member::Unnamed(syn::Index::from(index));
                    Ok(encode_value(&attrs, index, quote! { self.#member }))
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! {
                ::yaml::Value::Sequence(::yaml::alloc::vec![#(#values),*])
            })
        }
        Fields::Unit => Ok(quote! { ::yaml::Value::Null }),
    }
}

fn encode_enum(data: &DataEnum, container: &Container, ident: &Ident) -> syn::Result<TokenStream2> {
    let mut arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_field(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let name = renamed(&attrs, &variant.ident, container);
        let variant_ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => arms.push(quote! {
                #ident::#variant_ident => ::yaml::Value::String(::yaml::alloc::string::String::from(#name))
            }),
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field = fields.unnamed.first().expect("one field");
                let field_attrs = parse_field(&field.attrs)?;
                let value = encode_value(&field_attrs, 0, quote! { inner });
                arms.push(quote! {
                    #ident::#variant_ident(inner) => {
                        let mut mapping = ::yaml::Mapping::new();
                        mapping.insert(
                            ::yaml::Value::String(::yaml::alloc::string::String::from(#name)),
                            #value,
                        );
                        ::yaml::Value::Mapping(mapping)
                    }
                });
            }
            Fields::Unnamed(fields) => {
                let bindings = (0..fields.unnamed.len())
                    .map(|index| quote::format_ident!("inner_{index}"))
                    .collect::<Vec<_>>();
                let values = fields
                    .unnamed
                    .iter()
                    .zip(&bindings)
                    .map(|(field, binding)| {
                        let attrs = parse_field(&field.attrs)?;
                        Ok(encode_value(&attrs, 0, quote! { #binding }))
                    })
                    .collect::<syn::Result<Vec<_>>>()?;
                arms.push(quote! {
                    #ident::#variant_ident(#(#bindings),*) => {
                        let payload = ::yaml::Value::Sequence(::yaml::alloc::vec![#(#values),*]);
                        let mut mapping = ::yaml::Mapping::new();
                        mapping.insert(
                            ::yaml::Value::String(::yaml::alloc::string::String::from(#name)),
                            payload,
                        );
                        ::yaml::Value::Mapping(mapping)
                    }
                });
            }
            Fields::Named(fields) => {
                let bindings = fields
                    .named
                    .iter()
                    .map(|field| field.ident.clone().expect("named field"))
                    .collect::<Vec<_>>();
                let mut inserts = Vec::new();
                for field in &fields.named {
                    let attrs = parse_field(&field.attrs)?;
                    if attrs.skip {
                        continue;
                    }
                    let binding = field.ident.as_ref().expect("named field");
                    let field_name = renamed(&attrs, binding, container);
                    let value = encode_value(&attrs, 0, quote! { #binding });
                    inserts.push(quote! {
                        inner.insert(
                            ::yaml::Value::String(::yaml::alloc::string::String::from(#field_name)),
                            #value,
                        );
                    });
                }
                arms.push(quote! {
                    #ident::#variant_ident { #(#bindings),* } => {
                        let mut inner = ::yaml::Mapping::new();
                        #(#inserts)*
                        let mut mapping = ::yaml::Mapping::new();
                        mapping.insert(
                            ::yaml::Value::String(::yaml::alloc::string::String::from(#name)),
                            ::yaml::Value::Mapping(inner),
                        );
                        ::yaml::Value::Mapping(mapping)
                    }
                });
            }
        }
    }
    Ok(quote! {
        match self {
            #(#arms),*
        }
    })
}

fn expand_decode(input: DeriveInput) -> syn::Result<TokenStream2> {
    let container = parse_container(&input.attrs)?;
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let body = match &input.data {
        Data::Struct(data) => decode_struct(data, &container)?,
        Data::Enum(data) => decode_enum(data, &container, ident)?,
        Data::Union(_) => return Err(syn::Error::new_spanned(ident, "unions are not supported")),
    };
    Ok(quote! {
        impl #impl_generics ::yaml::Decode for #ident #ty_generics #where_clause {
            fn decode(value: &::yaml::Value) -> ::yaml::Result<Self> {
                #body
            }
        }
    })
}

fn decode_struct(data: &DataStruct, container: &Container) -> syn::Result<TokenStream2> {
    if container.transparent {
        let field = single_field(&data.fields)?;
        let attrs = parse_field(&field.attrs)?;
        let ty = &field.ty;
        return Ok(decode_value(&attrs, ty, quote! { value }));
    }
    match &data.fields {
        Fields::Named(fields) => {
            let mut initializers = Vec::new();
            for field in &fields.named {
                let attrs = parse_field(&field.attrs)?;
                let ident = field.ident.clone().expect("named field");
                let ty = &field.ty;
                let name = renamed(&attrs, &ident, container);
                let getter = quote! { value.untag().get(#name) };
                let decode_expr = decode_value(&attrs, ty, quote! { field });
                let decode = if attrs.skip || attrs.default {
                    quote! {
                        match #getter {
                            Some(field) => #decode_expr,
                            None => ::core::default::Default::default(),
                        }
                    }
                } else {
                    quote! {
                        match #getter {
                            Some(field) => #decode_expr,
                            None => return Err(::yaml::Error::missing_field(#name)),
                        }
                    }
                };
                initializers.push(quote! { #ident: #decode });
            }
            Ok(quote! {
                Ok(Self { #(#initializers),* })
            })
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                let field = fields.unnamed.first().expect("one field");
                let attrs = parse_field(&field.attrs)?;
                let ty = &field.ty;
                let decode = decode_value(&attrs, ty, quote! { value });
                return Ok(quote! { Ok(Self(#decode)) });
            }
            let mut elements = Vec::new();
            let mut bindings = Vec::new();
            for (index, field) in fields.unnamed.iter().enumerate() {
                let attrs = parse_field(&field.attrs)?;
                let ty = &field.ty;
                let binding = quote::format_ident!("field_{index}");
                let decode = decode_value(&attrs, ty, quote! { element });
                elements.push(quote! {
                    let element = sequence.get(#index).ok_or_else(|| {
                        ::yaml::Error::new("sequence has too few elements")
                    })?;
                    let #binding = #decode;
                });
                bindings.push(binding);
            }
            Ok(quote! {
                let sequence = value.untag().as_sequence().ok_or_else(|| {
                    ::yaml::Error::invalid_type("sequence", value)
                })?;
                #(#elements)*
                Ok(Self(#(#bindings),*))
            })
        }
        Fields::Unit => Ok(quote! {
            match value.untag() {
                ::yaml::Value::Null => Ok(Self),
                other => Err(::yaml::Error::invalid_type("null", other)),
            }
        }),
    }
}

fn decode_enum(data: &DataEnum, container: &Container, ident: &Ident) -> syn::Result<TokenStream2> {
    let mut string_arms = Vec::new();
    let mut map_arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_field(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let name = renamed(&attrs, &variant.ident, container);
        let variant_ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => string_arms.push(quote! {
                #name => Ok(#ident::#variant_ident)
            }),
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let field = fields.unnamed.first().expect("one field");
                let field_attrs = parse_field(&field.attrs)?;
                let decode = decode_value(&field_attrs, &field.ty, quote! { payload });
                map_arms.push(quote! {
                    #name => Ok(#ident::#variant_ident(#decode))
                });
            }
            Fields::Unnamed(fields) => {
                let mut elements = Vec::new();
                let mut bindings = Vec::new();
                for (index, field) in fields.unnamed.iter().enumerate() {
                    let field_attrs = parse_field(&field.attrs)?;
                    let binding = quote::format_ident!("field_{index}");
                    let decode = decode_value(&field_attrs, &field.ty, quote! { element });
                    elements.push(quote! {
                        let element = sequence.get(#index).ok_or_else(|| {
                            ::yaml::Error::new("variant sequence has too few elements")
                        })?;
                        let #binding = #decode;
                    });
                    bindings.push(binding);
                }
                map_arms.push(quote! {
                    #name => {
                        let sequence = payload.untag().as_sequence().ok_or_else(|| {
                            ::yaml::Error::invalid_type("sequence", payload)
                        })?;
                        #(#elements)*
                        Ok(#ident::#variant_ident(#(#bindings),*))
                    }
                });
            }
            Fields::Named(fields) => {
                let mut initializers = Vec::new();
                for field in &fields.named {
                    let field_attrs = parse_field(&field.attrs)?;
                    let field_ident = field.ident.clone().expect("named field");
                    let field_name = renamed(&field_attrs, &field_ident, container);
                    let decode = decode_value(&field_attrs, &field.ty, quote! { field });
                    initializers.push(quote! {
                        #field_ident: match payload.untag().get(#field_name) {
                            Some(field) => #decode,
                            None => ::core::default::Default::default(),
                        }
                    });
                }
                map_arms.push(quote! {
                    #name => Ok(#ident::#variant_ident { #(#initializers),* })
                });
            }
        }
    }

    Ok(quote! {
        if let Some(name) = value.untag().as_str() {
            match name {
                #(#string_arms,)*
                other => Err(::yaml::Error::new(::yaml::alloc::format!("unknown variant `{other}`"))),
            }
        } else if let Some(mapping) = value.untag().as_mapping() {
            let Some((key, payload)) = mapping.iter().next() else {
                return Err(::yaml::Error::new("empty tagged mapping"));
            };
            let name = key.as_str().ok_or_else(|| ::yaml::Error::new("variant tag must be a string"))?;
            match name {
                #(#map_arms,)*
                other => Err(::yaml::Error::new(::yaml::alloc::format!("unknown variant `{other}`"))),
            }
        } else {
            Err(::yaml::Error::new("expected enum variant name or mapping"))
        }
    })
}

fn encode_value(attrs: &Field, _index: usize, value: TokenStream2) -> TokenStream2 {
    match &attrs.with {
        Some(path) => quote! { #path::encode(&#value) },
        None => quote! { ::yaml::Encode::encode(&#value) },
    }
}

fn decode_value(attrs: &Field, _ty: &Type, value: TokenStream2) -> TokenStream2 {
    match &attrs.with {
        Some(path) => quote! { #path::decode(#value)? },
        None => quote! { ::yaml::Decode::decode(#value)? },
    }
}

fn single_field(fields: &Fields) -> syn::Result<&syn::Field> {
    match fields {
        Fields::Named(fields) if fields.named.len() == 1 => Ok(fields.named.first().expect("one field")),
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(fields.unnamed.first().expect("one field")),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[yaml(transparent)] requires a struct with exactly one field",
        )),
    }
}

fn member(field: &syn::Field, index: usize) -> syn::Member {
    match &field.ident {
        Some(ident) => syn::Member::Named(ident.clone()),
        None => syn::Member::Unnamed(index.into()),
    }
}

fn renamed(attrs: &Field, ident: &Ident, container: &Container) -> String {
    if let Some(rename) = &attrs.rename {
        return rename.clone();
    }
    let raw = ident.to_string();
    match container.rename_all {
        Some(rule) => apply_rule(rule, &raw),
        None => raw,
    }
}

fn parse_container(attrs: &[Attribute]) -> syn::Result<Container> {
    let mut container = Container::default();
    for attr in attrs {
        if !attr.path().is_ident("yaml") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("transparent") {
                container.transparent = true;
                return Ok(());
            }
            if meta.path.is_ident("rename_all") {
                let value: LitStr = meta.value()?.parse()?;
                container.rename_all = Some(parse_rule(&value)?);
                return Ok(());
            }
            Err(meta.error("unknown yaml container attribute"))
        })?;
    }
    Ok(container)
}

fn parse_field(attrs: &[Attribute]) -> syn::Result<Field> {
    let mut field = Field::default();
    for attr in attrs {
        if !attr.path().is_ident("yaml") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                field.skip = true;
                return Ok(());
            }
            if meta.path.is_ident("default") {
                field.default = true;
                return Ok(());
            }
            if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                field.rename = Some(value.value());
                return Ok(());
            }
            if meta.path.is_ident("with") {
                let value: LitStr = meta.value()?.parse()?;
                field.with = Some(syn::parse_str(&value.value())?);
                return Ok(());
            }
            Err(meta.error("unknown yaml field attribute"))
        })?;
    }
    Ok(field)
}

#[derive(Clone, Copy)]
enum RenameRule {
    Lower,
    Upper,
    Snake,
    Kebab,
    Camel,
    Pascal,
    ScreamingSnake,
}

fn parse_rule(value: &LitStr) -> syn::Result<RenameRule> {
    match value.value().as_str() {
        "lowercase" => Ok(RenameRule::Lower),
        "UPPERCASE" => Ok(RenameRule::Upper),
        "snake_case" => Ok(RenameRule::Snake),
        "kebab-case" => Ok(RenameRule::Kebab),
        "camelCase" => Ok(RenameRule::Camel),
        "PascalCase" => Ok(RenameRule::Pascal),
        "SCREAMING_SNAKE_CASE" => Ok(RenameRule::ScreamingSnake),
        other => Err(syn::Error::new(value.span(), format!("unknown rename_all rule `{other}`"))),
    }
}

fn apply_rule(rule: RenameRule, name: &str) -> String {
    let words = words(name);
    match rule {
        RenameRule::Lower => name.to_lowercase(),
        RenameRule::Upper => name.to_uppercase(),
        RenameRule::Snake => words.join("_").to_lowercase(),
        RenameRule::Kebab => words.join("-").to_lowercase(),
        RenameRule::ScreamingSnake => words.join("_").to_uppercase(),
        RenameRule::Camel => {
            let mut words = words.into_iter();
            let first = words.next().unwrap_or_default().to_lowercase();
            let rest = words.map(capitalize).collect::<String>();
            first + &rest
        }
        RenameRule::Pascal => words.into_iter().map(capitalize).collect(),
    }
}

fn words(name: &str) -> Vec<&str> {
    let mut words = Vec::new();
    let mut start = 0;
    let bytes = name.as_bytes();
    for index in 1..bytes.len() {
        let previous = bytes[index - 1];
        let current = bytes[index];
        if current == b'_' || current == b'-' {
            if start < index {
                words.push(&name[start..index]);
            }
            start = index + 1;
        } else if current.is_ascii_uppercase() && previous.is_ascii_lowercase() {
            words.push(&name[start..index]);
            start = index;
        }
    }
    if start < name.len() {
        words.push(&name[start..]);
    }
    if words.is_empty() {
        words.push(name);
    }
    words
}

fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}
