//! Derive macros for the `json` crate.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{Attribute, Data, DataEnum, DeriveInput, Field, Fields, GenericParam, LitStr, Path, Token, Type};

/// Derive `json::Encode`.
#[proc_macro_derive(Encode, attributes(json))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    match expand(input.into(), Direction::Encode) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Derive `json::Decode`.
#[proc_macro_derive(Decode, attributes(json))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    match expand(input.into(), Direction::Decode) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Encode,
    Decode,
}

#[derive(Default)]
struct ContainerAttrs {
    rename_all: Option<RenameRule>,
    deny_unknown_fields: bool,
    deny_duplicate_fields: bool,
    transparent: bool,
    tag: Option<String>,
    content: Option<String>,
    untagged: bool,
    krate: Option<Path>,
    bound: Option<String>,
    encode_only: bool,
    decode_only: bool,
}

#[derive(Default)]
struct FieldAttrs {
    rename: Option<String>,
    aliases: Vec<String>,
    skip: bool,
    default: Option<Option<Path>>,
    skip_if: Option<Path>,
    with: Option<Path>,
    encode_with: Option<Path>,
    decode_with: Option<Path>,
}

#[derive(Default)]
struct VariantAttrs {
    rename: Option<String>,
    skip: bool,
    other: bool,
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
    for character in name.chars() {
        if character == '_' || character == '-' {
            if !current.is_empty() {
                words.push(core::mem::take(&mut current));
            }
            previous_lower = false;
        } else if character.is_uppercase() && previous_lower {
            if !current.is_empty() {
                words.push(core::mem::take(&mut current));
            }
            current.push(character);
            previous_lower = false;
        } else {
            current.push(character);
            previous_lower = character.is_lowercase() || character.is_ascii_digit();
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
        if !attr.path().is_ident("json") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("deny_unknown_fields") {
                out.deny_unknown_fields = true;
            } else if meta.path.is_ident("deny_duplicate_fields") {
                out.deny_duplicate_fields = true;
            } else if meta.path.is_ident("transparent") {
                out.transparent = true;
            } else if meta.path.is_ident("untagged") {
                out.untagged = true;
            } else if meta.path.is_ident("encode_only") {
                out.encode_only = true;
            } else if meta.path.is_ident("decode_only") {
                out.decode_only = true;
            } else if meta.path.is_ident("rename_all") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename_all = Some(
                    RenameRule::parse(&value.value())
                        .ok_or_else(|| syn::Error::new(value.span(), "unknown rename_all rule"))?,
                );
            } else if meta.path.is_ident("tag") {
                let value: LitStr = meta.value()?.parse()?;
                out.tag = Some(value.value());
            } else if meta.path.is_ident("content") {
                let value: LitStr = meta.value()?.parse()?;
                out.content = Some(value.value());
            } else if meta.path.is_ident("crate") {
                let value: LitStr = meta.value()?.parse()?;
                out.krate = Some(
                    syn::parse_str(&value.value())
                        .map_err(|error| syn::Error::new(value.span(), format!("invalid crate path: {error}")))?,
                );
            } else if meta.path.is_ident("bound") {
                let value: LitStr = meta.value()?.parse()?;
                out.bound = Some(value.value());
            } else {
                return Err(meta.error("unknown json container attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_field(attrs: &[Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("json") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("borrow") {
                // Decoding `&'a str` already borrows; accepted for symmetry.
            } else if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename = Some(value.value());
            } else if meta.path.is_ident("alias") {
                let value: LitStr = meta.value()?.parse()?;
                out.aliases.push(value.value());
            } else if meta.path.is_ident("default") {
                if meta.input.peek(Token![=]) {
                    let value: LitStr = meta.value()?.parse()?;
                    out.default =
                        Some(Some(syn::parse_str(&value.value()).map_err(|error| {
                            syn::Error::new(value.span(), format!("invalid default path: {error}"))
                        })?));
                } else {
                    out.default = Some(None);
                }
            } else if meta.path.is_ident("skip_serializing_if") {
                let value: LitStr = meta.value()?.parse()?;
                out.skip_if = Some(syn::parse_str(&value.value()).map_err(|error| {
                    syn::Error::new(value.span(), format!("invalid skip_serializing_if path: {error}"))
                })?);
            } else if meta.path.is_ident("with") {
                let value: LitStr = meta.value()?.parse()?;
                out.with = Some(
                    syn::parse_str(&value.value())
                        .map_err(|error| syn::Error::new(value.span(), format!("invalid with path: {error}")))?,
                );
            } else if meta.path.is_ident("encode_with") {
                let value: LitStr = meta.value()?.parse()?;
                out.encode_with =
                    Some(syn::parse_str(&value.value()).map_err(|error| {
                        syn::Error::new(value.span(), format!("invalid encode_with path: {error}"))
                    })?);
            } else if meta.path.is_ident("decode_with") {
                let value: LitStr = meta.value()?.parse()?;
                out.decode_with =
                    Some(syn::parse_str(&value.value()).map_err(|error| {
                        syn::Error::new(value.span(), format!("invalid decode_with path: {error}"))
                    })?);
            } else if meta.path.is_ident("raw") {
                // Raw fields are ordinary `RawValue`/`RawValueBuf` fields; no
                // extra code generation is needed.
            } else if meta.path.is_ident("flatten") {
                return Err(meta.error("json(flatten) is not implemented yet"));
            } else {
                return Err(meta.error("unknown json field attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_variant(attrs: &[Attribute]) -> syn::Result<VariantAttrs> {
    let mut out = VariantAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("json") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("other") {
                out.other = true;
            } else if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename = Some(value.value());
            } else if meta.path.is_ident("alias") {
                let _: LitStr = meta.value()?.parse()?;
            } else {
                return Err(meta.error("unknown json variant attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn expand(input: TokenStream2, direction: Direction) -> syn::Result<TokenStream2> {
    let input: DeriveInput = syn::parse2(input)?;
    let container = parse_container(&input.attrs)?;
    if direction == Direction::Encode && container.decode_only {
        return Ok(TokenStream2::new());
    }
    if direction == Direction::Decode && container.encode_only {
        return Ok(TokenStream2::new());
    }

    let krate = container.krate.clone().unwrap_or_else(|| syn::parse_quote!(::json));
    match direction {
        Direction::Encode => expand_encode(&input, &container, &krate),
        Direction::Decode => expand_decode(&input, &container, &krate),
    }
}

fn member(field: &Field, index: usize) -> syn::Member {
    match &field.ident {
        Some(ident) => syn::Member::Named(ident.clone()),
        None => syn::Member::Unnamed(syn::Index::from(index)),
    }
}

fn field_key(field: &Field, index: usize, attrs: &FieldAttrs, container: &ContainerAttrs) -> String {
    if let Some(rename) = &attrs.rename {
        return rename.clone();
    }
    let original = field
        .ident
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| index.to_string());
    match container.rename_all {
        Some(rule) => rule.apply(&original),
        None => original,
    }
}

fn variant_name(variant: &syn::Variant, attrs: &VariantAttrs, container: &ContainerAttrs) -> String {
    if let Some(rename) = &attrs.rename {
        return rename.clone();
    }
    match container.rename_all {
        Some(rule) => rule.apply(&variant.ident.to_string()),
        None => variant.ident.to_string(),
    }
}

fn encode_generics(input: &DeriveInput, container: &ContainerAttrs, krate: &Path) -> syn::Generics {
    let mut generics = input.generics.clone();
    if container.bound.is_none() {
        for param in &mut generics.params {
            if let GenericParam::Type(param) = param {
                param.bounds.push(syn::parse_quote!(#krate::Encode));
            }
        }
    }
    generics
}

fn decode_generics(input: &DeriveInput, container: &ContainerAttrs, krate: &Path) -> (syn::Generics, syn::Lifetime) {
    let lifetime = input
        .generics
        .lifetimes()
        .next()
        .map(|lt| lt.lifetime.clone())
        .unwrap_or_else(|| syn::parse_quote!('json));
    let mut generics = input.generics.clone();
    if input.generics.lifetimes().next().is_none() {
        generics
            .params
            .insert(0, GenericParam::Lifetime(syn::LifetimeParam::new(lifetime.clone())));
    }
    if container.bound.is_none() {
        for param in &mut generics.params {
            if let GenericParam::Type(param) = param {
                param.bounds.push(syn::parse_quote!(#krate::Decode<#lifetime>));
            }
        }
    }
    (generics, lifetime)
}

fn expand_encode(input: &DeriveInput, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let generics = encode_generics(input, container, krate);
    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let where_clause = container
        .bound
        .as_ref()
        .map(|bound| -> syn::WhereClause { syn::parse_str(bound).expect("valid bound") })
        .or_else(|| generics.where_clause.clone());

    let body = match &input.data {
        Data::Struct(data) => encode_struct(&data.fields, container, krate)?,
        Data::Enum(data) => encode_enum(data, container, krate)?,
        Data::Union(_) => return Err(syn::Error::new_spanned(input, "unions are not supported")),
    };

    Ok(quote! {
        impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
            fn encode<W: #krate::Write>(
                &self,
                encoder: &mut #krate::Encoder<W>,
            ) -> ::core::result::Result<(), W::Error> {
                #body
            }
        }
    })
}

fn expand_decode(input: &DeriveInput, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let (generics, lifetime) = decode_generics(input, container, krate);
    let (impl_generics, _, _) = generics.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let where_clause = container
        .bound
        .as_ref()
        .map(|bound| -> syn::WhereClause { syn::parse_str(bound).expect("valid bound") })
        .or_else(|| generics.where_clause.clone());

    let body = match &input.data {
        Data::Struct(data) => decode_struct(&data.fields, container, krate)?,
        Data::Enum(data) => decode_enum(data, container, krate, &lifetime)?,
        Data::Union(_) => return Err(syn::Error::new_spanned(input, "unions are not supported")),
    };

    Ok(quote! {
        impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
            fn decode(decoder: &mut #krate::Decoder<#lifetime>) -> #krate::Result<Self> {
                #body
            }
        }
    })
}

fn encode_struct(fields: &Fields, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    if container.transparent {
        let field = single_field(fields)?;
        let attrs = parse_field(&field.attrs)?;
        let index = 0;
        let member = member(&field, index);
        let encode = encode_field_expr(&attrs, &quote!(&self.#member), krate);
        return Ok(quote! { #encode; Ok(()) });
    }

    match fields {
        Fields::Named(fields) => {
            let body = encode_named_fields(fields, container, krate, |field, index| {
                let member = member(field, index);
                quote!(&self.#member)
            })?;
            Ok(quote! { #body })
        }
        Fields::Unnamed(fields) => {
            let body = encode_tuple_fields(fields, container, krate, |field, index| {
                let member = member(field, index);
                quote!(&self.#member)
            })?;
            Ok(quote! { #body })
        }
        Fields::Unit => Ok(quote! { encoder.null() }),
    }
}

fn encode_named_fields<F>(
    fields: &syn::FieldsNamed,
    container: &ContainerAttrs,
    krate: &Path,
    access: F,
) -> syn::Result<TokenStream2>
where
    F: Fn(&Field, usize) -> TokenStream2,
{
    let mut statements = Vec::new();
    for (index, field) in fields.named.iter().enumerate() {
        let attrs = parse_field(&field.attrs)?;
        if attrs.skip {
            continue;
        }
        let key = field_key(field, index, &attrs, container);
        let value = access(field, index);
        let encode = encode_field_expr(&attrs, &value, krate);
        if let Some(path) = &attrs.skip_if {
            statements.push(quote! {
                if !#path(#value) {
                    encoder.comma(first)?;
                    first = false;
                    encoder.key(#key)?;
                    #encode;
                }
            });
        } else {
            statements.push(quote! {
                encoder.comma(first)?;
                first = false;
                encoder.key(#key)?;
                #encode;
            });
        }
    }
    Ok(quote! {
        encoder.begin_object()?;
        let mut first = true;
        #(#statements)*
        encoder.end_object()
    })
}

fn encode_tuple_fields<F>(
    fields: &syn::FieldsUnnamed,
    container: &ContainerAttrs,
    krate: &Path,
    access: F,
) -> syn::Result<TokenStream2>
where
    F: Fn(&Field, usize) -> TokenStream2,
{
    let _ = container;
    let mut statements = Vec::new();
    for (index, field) in fields.unnamed.iter().enumerate() {
        let attrs = parse_field(&field.attrs)?;
        if attrs.skip {
            continue;
        }
        let value = access(field, index);
        let encode = encode_field_expr(&attrs, &value, krate);
        statements.push(quote! {
            encoder.comma(first)?;
            first = false;
            #encode;
        });
    }
    Ok(quote! {
        encoder.begin_array()?;
        let mut first = true;
        #(#statements)*
        encoder.end_array()
    })
}

fn single_field(fields: &Fields) -> syn::Result<Field> {
    let mut iter = match fields {
        Fields::Named(fields) => fields.named.iter().cloned().collect::<Vec<_>>(),
        Fields::Unnamed(fields) => fields.unnamed.iter().cloned().collect::<Vec<_>>(),
        Fields::Unit => Vec::new(),
    };
    if iter.len() != 1 {
        return Err(syn::Error::new(Span::call_site(), "transparent requires exactly one field"));
    }
    Ok(iter.pop().unwrap())
}

fn encode_field_expr(attrs: &FieldAttrs, value: &TokenStream2, krate: &Path) -> TokenStream2 {
    if let Some(path) = &attrs.encode_with {
        return quote! { #path(#value, encoder)? };
    }
    if let Some(path) = &attrs.with {
        return quote! { #path::serialize(#value, encoder)? };
    }
    quote! { #krate::Encode::encode(#value, encoder)? }
}

fn decode_struct(fields: &Fields, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    if container.transparent {
        let field = single_field(fields)?;
        let attrs = parse_field(&field.attrs)?;
        let ty = &field.ty;
        let decode = decode_expr(&attrs, ty, krate);
        return match &field.ident {
            Some(ident) => Ok(quote! { Ok(Self { #ident: #decode }) }),
            None => Ok(quote! { Ok(Self(#decode)) }),
        };
    }

    match fields {
        Fields::Named(_) => {
            let expr = decode_named_fields(fields, container, krate, quote!(Self), true)?;
            Ok(quote! { Ok(#expr) })
        }
        Fields::Unnamed(_) => {
            let expr = decode_tuple_fields(fields, container, krate, quote!(Self))?;
            Ok(quote! { Ok(#expr) })
        }
        Fields::Unit => Ok(quote! {
            decoder.decode_null()?;
            Ok(Self)
        }),
    }
}

fn decode_named_fields(
    fields: &Fields,
    container: &ContainerAttrs,
    krate: &Path,
    construct: TokenStream2,
    begin: bool,
) -> syn::Result<TokenStream2> {
    let Fields::Named(fields) = fields else {
        return Err(syn::Error::new(Span::call_site(), "expected named fields"));
    };
    let mut locals = Vec::new();
    let mut arms = Vec::new();
    let mut construct_fields = Vec::new();
    let deny_duplicates = container.deny_duplicate_fields;

    for (index, field) in fields.named.iter().enumerate() {
        let ident = field.ident.as_ref().unwrap();
        let attrs = parse_field(&field.attrs)?;
        let local = format_ident!("__json_field_{index}");
        let ty = &field.ty;
        let decode = decode_expr(&attrs, ty, krate);
        if !attrs.skip {
            let mut keys = vec![field_key(field, index, &attrs, container)];
            keys.extend(attrs.aliases.iter().cloned());
            let assignment = quote! {
                if __json_track_duplicates
                    && !decoder.duplicate_action(#local.is_some(), #deny_duplicates)?
                {
                    decoder.skip_value()?;
                } else {
                    #local = Some(#decode);
                }
            };
            for key in keys {
                arms.push(quote! { #key => { #assignment } });
            }
        }
        let default = if let Some(default) = &attrs.default {
            match default {
                Some(path) => quote! { #path() },
                None => quote! { ::core::default::Default::default() },
            }
        } else if is_option_type(ty) {
            quote! { ::core::option::Option::None }
        } else if attrs.skip {
            quote! { ::core::default::Default::default() }
        } else {
            let ident_str = syn::LitStr::new(&ident.to_string(), ident.span());
            quote! { return Err(decoder.error(#krate::ErrorKind::MissingField(#ident_str))) }
        };
        locals.push(quote! { let mut #local: ::core::option::Option<#ty> = ::core::option::Option::None; });
        construct_fields.push(quote! { #ident: match #local { ::core::option::Option::Some(value) => value, ::core::option::Option::None => #default } });
    }

    let unknown = if container.deny_unknown_fields {
        quote! { return Err(decoder.error(#krate::ErrorKind::UnknownField)); }
    } else {
        quote! { decoder.skip_value()?; }
    };

    let begin_stmt = if begin {
        quote! { decoder.begin_object()?; }
    } else {
        TokenStream2::new()
    };
    let begin_first = begin;

    Ok(quote! { {
        #begin_stmt
        #(#locals)*
        let __json_track_duplicates = #deny_duplicates || decoder.tracks_duplicates();
        let mut __json_first = #begin_first;
        while decoder.object_next(__json_first)? {
            __json_first = false;
            let key = decoder.key()?;
            match key.as_str() {
                #(#arms)*
                _ => { #unknown }
            }
        }
        #construct { #(#construct_fields),* }
    } })
}

fn decode_tuple_fields(
    fields: &Fields,
    _container: &ContainerAttrs,
    krate: &Path,
    construct: TokenStream2,
) -> syn::Result<TokenStream2> {
    let Fields::Unnamed(fields) = fields else {
        return Err(syn::Error::new(Span::call_site(), "expected tuple fields"));
    };
    let mut statements = Vec::new();
    let mut values = Vec::new();
    for (index, field) in fields.unnamed.iter().enumerate() {
        let attrs = parse_field(&field.attrs)?;
        let ty = &field.ty;
        if attrs.skip {
            let default = if let Some(default) = &attrs.default {
                match default {
                    Some(path) => quote! { #path() },
                    None => quote! { ::core::default::Default::default() },
                }
            } else if is_option_type(ty) {
                quote! { ::core::option::Option::None }
            } else {
                quote! { ::core::default::Default::default() }
            };
            values.push(default);
            continue;
        }
        let decode = decode_expr(&attrs, ty, krate);
        let local = format_ident!("__json_value_{index}");
        statements.push(quote! {
            if !decoder.array_next(__json_first)? {
                return Err(decoder.error(#krate::ErrorKind::ExpectedValue));
            }
            __json_first = false;
            let #local = #decode;
        });
        values.push(quote! { #local });
    }
    Ok(quote! { {
        decoder.begin_array()?;
        let mut __json_first = true;
        #(#statements)*
        if decoder.array_next(__json_first)? {
            return Err(decoder.error(#krate::ErrorKind::ExpectedValue));
        }
        #construct(#(#values),*)
    } })
}

fn decode_expr(attrs: &FieldAttrs, ty: &Type, _krate: &Path) -> TokenStream2 {
    if let Some(path) = &attrs.decode_with {
        return quote! { #path(decoder)? };
    }
    if let Some(path) = &attrs.with {
        return quote! { #path::deserialize(decoder)? };
    }
    quote! { decoder.decode_inline::<#ty>()? }
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(path) = ty
        && let Some(segment) = path.path.segments.last()
    {
        return segment.ident == "Option";
    }
    false
}

fn encode_enum(data: &DataEnum, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    if container.untagged {
        return encode_untagged(data, container, krate);
    }
    let mut arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_variant(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let name = variant_name(variant, &attrs, container);
        let pattern = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                if let Some(tag) = &container.tag {
                    arms.push(quote! {
                        Self::#pattern => {
                            encoder.begin_object()?;
                            encoder.comma(true)?;
                            encoder.key(#tag)?;
                            encoder.string(#name)?;
                            encoder.end_object()
                        }
                    });
                } else {
                    arms.push(quote! { Self::#pattern => encoder.string(#name) });
                }
            }
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let attrs = parse_field(&fields.unnamed[0].attrs)?;
                let encode = encode_field_expr(&attrs, &quote!(__json_value), krate);
                if let Some(tag) = &container.tag {
                    if let Some(content) = &container.content {
                        arms.push(quote! {
                            Self::#pattern(__json_value) => {
                                encoder.begin_object()?;
                                encoder.comma(true)?;
                                encoder.key(#tag)?;
                                encoder.string(#name)?;
                                encoder.comma(false)?;
                                encoder.key(#content)?;
                                #encode;
                                encoder.end_object()
                            }
                        });
                    } else {
                        return Err(syn::Error::new_spanned(
                            variant,
                            "internally tagged enums only support unit and struct variants",
                        ));
                    }
                } else {
                    arms.push(quote! {
                        Self::#pattern(__json_value) => {
                            encoder.begin_object()?;
                            encoder.comma(true)?;
                            encoder.key(#name)?;
                            #encode;
                            encoder.end_object()
                        }
                    });
                }
            }
            Fields::Unnamed(fields) => {
                if container.tag.is_some() && container.content.is_none() {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "internally tagged enums only support unit and struct variants",
                    ));
                }
                let bindings: Vec<_> = (0..fields.unnamed.len())
                    .map(|index| format_ident!("__json_value_{index}"))
                    .collect();
                let body = encode_tuple_fields(fields, container, krate, |_field, index| {
                    let binding = &bindings[index];
                    quote!(#binding)
                })?;
                let binds = bindings.iter();
                if let Some(tag) = &container.tag {
                    let content = container.content.as_ref().expect("adjacent tag");
                    arms.push(quote! {
                        Self::#pattern(#(#binds),*) => {
                            encoder.begin_object()?;
                            encoder.comma(true)?;
                            encoder.key(#tag)?;
                            encoder.string(#name)?;
                            encoder.comma(false)?;
                            encoder.key(#content)?;
                            #body?;
                            encoder.end_object()
                        }
                    });
                } else {
                    arms.push(quote! {
                        Self::#pattern(#(#binds),*) => {
                            encoder.begin_object()?;
                            encoder.comma(true)?;
                            encoder.key(#name)?;
                            #body?;
                            encoder.end_object()
                        }
                    });
                }
            }
            Fields::Named(fields) => {
                let bindings: Vec<_> = fields.named.iter().map(|field| field.ident.clone().unwrap()).collect();
                let body = encode_named_fields(fields, container, krate, |_field, index| {
                    let binding = &bindings[index];
                    quote!(#binding)
                })?;
                let binds = bindings.iter();
                if let Some(tag) = &container.tag {
                    if let Some(content) = &container.content {
                        arms.push(quote! {
                            Self::#pattern { #(#binds),* } => {
                                encoder.begin_object()?;
                                encoder.comma(true)?;
                                encoder.key(#tag)?;
                                encoder.string(#name)?;
                                encoder.comma(false)?;
                                encoder.key(#content)?;
                                #body?;
                                encoder.end_object()
                            }
                        });
                    } else {
                        // Internally tagged: write the tag, then the fields
                        // directly into the same object.
                        let mut field_statements = Vec::new();
                        for (index, field) in fields.named.iter().enumerate() {
                            let attrs = parse_field(&field.attrs)?;
                            if attrs.skip {
                                continue;
                            }
                            let ident = field.ident.as_ref().unwrap();
                            let key = field_key(field, index, &attrs, container);
                            let encode = encode_field_expr(&attrs, &quote!(#ident), krate);
                            field_statements.push(quote! {
                                encoder.comma(first)?;
                                first = false;
                                encoder.key(#key)?;
                                #encode;
                            });
                        }
                        let tags_and_fields = quote! {
                            encoder.begin_object()?;
                            encoder.comma(true)?;
                            encoder.key(#tag)?;
                            encoder.string(#name)?;
                            let mut first = false;
                            #(#field_statements)*
                            encoder.end_object()
                        };
                        arms.push(quote! {
                            Self::#pattern { #(#binds),* } => { #tags_and_fields }
                        });
                    }
                } else {
                    arms.push(quote! {
                        Self::#pattern { #(#binds),* } => {
                            encoder.begin_object()?;
                            encoder.comma(true)?;
                            encoder.key(#name)?;
                            #body?;
                            encoder.end_object()
                        }
                    });
                }
            }
        }
    }
    Ok(quote! {
        match self {
            #(#arms),*
        }
    })
}

fn decode_enum(
    data: &DataEnum,
    container: &ContainerAttrs,
    krate: &Path,
    lifetime: &syn::Lifetime,
) -> syn::Result<TokenStream2> {
    if container.untagged {
        return decode_untagged(data, container, krate);
    }
    if let Some(tag) = &container.tag {
        if let Some(content) = &container.content {
            return decode_adjacently_tagged(data, container, krate, lifetime, tag, content);
        }
        return decode_internally_tagged(data, container, krate, tag);
    }
    decode_externally_tagged(data, container, krate)
}

fn decode_variant_expr(
    fields: &Fields,
    container: &ContainerAttrs,
    krate: &Path,
    construct: TokenStream2,
) -> syn::Result<TokenStream2> {
    match fields {
        Fields::Unit => Ok(construct),
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let attrs = parse_field(&fields.unnamed[0].attrs)?;
            let decode = decode_expr(&attrs, &fields.unnamed[0].ty, krate);
            Ok(quote! { #construct(#decode) })
        }
        Fields::Unnamed(_) => decode_tuple_fields(fields, container, krate, construct),
        Fields::Named(_) => decode_named_fields(fields, container, krate, construct, true),
    }
}

fn encode_untagged(data: &DataEnum, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    let mut arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_variant(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => arms.push(quote! { Self::#ident => encoder.null() }),
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let attrs = parse_field(&fields.unnamed[0].attrs)?;
                let encode = encode_field_expr(&attrs, &quote!(__json_value), krate);
                arms.push(quote! { Self::#ident(__json_value) => { #encode; Ok(()) } });
            }
            Fields::Unnamed(fields) => {
                let bindings: Vec<_> = (0..fields.unnamed.len())
                    .map(|index| format_ident!("__json_value_{index}"))
                    .collect();
                let body = encode_tuple_fields(fields, container, krate, |_field, index| {
                    let binding = &bindings[index];
                    quote!(#binding)
                })?;
                let binds = bindings.iter();
                arms.push(quote! { Self::#ident(#(#binds),*) => { #body } });
            }
            Fields::Named(fields) => {
                let bindings: Vec<_> = fields.named.iter().map(|field| field.ident.clone().unwrap()).collect();
                let body = encode_named_fields(fields, container, krate, |_field, index| {
                    let binding = &bindings[index];
                    quote!(#binding)
                })?;
                let binds = bindings.iter();
                arms.push(quote! { Self::#ident { #(#binds),* } => { #body } });
            }
        }
    }
    Ok(quote! {
        match self {
            #(#arms),*
        }
    })
}

fn decode_untagged(data: &DataEnum, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    let mut attempts = Vec::new();
    for variant in &data.variants {
        let attrs = parse_variant(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => attempts.push(quote! {
                {
                    let mut decoder = #krate::Decoder::new(raw.get());
                    if decoder.decode_null().is_ok() && decoder.finish().is_ok() {
                        return Ok(Self::#ident);
                    }
                }
            }),
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let attrs = parse_field(&fields.unnamed[0].attrs)?;
                let decode = decode_expr(&attrs, &fields.unnamed[0].ty, krate);
                attempts.push(quote! {
                    {
                        let mut decoder = #krate::Decoder::new(raw.get());
                        let result = (|| -> #krate::Result<Self> {
                            let value = #decode;
                            decoder.finish()?;
                            Ok(Self::#ident(value))
                        })();
                        if let Ok(value) = result {
                            return Ok(value);
                        }
                    }
                });
            }
            Fields::Unnamed(_) => {
                let expr = decode_tuple_fields(&variant.fields, container, krate, quote!(Self::#ident))?;
                attempts.push(quote! {
                    {
                        let mut decoder = #krate::Decoder::new(raw.get());
                        let result = (|| -> #krate::Result<Self> {
                            let value = #expr;
                            decoder.finish()?;
                            Ok(value)
                        })();
                        if let Ok(value) = result {
                            return Ok(value);
                        }
                    }
                });
            }
            Fields::Named(_) => {
                let expr = decode_named_fields(&variant.fields, container, krate, quote!(Self::#ident), true)?;
                attempts.push(quote! {
                    {
                        let mut decoder = #krate::Decoder::new(raw.get());
                        let result = (|| -> #krate::Result<Self> {
                            let value = #expr;
                            decoder.finish()?;
                            Ok(value)
                        })();
                        if let Ok(value) = result {
                            return Ok(value);
                        }
                    }
                });
            }
        }
    }
    Ok(quote! {
        let raw = decoder.raw_value()?;
        #(#attempts)*
        Err(decoder.error(#krate::ErrorKind::UnknownVariant))
    })
}

fn decode_externally_tagged(data: &DataEnum, container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    let mut string_arms = Vec::new();
    let mut object_arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_variant(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let name = variant_name(variant, &attrs, container);
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                string_arms.push(quote! { #name => Ok(Self::#ident) });
            }
            _ => {
                let construct = quote!(Self::#ident);
                let expr = decode_variant_expr(&variant.fields, container, krate, construct)?;
                object_arms.push(quote! {
                    #name => {
                        let value = #expr;
                        if !decoder.object_done()? {
                            return Err(decoder.error(#krate::ErrorKind::ExpectedEnd));
                        }
                        Ok(value)
                    }
                });
            }
        }
    }

    Ok(quote! {
        decoder.skip_ws()?;
        match decoder.peek_byte() {
            Some(b'"') => {
                let __json_name = decoder.decode_str()?;
                match __json_name {
                    #(#string_arms,)*
                    _ => Err(decoder.error(#krate::ErrorKind::UnknownVariant)),
                }
            }
            Some(b'{') => {
                decoder.begin_object()?;
                if !decoder.object_next(true)? {
                    return Err(decoder.error(#krate::ErrorKind::UnknownVariant));
                }
                let __json_name = decoder.key()?;
                match __json_name.as_str() {
                    #(#object_arms,)*
                    _ => Err(decoder.error(#krate::ErrorKind::UnknownVariant)),
                }
            }
            None => Err(decoder.error(#krate::ErrorKind::UnexpectedEof)),
            Some(_) => Err(decoder.error(#krate::ErrorKind::ExpectedValue)),
        }
    })
}

fn decode_internally_tagged(
    data: &DataEnum,
    container: &ContainerAttrs,
    krate: &Path,
    tag: &str,
) -> syn::Result<TokenStream2> {
    let mut arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_variant(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let name = variant_name(variant, &attrs, container);
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => {
                arms.push(quote! {
                    #name => {
                        if !decoder.object_done()? {
                            return Err(decoder.error(#krate::ErrorKind::ExpectedEnd));
                        }
                        Ok(Self::#ident)
                    }
                });
            }
            Fields::Named(_) => {
                let expr = decode_named_fields(&variant.fields, container, krate, quote!(Self::#ident), false)?;
                arms.push(quote! { #name => Ok(#expr) });
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    variant,
                    "internally tagged enums only support unit and struct variants",
                ));
            }
        }
    }
    Ok(quote! {
        decoder.begin_object()?;
        if !decoder.object_next(true)? {
            return Err(decoder.error(#krate::ErrorKind::MissingField(#tag)));
        }
        let __json_tag = decoder.key()?;
        if __json_tag.as_str() != #tag {
            return Err(decoder.error(#krate::ErrorKind::MissingField(#tag)));
        }
        let __json_variant = decoder.decode_str()?;
        match __json_variant {
            #(#arms,)*
            _ => Err(decoder.error(#krate::ErrorKind::UnknownVariant)),
        }
    })
}

fn decode_adjacently_tagged(
    data: &DataEnum,
    container: &ContainerAttrs,
    krate: &Path,
    _lifetime: &syn::Lifetime,
    tag: &str,
    content: &str,
) -> syn::Result<TokenStream2> {
    let mut arms = Vec::new();
    for variant in &data.variants {
        let attrs = parse_variant(&variant.attrs)?;
        if attrs.skip {
            continue;
        }
        let name = variant_name(variant, &attrs, container);
        let ident = &variant.ident;
        match &variant.fields {
            Fields::Unit => arms.push(quote! {
                #name => Ok(Self::#ident)
            }),
            _ => {
                let construct = quote!(Self::#ident);
                let expr = decode_variant_expr(&variant.fields, container, krate, construct)?;
                arms.push(quote! {
                    #name => {
                        let raw = __json_content.ok_or_else(|| decoder.error(#krate::ErrorKind::MissingField(#content)))?;
                        let mut __json_decoder = #krate::Decoder::new(raw.get());
                        let __json_value = {
                            let decoder = &mut __json_decoder;
                            #expr
                        };
                        __json_decoder.finish()?;
                        Ok(__json_value)
                    }
                });
            }
        }
    }
    Ok(quote! {
        decoder.begin_object()?;
        let mut __json_tag_value = None;
        let mut __json_content = None;
        let mut __json_first = true;
        while decoder.object_next(__json_first)? {
            __json_first = false;
            let key = decoder.key()?;
            match key.as_str() {
                #tag => { __json_tag_value = Some(decoder.decode_str()?); }
                #content => { __json_content = Some(decoder.raw_value()?); }
                _ => { decoder.skip_value()?; }
            }
        }
        let __json_tag_value = __json_tag_value.ok_or_else(|| decoder.error(#krate::ErrorKind::MissingField(#tag)))?;
        match __json_tag_value {
            #(#arms,)*
            _ => Err(decoder.error(#krate::ErrorKind::UnknownVariant)),
        }
    })
}
