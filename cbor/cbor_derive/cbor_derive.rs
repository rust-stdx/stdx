//! Derive macros for the `cbor` crate.
//!
//! `#[derive(Encode, Decode)]` generates the `cbor::Encode` and
//! `cbor::Decode` implementations.
//!
//! Structs use a text-key map by default. `#[cbor(array)]` selects a
//! positional array and `#[cbor(key = N)]` selects an integer-keyed map.
//! Enums use serde-compatible external tagging by default; `#[cbor(array)]`
//! uses a flat `[variant_index, fields...]` array, `#[cbor(index_only)]`
//! encodes the variant index as an integer, and `#[cbor(tagged)]` encodes
//! `[variant_index, {field_key: value, ...}]` (the legacy map-enum shape).
//!
//! See the `cbor` crate documentation for the full attribute list.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, ExprLit, ExprUnary, Field, Fields, GenericParam, Ident,
    Lit, LitInt, LitStr, Path, Token, Type, UnOp, WherePredicate,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    visit::Visit,
};

/// Derive `cbor::Encode`.
///
/// Structs use a text-key map by default. Container attributes:
///
/// - `#[cbor(array)]` — positional array
/// - `#[cbor(map)]` — text-key map (the default)
/// - `#[cbor(tagged)]` — enums only: `[variant_index, {field_key: value}]`
/// - `#[cbor(tag = N)]` / `#[cbor(tag_optional)]` — wrap in a tag
/// - `#[cbor(transparent)]` — use the single field's encoding
/// - `#[cbor(rename_all = "...")]` — rename text keys
/// - `#[cbor(declaration_order)]` — keep declaration order instead of
///   canonical key order
/// - `#[cbor(deny_unknown_fields)]` — reject unknown map keys
/// - `#[cbor(bound = "...")]` / `#[cbor(crate = "...")]`
/// - `#[cbor(encode_only)]` / `#[cbor(decode_only)]`
/// - `#[cbor(index_only)]` — enums only
///
/// Field and variant attributes:
///
/// - `#[cbor(key = N)]` — integer map key
/// - `#[cbor(index = N)]` — explicit array position or variant index
/// - `#[cbor(rename = "...")]`
/// - `#[cbor(skip)]`, `#[cbor(default)]`, `#[cbor(default = "path")]`
/// - `#[cbor(skip_if_none)]`, `#[cbor(skip_if = "path")]`
/// - `#[cbor(bytes)]` — encode `Vec<u8>`/`&[u8]` as a byte string
/// - `#[cbor(with = "module")]`, `#[cbor(encode_with = "...")]`,
///   `#[cbor(decode_with = "...")]`
/// - `#[cbor(borrow)]` — borrow from the decoder input
///
/// Enums use serde-compatible external tagging by default. Use
/// `#[cbor(array)]` for `[variant_index, fields...]`, `#[cbor(index_only)]`
/// for an integer index, `#[cbor(tagged)]` for `[variant_index, {field_key:
/// value, ...}]`, or `#[cbor(key = N)]` on variants for an integer-keyed map.
#[proc_macro_derive(Encode, attributes(cbor))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    match expand(input.into(), Direction::Encode) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Derive `cbor::Decode`.
///
/// This accepts the same attributes as [`derive@Encode`].
#[proc_macro_derive(Decode, attributes(cbor))]
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
    map: bool,
    array: bool,
    tagged: bool,
    tag: Option<u64>,
    tag_optional: bool,
    transparent: bool,
    rename_all: Option<RenameRule>,
    declaration_order: bool,
    index_only: bool,
    encode_only: bool,
    decode_only: bool,
    deny_unknown_fields: bool,
    krate: Option<Path>,
    bound: Option<String>,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    key: Option<i64>,
    index: Option<usize>,
    rename: Option<String>,
    skip: bool,
    default: Option<Option<Path>>,
    skip_if_none: bool,
    skip_if: Option<Path>,
    borrow: bool,
    bytes: bool,
    with: Option<Path>,
    encode_with: Option<Path>,
    decode_with: Option<Path>,
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
        if !attr.path().is_ident("cbor") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("map") {
                out.map = true;
            } else if meta.path.is_ident("array") {
                out.array = true;
            } else if meta.path.is_ident("tagged") {
                out.tagged = true;
            } else if meta.path.is_ident("transparent") {
                out.transparent = true;
            } else if meta.path.is_ident("declaration_order") {
                out.declaration_order = true;
            } else if meta.path.is_ident("index_only") {
                out.index_only = true;
            } else if meta.path.is_ident("encode_only") {
                out.encode_only = true;
            } else if meta.path.is_ident("decode_only") {
                out.decode_only = true;
            } else if meta.path.is_ident("deny_unknown_fields") {
                out.deny_unknown_fields = true;
            } else if meta.path.is_ident("tag_optional") {
                out.tag_optional = true;
            } else if meta.path.is_ident("tag") {
                out.tag = Some(parse_u64(meta.value()?)?);
            } else if meta.path.is_ident("rename_all") {
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
                return Err(meta.error("unknown cbor container attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_field(attrs: &[Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("cbor") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("borrow") {
                out.borrow = true;
            } else if meta.path.is_ident("bytes") {
                out.bytes = true;
            } else if meta.path.is_ident("skip_if_none") {
                out.skip_if_none = true;
            } else if meta.path.is_ident("key") {
                out.key = Some(parse_i64(meta.value()?)?);
            } else if meta.path.is_ident("index") {
                out.index = Some(parse_usize(meta.value()?)?);
            } else if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename = Some(value.value());
            } else if meta.path.is_ident("default") {
                if meta.input.peek(Token![=]) {
                    let value: LitStr = meta.value()?.parse()?;
                    out.default =
                        Some(Some(syn::parse_str(&value.value()).map_err(|e| {
                            syn::Error::new(value.span(), format!("invalid default path: {e}"))
                        })?));
                } else {
                    out.default = Some(None);
                }
            } else if meta.path.is_ident("skip_if") {
                let value: LitStr = meta.value()?.parse()?;
                out.skip_if = Some(
                    syn::parse_str(&value.value())
                        .map_err(|e| syn::Error::new(value.span(), format!("invalid skip_if path: {e}")))?,
                );
            } else if meta.path.is_ident("with") {
                let value: LitStr = meta.value()?.parse()?;
                out.with = Some(
                    syn::parse_str(&value.value())
                        .map_err(|e| syn::Error::new(value.span(), format!("invalid with path: {e}")))?,
                );
            } else if meta.path.is_ident("encode_with") {
                let value: LitStr = meta.value()?.parse()?;
                out.encode_with = Some(
                    syn::parse_str(&value.value())
                        .map_err(|e| syn::Error::new(value.span(), format!("invalid encode_with path: {e}")))?,
                );
            } else if meta.path.is_ident("decode_with") {
                let value: LitStr = meta.value()?.parse()?;
                out.decode_with = Some(
                    syn::parse_str(&value.value())
                        .map_err(|e| syn::Error::new(value.span(), format!("invalid decode_with path: {e}")))?,
                );
            } else {
                return Err(meta.error("unknown cbor field attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_u64(input: ParseStream) -> syn::Result<u64> {
    let value: LitInt = input.parse()?;
    value.base10_parse()
}

fn parse_usize(input: ParseStream) -> syn::Result<usize> {
    let value: LitInt = input.parse()?;
    value.base10_parse()
}

fn parse_i64(input: ParseStream) -> syn::Result<i64> {
    let expr: Expr = input.parse()?;
    match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Int(value), ..
        }) => value.base10_parse(),
        Expr::Unary(ExprUnary {
            op: UnOp::Neg(_),
            expr,
            ..
        }) => match *expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(value), ..
            }) => Ok(-value.base10_parse::<i64>()?),
            other => Err(syn::Error::new(other.span(), "expected an integer")),
        },
        other => Err(syn::Error::new(other.span(), "expected an integer")),
    }
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
    if container.map && container.array {
        return Err(syn::Error::new(input.ident.span(), "`map` and `array` are mutually exclusive"));
    }
    if container.tagged {
        if container.index_only || container.array || container.map {
            return Err(syn::Error::new(
                input.ident.span(),
                "`tagged` cannot be combined with `map`, `array` or `index_only`",
            ));
        }
        if !matches!(input.data, Data::Enum(_)) {
            return Err(syn::Error::new(input.ident.span(), "`tagged` requires an enum"));
        }
    }
    if container.transparent {
        if container.map || container.array {
            return Err(syn::Error::new(
                input.ident.span(),
                "`transparent` cannot be combined with `map` or `array`",
            ));
        }
        return expand_transparent(&input, &container, direction);
    }
    match &input.data {
        Data::Struct(data) => expand_struct(&input, &container, data, direction),
        Data::Enum(data) => expand_enum(&input, &container, data, direction),
        Data::Union(_) => Err(syn::Error::new(input.ident.span(), "unions are not supported; use a struct")),
    }
}

fn expand_transparent(
    input: &DeriveInput,
    container: &ContainerAttrs,
    direction: Direction,
) -> syn::Result<TokenStream2> {
    let data = match &input.data {
        Data::Struct(data) => data,
        _ => return Err(syn::Error::new(input.ident.span(), "`transparent` requires a struct")),
    };
    if data.fields.len() != 1 {
        return Err(syn::Error::new(input.ident.span(), "`transparent` requires exactly one field"));
    }
    let field = data.fields.iter().next().unwrap();
    let field_attrs = parse_field(&field.attrs)?;
    if field_attrs.skip {
        return Err(syn::Error::new(field.span(), "a transparent field cannot be skipped"));
    }
    let field_ident = field.ident.as_ref();
    let krate = krate(container);
    let tag_write = container.tag.map(|tag| quote!(e.tag(#tag)?;));
    let tag_read = tag_check(container, &krate);
    let name = &input.ident;

    match direction {
        Direction::Encode => {
            let (impl_generics, ty_generics, where_clause) = encode_generics(input, container, &krate);
            let access = match field_ident {
                Some(ident) => quote!(self.#ident),
                None => quote!(self.0),
            };
            let encode_expr = encode_field_expr(&krate, &field.ty, &field_attrs, &quote!(&#access), direction)?;
            Ok(quote! {
                impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
                    #[inline]
                    fn encode<W: #krate::Write>(&self, e: &mut #krate::Encoder<W>)
                        -> ::core::result::Result<(), W::Error>
                    {
                        #tag_write
                        #encode_expr
                        Ok(())
                    }
                }
            })
        }
        Direction::Decode => {
            let (impl_generics, ty_generics, where_clause) = decode_generics(input, container, &krate)?;
            let lifetime = decode_lifetime(input);
            let decode_expr = decode_field_expr(&krate, &field.ty, &field_attrs, direction)?;
            let construct = match field_ident {
                Some(ident) => quote!(Self { #ident: value }),
                None => quote!(Self(value)),
            };
            Ok(quote! {
                impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
                    #[inline]
                    fn decode(d: &mut #krate::Decoder<#lifetime>)
                        -> #krate::Result<Self>
                    {
                        #tag_read
                        d.enter_depth()?;
                        let result = (|| {
                            let value = #decode_expr;
                            Ok(#construct)
                        })();
                        d.leave_depth();
                        result
                    }
                }
            })
        }
    }
}

fn expand_struct(
    input: &DeriveInput,
    container: &ContainerAttrs,
    data: &DataStruct,
    direction: Direction,
) -> syn::Result<TokenStream2> {
    let fields = struct_fields(data)?;
    let array = container.array;
    let has_key = fields.iter().any(|f| f.attrs.key.is_some());
    if array && has_key {
        return Err(syn::Error::new(input.ident.span(), "`array` cannot be combined with `key`"));
    }
    if array {
        for field in &fields {
            if field.attrs.skip_if_none || field.attrs.skip_if.is_some() {
                return Err(syn::Error::new(
                    field.span,
                    "conditional omission is not supported for array structs; use Option or #[cbor(skip)]",
                ));
            }
        }
    }
    if !array && data.fields.iter().any(|f| f.ident.is_none()) && !has_key {
        return Err(syn::Error::new(
            input.ident.span(),
            "tuple structs need `#[cbor(array)]` or integer `#[cbor(key = N)]`",
        ));
    }
    if array {
        expand_struct_array(input, container, &fields, direction)
    } else {
        expand_struct_map(input, container, &fields, direction)
    }
}

fn expand_struct_map(
    input: &DeriveInput,
    container: &ContainerAttrs,
    fields: &[FieldInfo],
    direction: Direction,
) -> syn::Result<TokenStream2> {
    let krate = krate(container);
    let name = &input.ident;
    let keys = map_keys(container, fields)?;
    let tag_write = container.tag.map(|tag| quote!(e.tag(#tag)?;));
    let tag_read = tag_check(container, &krate);

    match direction {
        Direction::Encode => {
            let (impl_generics, ty_generics, where_clause) = encode_generics(input, container, &krate);
            let mut len_terms = Vec::new();
            for (field, _) in fields.iter().zip(keys.iter()) {
                if field.attrs.skip {
                    continue;
                }
                len_terms.push(1usize);
            }
            let base_len = len_terms.len();
            let mut len_adjust = Vec::new();
            let mut body = Vec::new();
            let order = key_order(&keys, container.declaration_order);
            for &index in &order {
                let field = &fields[index];
                let key = &keys[index];
                if field.attrs.skip {
                    continue;
                }
                let access = field_access(field);
                let key_write = keys_write(&krate, key)?;
                if field.attrs.skip_if_none {
                    len_adjust.push(quote! {
                        if #access.is_none() { __len -= 1; }
                    });
                    let value_ty = option_inner(&field.ty).unwrap_or(&field.ty);
                    let encode_expr = encode_field_expr(&krate, value_ty, &field.attrs, &quote!(__value), direction)?;
                    body.push(quote! {
                        if let ::core::option::Option::Some(__value) = &#access {
                            #key_write
                            #encode_expr
                        }
                    });
                } else if let Some(pred) = &field.attrs.skip_if {
                    let encode_expr = encode_field_expr(&krate, &field.ty, &field.attrs, &quote!(&#access), direction)?;
                    len_adjust.push(quote! {
                        if #pred(&#access) { __len -= 1; }
                    });
                    body.push(quote! {
                        if !#pred(&#access) {
                            #key_write
                            #encode_expr
                        }
                    });
                } else {
                    let encode_expr = encode_field_expr(&krate, &field.ty, &field.attrs, &quote!(&#access), direction)?;
                    body.push(quote! {
                        #key_write
                        #encode_expr
                    });
                }
            }
            Ok(quote! {
                impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
                    fn encode<W: #krate::Write>(&self, e: &mut #krate::Encoder<W>)
                        -> ::core::result::Result<(), W::Error>
                    {
                        #tag_write
                        let mut __len: usize = #base_len;
                        #(#len_adjust)*
                        e.map(__len)?;
                        #(#body)*
                        Ok(())
                    }
                }
            })
        }
        Direction::Decode => {
            let (impl_generics, ty_generics, where_clause) = decode_generics(input, container, &krate)?;
            let lifetime = decode_lifetime(input);
            let mut declarations = Vec::new();
            let mut arms = Vec::new();
            let mut init = Vec::new();
            if fields.len() <= 128 {
                init.push(quote!(let mut __seen: u128 = 0u128;));
            } else {
                let n = fields.len();
                init.push(quote!(let mut __seen = [false; #n];));
            }
            for (index, (field, key)) in fields.iter().zip(keys.iter()).enumerate() {
                let ident = field_ident(field, index);
                let ty = &field.ty;
                declarations.push(quote!(let mut #ident: ::core::option::Option<#ty> = ::core::option::Option::None;));
                if field.attrs.skip {
                    continue;
                }
                let key_match = key_match(&krate, key)?;
                let decode_expr = decode_field_expr(&krate, &field.ty, &field.attrs, direction)?;
                let seen_check = if fields.len() <= 128 {
                    let flag = quote!(1u128 << #index);
                    quote! {
                        if __seen & #flag != 0 { return Err(#krate::Error::duplicate_key().at(__start)); }
                        __seen |= #flag;
                    }
                } else {
                    quote! {
                        if __seen[#index] { return Err(#krate::Error::duplicate_key().at(__start)); }
                        __seen[#index] = true;
                    }
                };
                arms.push(quote! {
                    #key_match => {
                        #seen_check
                        #ident = ::core::option::Option::Some(#decode_expr);
                    }
                });
            }
            let construct = struct_construct(fields, container, &krate)?;
            let has_key = fields.iter().any(|f| f.attrs.key.is_some());
            let all_non_negative = keys.iter().all(|key| match key {
                WireKey::Int(value) => *value >= 0,
                _ => true,
            });
            let key_read = if has_key && all_non_negative {
                quote!(let __key = d.u64()?;)
            } else if has_key {
                quote!(let __key = d.i64()?;)
            } else {
                quote!(let __key = d.text()?;)
            };
            let unknown_arm = if container.deny_unknown_fields {
                quote!(_ => { return Err(#krate::Error::message("unknown field").at(d.position())); })
            } else {
                quote!(_ => { d.skip()?; })
            };
            let loop_body = quote! {
                #(#arms)*
                #unknown_arm
            };
            Ok(quote! {
                impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
                    fn decode(d: &mut #krate::Decoder<#lifetime>) -> #krate::Result<Self> {
                        #tag_read
                        d.enter_depth()?;
                        let result = (|| {
                            let __start = d.position();
                            #(#declarations)*
                            #(#init)*
                            match d.map()? {
                                ::core::option::Option::Some(__len) => {
                                    for _ in 0..__len {
                                        #key_read
                                        match __key {
                                            #loop_body
                                        }
                                    }
                                }
                                ::core::option::Option::None => loop {
                                    if d.is_break() { d.read_break()?; break; }
                                    #key_read
                                    match __key {
                                        #loop_body
                                    }
                                }
                            }
                            Ok(#construct)
                        })();
                        d.leave_depth();
                        result
                    }
                }
            })
        }
    }
}

fn expand_struct_array(
    input: &DeriveInput,
    container: &ContainerAttrs,
    fields: &[FieldInfo],
    direction: Direction,
) -> syn::Result<TokenStream2> {
    let krate = krate(container);
    let name = &input.ident;
    let indices = array_indices(fields)?;
    let max_index = indices.iter().copied().max().map(|i| i + 1).unwrap_or(0);
    let tag_write = container.tag.map(|tag| quote!(e.tag(#tag)?;));
    let tag_read = tag_check(container, &krate);

    match direction {
        Direction::Encode => {
            let (impl_generics, ty_generics, where_clause) = encode_generics(input, container, &krate);
            let mut body = Vec::new();
            for index in 0..max_index {
                if let Some(field_index) = indices.iter().position(|&i| i == index) {
                    let field = &fields[field_index];
                    if field.attrs.skip {
                        body.push(quote!(e.null()?;));
                    } else {
                        let access = field_access(field);
                        let encode_expr =
                            encode_field_expr(&krate, &field.ty, &field.attrs, &quote!(&#access), direction)?;
                        body.push(encode_expr);
                    }
                } else {
                    body.push(quote!(e.null()?;));
                }
            }
            Ok(quote! {
                impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
                    fn encode<W: #krate::Write>(&self, e: &mut #krate::Encoder<W>)
                        -> ::core::result::Result<(), W::Error>
                    {
                        #tag_write
                        e.array(#max_index)?;
                        #(#body)*
                        Ok(())
                    }
                }
            })
        }
        Direction::Decode => {
            let (impl_generics, ty_generics, where_clause) = decode_generics(input, container, &krate)?;
            let lifetime = decode_lifetime(input);
            let mut declarations = Vec::new();
            let mut decode_arms = Vec::new();
            for (index, field) in fields.iter().enumerate() {
                let ident = field_ident(field, index);
                let ty = &field.ty;
                declarations.push(quote!(let mut #ident: ::core::option::Option<#ty> = ::core::option::Option::None;));
                let wire = indices[index];
                if field.attrs.skip {
                    decode_arms.push(quote! {
                        #wire => { d.skip()?; }
                    });
                } else {
                    let decode_expr = decode_field_expr(&krate, &field.ty, &field.attrs, direction)?;
                    decode_arms.push(quote! {
                        #wire => { #ident = ::core::option::Option::Some(#decode_expr); }
                    });
                }
            }
            let construct = struct_construct(fields, container, &krate)?;
            Ok(quote! {
                impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
                    fn decode(d: &mut #krate::Decoder<#lifetime>) -> #krate::Result<Self> {
                        #tag_read
                        d.enter_depth()?;
                        let result = (|| {
                            let __start = d.position();
                            #(#declarations)*
                            let mut __index: usize = 0;
                            match d.array()? {
                                ::core::option::Option::Some(__len) => {
                                    for _ in 0..__len {
                                        match __index {
                                            #(#decode_arms)*
                                            _ => { d.skip()?; }
                                        }
                                        __index += 1;
                                    }
                                }
                                ::core::option::Option::None => loop {
                                    if d.is_break() { d.read_break()?; break; }
                                    match __index {
                                        #(#decode_arms)*
                                        _ => { d.skip()?; }
                                    }
                                    __index += 1;
                                }
                            }
                            Ok(#construct)
                        })();
                        d.leave_depth();
                        result
                    }
                }
            })
        }
    }
}

fn expand_enum(
    input: &DeriveInput,
    container: &ContainerAttrs,
    data: &DataEnum,
    direction: Direction,
) -> syn::Result<TokenStream2> {
    let krate = krate(container);
    let variants = enum_variants(data)?;
    let has_key = variants.iter().any(|v| v.attrs.key.is_some());
    let tag_write = container.tag.map(|tag| quote!(e.tag(#tag)?;));
    let tag_read = tag_check(container, &krate);

    if container.index_only {
        for variant in &variants {
            if !variant.fields.is_empty() {
                return Err(syn::Error::new(variant.span, "`index_only` requires unit variants"));
            }
        }
        return expand_enum_index_only(input, container, &variants, direction, &krate, &tag_write, &tag_read);
    }

    if container.tagged {
        return expand_enum_tagged(input, container, &variants, direction, &krate, &tag_write, &tag_read);
    }

    if container.array {
        return expand_enum_array(input, container, &variants, direction, &krate, &tag_write, &tag_read);
    }

    expand_enum_map(input, container, &variants, direction, &krate, &tag_write, &tag_read, has_key)
}

/// Integer-keyed map keys for one enum variant's fields.
///
/// Fields default to their positional index; `#[cbor(key = N)]` overrides it.
fn variant_field_keys(fields: &Fields) -> syn::Result<Vec<WireKey>> {
    let mut keys = Vec::new();
    for (position, field) in fields.iter().enumerate() {
        let attrs = parse_field(&field.attrs)?;
        if attrs.skip {
            keys.push(WireKey::Skip(position));
        } else {
            keys.push(WireKey::Int(attrs.key.unwrap_or(position as i64)));
        }
    }
    let spans: Vec<Span> = fields.iter().map(syn::spanned::Spanned::span).collect();
    check_duplicate_keys(&keys, &spans)?;
    Ok(keys)
}

/// `#[cbor(tagged)]`: `[variant_index, {field_key: value, ...}]`.
///
/// This is the legacy map-enum encoding: a
/// two-element array holding the variant index and the variant's fields as an
/// integer-keyed map. `None` fields are omitted with `#[cbor(skip_if_none)]`.
fn expand_enum_tagged(
    input: &DeriveInput,
    container: &ContainerAttrs,
    variants: &[VariantInfo],
    direction: Direction,
    krate: &Path,
    tag_write: &Option<TokenStream2>,
    tag_read: &TokenStream2,
) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    match direction {
        Direction::Encode => {
            let (impl_generics, ty_generics, where_clause) = encode_generics(input, container, krate);
            let mut arms = Vec::new();
            for variant in variants {
                let ident = &variant.ident;
                let index = variant.index;
                let bindings = (0..variant.fields.len())
                    .map(|i| format_ident!("__f{i}"))
                    .collect::<Vec<_>>();
                let pattern = variant_pattern(ident, &variant.fields, &bindings);
                if matches!(variant.fields, Fields::Unit) {
                    arms.push(quote! {
                        #pattern => {
                            e.array(2)?;
                            e.u64(#index)?;
                            e.map(0)?;
                        }
                    });
                    continue;
                }

                let keys = variant_field_keys(&variant.fields)?;
                let order = key_order(&keys, container.declaration_order);
                let total = keys.iter().filter(|key| !matches!(key, WireKey::Skip(_))).count();
                let mut len_adjust = Vec::new();
                let mut body = Vec::new();
                for &position in &order {
                    let field = variant.fields.iter().nth(position).expect("field position");
                    let attrs = parse_field(&field.attrs)?;
                    let binding = &bindings[position];
                    if attrs.skip {
                        body.push(quote! {
                            let _ = #binding;
                        });
                        continue;
                    }
                    let key_write = keys_write(krate, &keys[position])?;
                    if attrs.skip_if_none {
                        len_adjust.push(quote! {
                            if #binding.is_none() { __len -= 1; }
                        });
                        let value_ty = option_inner(&field.ty).unwrap_or(&field.ty);
                        let encode_expr = encode_field_expr(krate, value_ty, &attrs, &quote!(__value), direction)?;
                        body.push(quote! {
                            if let ::core::option::Option::Some(__value) = #binding {
                                #key_write
                                #encode_expr
                            }
                        });
                    } else if let Some(pred) = &attrs.skip_if {
                        let encode_expr = encode_field_expr(krate, &field.ty, &attrs, &quote!(#binding), direction)?;
                        len_adjust.push(quote! {
                            if #pred(#binding) { __len -= 1; }
                        });
                        body.push(quote! {
                            if !#pred(#binding) {
                                #key_write
                                #encode_expr
                            }
                        });
                    } else {
                        let encode_expr = encode_field_expr(krate, &field.ty, &attrs, &quote!(#binding), direction)?;
                        body.push(quote! {
                            #key_write
                            #encode_expr
                        });
                    }
                }

                arms.push(quote! {
                    #pattern => {
                        let mut __len: usize = #total;
                        #(#len_adjust)*
                        e.array(2)?;
                        e.u64(#index)?;
                        e.map(__len)?;
                        #(#body)*
                    }
                });
            }
            Ok(quote! {
                impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
                    fn encode<W: #krate::Write>(&self, e: &mut #krate::Encoder<W>)
                        -> ::core::result::Result<(), W::Error>
                    {
                        #tag_write
                        match self { #(#arms)* }
                        Ok(())
                    }
                }
            })
        }
        Direction::Decode => {
            let (impl_generics, ty_generics, where_clause) = decode_generics(input, container, krate)?;
            let lifetime = decode_lifetime(input);
            let mut arms = Vec::new();
            for variant in variants {
                let ident = &variant.ident;
                let index = variant.index;
                if matches!(variant.fields, Fields::Unit) {
                    arms.push(quote! {
                        #index => {
                            d.skip()?;
                            Ok(Self::#ident)
                        }
                    });
                    continue;
                }

                let keys = variant_field_keys(&variant.fields)?;
                let all_non_negative = keys.iter().all(|key| match key {
                    WireKey::Int(value) => *value >= 0,
                    _ => true,
                });
                let key_read = if all_non_negative {
                    quote!(let __key = d.u64()?;)
                } else {
                    quote!(let __key = d.i64()?;)
                };
                let small = variant.fields.len() <= 128;
                let mut declarations = Vec::new();
                if small {
                    declarations.push(quote!(let mut __seen: u128 = 0u128;));
                } else {
                    let n = variant.fields.len();
                    declarations.push(quote!(let mut __seen = [false; #n];));
                }
                let mut map_arms = Vec::new();
                for (position, field) in variant.fields.iter().enumerate() {
                    let binding = format_ident!("__f{position}");
                    let ty = &field.ty;
                    let attrs = parse_field(&field.attrs)?;
                    declarations
                        .push(quote!(let mut #binding: ::core::option::Option<#ty> = ::core::option::Option::None;));
                    if attrs.skip {
                        continue;
                    }
                    let key_match = key_match(krate, &keys[position])?;
                    let decode_expr = decode_field_expr(krate, &field.ty, &attrs, direction)?;
                    let seen_check = if small {
                        let flag = quote!(1u128 << #position);
                        quote! {
                            if __seen & #flag != 0 { return Err(#krate::Error::duplicate_key().at(__start)); }
                            __seen |= #flag;
                        }
                    } else {
                        quote! {
                            if __seen[#position] { return Err(#krate::Error::duplicate_key().at(__start)); }
                            __seen[#position] = true;
                        }
                    };
                    map_arms.push(quote! {
                        #key_match => {
                            #seen_check
                            #binding = ::core::option::Option::Some(#decode_expr);
                        }
                    });
                }
                let construct = variant_construct(ident, &variant.fields, krate)?;
                arms.push(quote! {
                    #index => {
                        #(#declarations)*
                        match d.map()? {
                            ::core::option::Option::Some(__len) => {
                                for _ in 0..__len {
                                    #key_read
                                    match __key {
                                        #(#map_arms)*
                                        _ => { d.skip()?; }
                                    }
                                }
                            }
                            ::core::option::Option::None => loop {
                                if d.is_break() { d.read_break()?; break; }
                                #key_read
                                match __key {
                                    #(#map_arms)*
                                    _ => { d.skip()?; }
                                }
                            }
                        }
                        #construct
                    }
                });
            }
            Ok(quote! {
                impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
                    fn decode(d: &mut #krate::Decoder<#lifetime>) -> #krate::Result<Self> {
                        #tag_read
                        d.enter_depth()?;
                        let result = (|| {
                            let __start = d.position();
                            if ::core::option::Option::Some(2) != d.array()? {
                                return Err(#krate::Error::message("expected enum (2-element array)").at(__start));
                            }
                            let __index = d.u64()?;
                            match __index {
                                #(#arms)*
                                _ => Err(#krate::Error::new(#krate::ErrorKind::UnknownVariant, d.position())),
                            }
                        })();
                        d.leave_depth();
                        result
                    }
                }
            })
        }
    }
}

fn expand_enum_index_only(
    input: &DeriveInput,
    container: &ContainerAttrs,
    variants: &[VariantInfo],
    direction: Direction,
    krate: &Path,
    tag_write: &Option<TokenStream2>,
    tag_read: &TokenStream2,
) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    match direction {
        Direction::Encode => {
            let (impl_generics, ty_generics, where_clause) = encode_generics(input, container, krate);
            let arms = variants.iter().map(|v| {
                let ident = &v.ident;
                let index = v.index;
                quote!(Self::#ident => e.u64(#index)?,)
            });
            Ok(quote! {
                impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
                    fn encode<W: #krate::Write>(&self, e: &mut #krate::Encoder<W>)
                        -> ::core::result::Result<(), W::Error>
                    {
                        #tag_write
                        match self { #(#arms)* }
                        Ok(())
                    }
                }
            })
        }
        Direction::Decode => {
            let (impl_generics, ty_generics, where_clause) = decode_generics(input, container, krate)?;
            let lifetime = decode_lifetime(input);
            let arms = variants.iter().map(|v| {
                let ident = &v.ident;
                let index = v.index;
                quote!(#index => Ok(Self::#ident),)
            });
            Ok(quote! {
                impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
                    fn decode(d: &mut #krate::Decoder<#lifetime>) -> #krate::Result<Self> {
                        #tag_read
                        match d.u64()? {
                            #(#arms)*
                            _ => Err(#krate::Error::new(#krate::ErrorKind::UnknownVariant, d.position())),
                        }
                    }
                }
            })
        }
    }
}

fn expand_enum_array(
    input: &DeriveInput,
    container: &ContainerAttrs,
    variants: &[VariantInfo],
    direction: Direction,
    krate: &Path,
    tag_write: &Option<TokenStream2>,
    tag_read: &TokenStream2,
) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    match direction {
        Direction::Encode => {
            let (impl_generics, ty_generics, where_clause) = encode_generics(input, container, krate);
            let arms = variants
                .iter()
                .map(|variant| -> syn::Result<TokenStream2> {
                    let ident = &variant.ident;
                    let index = variant.index;
                    let len = 1 + variant.fields.len();
                    let bindings = (0..variant.fields.len())
                        .map(|i| format_ident!("__f{i}"))
                        .collect::<Vec<_>>();
                    let pattern = variant_pattern(ident, &variant.fields, &bindings);
                    let mut body = Vec::new();
                    for (i, field) in variant.fields.iter().enumerate() {
                        let attrs = parse_field(&field.attrs)?;
                        if attrs.skip {
                            body.push(quote!(e.null()?;));
                        } else {
                            let binding = &bindings[i];
                            body.push(quote!(#krate::Encode::encode(#binding, e)?;));
                        }
                    }
                    Ok(quote! {
                        #pattern => {
                            e.array(#len)?;
                            e.u64(#index)?;
                            #(#body)*
                        }
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! {
                impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
                    fn encode<W: #krate::Write>(&self, e: &mut #krate::Encoder<W>)
                        -> ::core::result::Result<(), W::Error>
                    {
                        #tag_write
                        match self { #(#arms)* }
                        Ok(())
                    }
                }
            })
        }
        Direction::Decode => {
            let (impl_generics, ty_generics, where_clause) = decode_generics(input, container, krate)?;
            let lifetime = decode_lifetime(input);
            let arms = variants
                .iter()
                .map(|variant| -> syn::Result<TokenStream2> {
                    let ident = &variant.ident;
                    let index = variant.index;
                    let fields = &variant.fields;
                    let mut declarations = Vec::new();
                    let mut decode_arms = Vec::new();
                    for (i, field) in fields.iter().enumerate() {
                        let binding = format_ident!("__f{i}");
                        let ty = &field.ty;
                        let attrs = parse_field(&field.attrs)?;
                        declarations.push(
                            quote!(let mut #binding: ::core::option::Option<#ty> = ::core::option::Option::None;),
                        );
                        let decode = if attrs.skip {
                            quote!(d.skip()?;)
                        } else {
                            let decode_expr = decode_field_expr(krate, &field.ty, &attrs, Direction::Decode)?;
                            quote!(#binding = ::core::option::Option::Some(#decode_expr);)
                        };
                        decode_arms.push(quote!(#i => { #decode }));
                    }
                    let construct = variant_construct(ident, fields, krate)?;
                    Ok(quote! {
                        #index => {
                            #(#declarations)*
                            let mut __i = 0usize;
                            if let ::core::option::Option::Some(__len) = __fields_len {
                                for _ in 0..__len {
                                    match __i {
                                        #(#decode_arms)*
                                        _ => { d.skip()?; }
                                    }
                                    __i += 1;
                                }
                            } else {
                                loop {
                                    if d.is_break() { d.read_break()?; break; }
                                    match __i {
                                        #(#decode_arms)*
                                        _ => { d.skip()?; }
                                    }
                                    __i += 1;
                                }
                            }
                            #construct
                        }
                    })
                })
                .collect::<syn::Result<Vec<_>>>()?;
            Ok(quote! {
                impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
                    fn decode(d: &mut #krate::Decoder<#lifetime>) -> #krate::Result<Self> {
                        #tag_read
                        d.enter_depth()?;
                        let result = (|| {
                            let __start = d.position();
                            let __array = d.array()?;
                            if __array == ::core::option::Option::Some(0) {
                                return Err(#krate::Error::message("enum array is missing the variant index").at(__start));
                            }
                            let __index = d.u64()?;
                            let __fields_len = __array.map(|n| n.saturating_sub(1));
                            match __index {
                                #(#arms)*
                                _ => Err(#krate::Error::new(#krate::ErrorKind::UnknownVariant, d.position())),
                            }
                        })();
                        d.leave_depth();
                        result
                    }
                }
            })
        }
    }
}

fn expand_enum_map(
    input: &DeriveInput,
    container: &ContainerAttrs,
    variants: &[VariantInfo],
    direction: Direction,
    krate: &Path,
    tag_write: &Option<TokenStream2>,
    tag_read: &TokenStream2,
    has_key: bool,
) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let keys = variant_keys(container, variants, has_key)?;
    let external = !has_key;
    match direction {
        Direction::Encode => {
            let (impl_generics, ty_generics, where_clause) = encode_generics(input, container, krate);
            let order = key_order(&keys, container.declaration_order);
            let mut arms = Vec::new();
            for &index in &order {
                let variant = &variants[index];
                let key = &keys[index];
                let ident = &variant.ident;
                let bindings = (0..variant.fields.len())
                    .map(|i| format_ident!("__f{i}"))
                    .collect::<Vec<_>>();
                let pattern = variant_pattern(ident, &variant.fields, &bindings);

                if external && matches!(variant.fields, Fields::Unit) {
                    let WireKey::Text(name) = key else {
                        return Err(syn::Error::new(variant.span, "unit variants need a text key"));
                    };
                    arms.push(quote! {
                        #pattern => { e.text(#name)?; }
                    });
                    continue;
                }

                let key_write = match key {
                    WireKey::Text(name) => quote!(e.text(#name)?;),
                    WireKey::Int(value) if *value >= 0 => {
                        let value = *value as u64;
                        quote!(e.u64(#value)?;)
                    }
                    WireKey::Int(value) => quote!(e.i64(#value)?;),
                    WireKey::Skip(_) => TokenStream2::new(),
                };
                let payload = match &variant.fields {
                    Fields::Unit => quote!(e.null()?;),
                    Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                        quote!(#krate::Encode::encode(__f0, e)?;)
                    }
                    Fields::Unnamed(fields) => {
                        let len = fields.unnamed.len();
                        let body = bindings.iter().map(|b| quote!(#krate::Encode::encode(#b, e)?;));
                        quote! {
                            e.array(#len)?;
                            #(#body)*
                        }
                    }
                    Fields::Named(fields) => {
                        let mut body = Vec::new();
                        for (i, field) in fields.named.iter().enumerate() {
                            let attrs = parse_field(&field.attrs)?;
                            if attrs.skip {
                                continue;
                            }
                            let field_name = attrs
                                .rename
                                .clone()
                                .or_else(|| field.ident.as_ref().map(unraw))
                                .unwrap();
                            let field_name = if attrs.rename.is_some() {
                                field_name
                            } else {
                                container.rename_all.map(|r| r.apply(&field_name)).unwrap_or(field_name)
                            };
                            let binding = &bindings[i];
                            body.push(quote! {
                                e.text(#field_name)?;
                                #krate::Encode::encode(#binding, e)?;
                            });
                        }
                        let len = body.len();
                        quote! {
                            e.map(#len)?;
                            #(#body)*
                        }
                    }
                };
                arms.push(quote! {
                    #pattern => {
                        e.map(1)?;
                        #key_write
                        #payload
                    }
                });
            }
            Ok(quote! {
                impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
                    fn encode<W: #krate::Write>(&self, e: &mut #krate::Encoder<W>)
                        -> ::core::result::Result<(), W::Error>
                    {
                        #tag_write
                        match self { #(#arms)* }
                        Ok(())
                    }
                }
            })
        }
        Direction::Decode => {
            let (impl_generics, ty_generics, where_clause) = decode_generics(input, container, krate)?;
            let lifetime = decode_lifetime(input);
            let mut unit_arms = Vec::new();
            let mut map_arms = Vec::new();
            for (variant, key) in variants.iter().zip(keys.iter()) {
                let ident = &variant.ident;
                if external && matches!(variant.fields, Fields::Unit) {
                    let WireKey::Text(name) = key else {
                        return Err(syn::Error::new(variant.span, "unit variants need a text key"));
                    };
                    unit_arms.push(quote!(#name => Ok(Self::#ident),));
                    continue;
                }
                let key_match = match key {
                    WireKey::Text(name) => quote!(#name),
                    WireKey::Int(value) if *value >= 0 => {
                        let value = *value as u64;
                        quote!(#value)
                    }
                    WireKey::Int(value) => quote!(#value),
                    WireKey::Skip(_) => quote!(_),
                };
                let decode_payload = decode_variant_payload(variant, krate, container)?;
                map_arms.push(quote! {
                    #key_match => { #decode_payload }
                });
            }

            let map_check = quote! {
                let __pair = d.map()?;
                if let ::core::option::Option::Some(__len) = __pair {
                    if __len != 1 {
                        return Err(#krate::Error::message("expected a one-entry map").at(__start));
                    }
                } else if d.is_break() {
                    return Err(#krate::Error::message("expected a one-entry map").at(__start));
                }
            };
            let map_finish = quote! {
                if __pair.is_none() {
                    if !d.is_break() {
                        return Err(#krate::Error::message("expected one map entry").at(d.position()));
                    }
                    d.read_break()?;
                }
            };

            let all_non_negative = keys.iter().all(|key| match key {
                WireKey::Int(value) => *value >= 0,
                _ => true,
            });
            let dispatch = if external {
                quote! {
                    match d.datatype()? {
                        #krate::Type::Text => {
                            let __key = d.text()?;
                            match __key {
                                #(#unit_arms)*
                                _ => Err(#krate::Error::new(#krate::ErrorKind::UnknownVariant, d.position())),
                            }
                        }
                        #krate::Type::Map | #krate::Type::MapIndef => {
                            #map_check
                            let __key = d.text()?;
                            let __result = match __key {
                                #(#map_arms)*
                                _ => Err(#krate::Error::new(#krate::ErrorKind::UnknownVariant, d.position())),
                            };
                            #map_finish
                            __result
                        }
                        _ => Err(#krate::Error::message("expected enum text or map").at(__start)),
                    }
                }
            } else if all_non_negative {
                quote! {
                    #map_check
                    let __key = d.u64()?;
                    let __result = match __key {
                        #(#map_arms)*
                        _ => Err(#krate::Error::new(#krate::ErrorKind::UnknownVariant, d.position())),
                    };
                    #map_finish
                    __result
                }
            } else {
                quote! {
                    #map_check
                    let __key = d.i64()?;
                    let __result = match __key {
                        #(#map_arms)*
                        _ => Err(#krate::Error::new(#krate::ErrorKind::UnknownVariant, d.position())),
                    };
                    #map_finish
                    __result
                }
            };

            Ok(quote! {
                impl #impl_generics #krate::Decode<#lifetime> for #name #ty_generics #where_clause {
                    fn decode(d: &mut #krate::Decoder<#lifetime>) -> #krate::Result<Self> {
                        #tag_read
                        d.enter_depth()?;
                        let result = (|| {
                            let __start = d.position();
                            #dispatch
                        })();
                        d.leave_depth();
                        result
                    }
                }
            })
        }
    }
}

fn decode_variant_payload(
    variant: &VariantInfo,
    krate: &Path,
    container: &ContainerAttrs,
) -> syn::Result<TokenStream2> {
    let ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => Ok(quote! {
            if d.peek_byte()? == 0xf6 { d.null()?; } else { d.skip()?; }
            Ok(Self::#ident)
        }),
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let field = fields.unnamed.first().unwrap();
            let attrs = parse_field(&field.attrs)?;
            let decode_expr = decode_field_expr(krate, &field.ty, &attrs, Direction::Decode)?;
            Ok(quote!(Ok(Self::#ident(#decode_expr))))
        }
        Fields::Unnamed(fields) => {
            let mut declarations = Vec::new();
            let mut decode_arms = Vec::new();
            for (i, field) in fields.unnamed.iter().enumerate() {
                let binding = format_ident!("__f{i}");
                let ty = &field.ty;
                let attrs = parse_field(&field.attrs)?;
                declarations
                    .push(quote!(let mut #binding: ::core::option::Option<#ty> = ::core::option::Option::None;));
                let decode = if attrs.skip {
                    quote!(d.skip()?;)
                } else {
                    let decode_expr = decode_field_expr(krate, &field.ty, &attrs, Direction::Decode)?;
                    quote!(#binding = ::core::option::Option::Some(#decode_expr);)
                };
                decode_arms.push(quote!(#i => { #decode }));
            }
            let construct = variant_construct(ident, &variant.fields, krate)?;
            Ok(quote! {
                #(#declarations)*
                let mut __i = 0usize;
                match d.array()? {
                    ::core::option::Option::Some(__len) => {
                        for _ in 0..__len {
                            match __i {
                                #(#decode_arms)*
                                _ => { d.skip()?; }
                            }
                            __i += 1;
                        }
                    }
                    ::core::option::Option::None => loop {
                        if d.is_break() { d.read_break()?; break; }
                        match __i {
                            #(#decode_arms)*
                            _ => { d.skip()?; }
                        }
                        __i += 1;
                    }
                }
                #construct
            })
        }
        Fields::Named(fields) => {
            let mut declarations = Vec::new();
            let mut decode_arms = Vec::new();
            let mut construct_fields = Vec::new();
            if fields.named.len() <= 128 {
                declarations.push(quote!(let mut __seen: u128 = 0u128;));
            } else {
                let n = fields.named.len();
                declarations.push(quote!(let mut __seen = [false; #n];));
            }
            for (i, field) in fields.named.iter().enumerate() {
                let field_ident = field.ident.as_ref().unwrap();
                let binding = format_ident!("__f{i}");
                let ty = &field.ty;
                let attrs = parse_field(&field.attrs)?;
                declarations
                    .push(quote!(let mut #binding: ::core::option::Option<#ty> = ::core::option::Option::None;));
                let name = attrs.rename.clone().or_else(|| Some(unraw(field_ident))).unwrap();
                let name = if attrs.rename.is_some() {
                    name
                } else {
                    container.rename_all.map(|r| r.apply(&name)).unwrap_or(name)
                };
                if !attrs.skip {
                    let decode_expr = decode_field_expr(krate, &field.ty, &attrs, Direction::Decode)?;
                    let seen_check = if fields.named.len() <= 128 {
                        let flag = quote!(1u128 << #i);
                        quote! {
                            if __seen & #flag != 0 { return Err(#krate::Error::duplicate_key().at(__start)); }
                            __seen |= #flag;
                        }
                    } else {
                        quote! {
                            if __seen[#i] { return Err(#krate::Error::duplicate_key().at(__start)); }
                            __seen[#i] = true;
                        }
                    };
                    decode_arms.push(quote! {
                        #name => {
                            #seen_check
                            #binding = ::core::option::Option::Some(#decode_expr);
                        }
                    });
                }
                let value = if attrs.skip {
                    quote!(::core::default::Default::default())
                } else if attrs.default.is_some() || attrs.skip_if.is_some() {
                    quote!(#binding.unwrap_or_default())
                } else if is_option(&field.ty) {
                    quote!(#binding.unwrap_or(::core::option::Option::None))
                } else {
                    quote!(match #binding {
                        ::core::option::Option::Some(v) => v,
                        ::core::option::Option::None => return Err(#krate::Error::missing_field(stringify!(#field_ident)).at(d.position())),
                    })
                };
                construct_fields.push(quote!(#field_ident: #value));
            }
            let unknown_arm = if container.deny_unknown_fields {
                quote!(_ => { return Err(#krate::Error::message("unknown field").at(d.position())); })
            } else {
                quote!(_ => { d.skip()?; })
            };
            Ok(quote! {
                #(#declarations)*
                let mut __i = 0usize;
                match d.map()? {
                    ::core::option::Option::Some(__len) => {
                        for _ in 0..__len {
                            let __key = d.text()?;
                            match __key {
                                #(#decode_arms)*
                                #unknown_arm
                            }
                            __i += 1;
                        }
                    }
                    ::core::option::Option::None => loop {
                        if d.is_break() { d.read_break()?; break; }
                        let __key = d.text()?;
                        match __key {
                            #(#decode_arms)*
                            #unknown_arm
                        }
                        __i += 1;
                    }
                }
                let _ = __i;
                Ok(Self::#ident { #(#construct_fields),* })
            })
        }
    }
}

fn struct_fields(data: &DataStruct) -> syn::Result<Vec<FieldInfo>> {
    let mut out = Vec::new();
    for (position, field) in data.fields.iter().enumerate() {
        let attrs = parse_field(&field.attrs)?;
        validate_field(&attrs, field)?;
        out.push(FieldInfo {
            ident: field.ident.clone(),
            ty: field.ty.clone(),
            attrs,
            position,
            span: field.span(),
        });
    }
    Ok(out)
}

fn validate_field(attrs: &FieldAttrs, field: &Field) -> syn::Result<()> {
    if attrs.key.is_some() && attrs.index.is_some() {
        return Err(syn::Error::new(field.span(), "`key` and `index` are mutually exclusive"));
    }
    if attrs.bytes && attrs.with.is_some() {
        return Err(syn::Error::new(field.span(), "`bytes` and `with` are mutually exclusive"));
    }
    if attrs.skip && (attrs.default.is_some() || attrs.skip_if_none || attrs.skip_if.is_some()) {
        return Err(syn::Error::new(
            field.span(),
            "`skip` cannot be combined with `default`, `skip_if_none` or `skip_if`",
        ));
    }
    Ok(())
}

struct FieldInfo {
    ident: Option<Ident>,
    ty: Type,
    attrs: FieldAttrs,
    position: usize,
    span: Span,
}

fn field_ident(field: &FieldInfo, index: usize) -> Ident {
    match &field.ident {
        Some(ident) => format_ident!("__field_{}", ident),
        None => format_ident!("__field_{index}"),
    }
}

fn field_access(field: &FieldInfo) -> TokenStream2 {
    match &field.ident {
        Some(ident) => quote!(self.#ident),
        None => {
            let index = syn::Index::from(field.position);
            quote!(self.#index)
        }
    }
}

fn struct_construct(fields: &[FieldInfo], _container: &ContainerAttrs, krate: &Path) -> syn::Result<TokenStream2> {
    let mut values = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        let binding = field_ident(field, index);
        let name = field
            .attrs
            .rename
            .clone()
            .or_else(|| field.ident.as_ref().map(unraw))
            .unwrap_or_else(|| index.to_string());
        let value = if field.attrs.skip {
            quote!(::core::default::Default::default())
        } else if let Some(default) = &field.attrs.default {
            match default {
                Some(path) => quote!(#binding.unwrap_or_else(|| #path())),
                None => quote!(#binding.unwrap_or_default()),
            }
        } else if field.attrs.skip_if_none {
            quote!(#binding.unwrap_or(::core::option::Option::None))
        } else if field.attrs.skip_if.is_some() {
            quote!(#binding.unwrap_or_default())
        } else if is_option(&field.ty) {
            quote!(#binding.unwrap_or(::core::option::Option::None))
        } else {
            let missing = quote!(#krate::Error::missing_field(#name));
            quote!(match #binding {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => return Err(#missing.at(__start)),
            })
        };
        if let Some(ident) = &field.ident {
            values.push(quote!(#ident: #value));
        } else {
            values.push(value);
        }
    }
    if fields.iter().any(|f| f.ident.is_none()) {
        Ok(quote!(Self(#(#values),*)))
    } else {
        Ok(quote!(Self { #(#values),* }))
    }
}

fn is_option(ty: &Type) -> bool {
    if let Type::Path(path) = ty {
        path.path.segments.last().map(|s| s.ident == "Option").unwrap_or(false)
    } else {
        false
    }
}

/// The `T` in `Option<T>`, when `ty` is an `Option`.
fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    args.args.iter().find_map(|arg| match arg {
        syn::GenericArgument::Type(ty) => Some(ty),
        _ => None,
    })
}

/// Build the encoding expression for one field.
///
/// `value` must be an expression of type `&T` (a borrow of the field).
fn encode_field_expr(
    krate: &Path,
    ty: &Type,
    attrs: &FieldAttrs,
    value: &TokenStream2,
    direction: Direction,
) -> syn::Result<TokenStream2> {
    let _ = direction;
    if attrs.bytes {
        if is_option(ty) {
            return Ok(quote! {
                match #value {
                    ::core::option::Option::Some(__bytes) => {
                        e.bytes(::core::convert::AsRef::<[u8]>::as_ref(__bytes))?;
                    }
                    ::core::option::Option::None => {
                        e.null()?;
                    }
                }
            });
        }
        return Ok(quote! {
            e.bytes(::core::convert::AsRef::<[u8]>::as_ref(#value))?;
        });
    }
    if let Some(path) = &attrs.encode_with {
        return Ok(quote!(#path(#value, e)?;));
    }
    if let Some(path) = &attrs.with {
        return Ok(quote!(#path::encode(#value, e)?;));
    }
    Ok(quote! {
        #krate::Encode::encode(#value, e)?;
    })
}

fn decode_field_expr(krate: &Path, ty: &Type, attrs: &FieldAttrs, _direction: Direction) -> syn::Result<TokenStream2> {
    if attrs.bytes {
        return Ok(quote!(#krate::DecodeBytes::decode_bytes(d)?));
    }
    if let Some(path) = &attrs.decode_with {
        return Ok(quote!(#path(d)?));
    }
    if let Some(path) = &attrs.with {
        return Ok(quote!(#path::decode(d)?));
    }
    Ok(quote!(<#ty as #krate::Decode>::decode(d)?))
}

#[derive(Clone)]
enum WireKey {
    Text(String),
    Int(i64),
    Skip(usize),
}

fn map_keys(container: &ContainerAttrs, fields: &[FieldInfo]) -> syn::Result<Vec<WireKey>> {
    let has_key = fields.iter().any(|f| f.attrs.key.is_some());
    let mut keys = Vec::new();
    for (index, field) in fields.iter().enumerate() {
        if field.attrs.skip {
            keys.push(WireKey::Skip(index));
            continue;
        }
        if has_key {
            let key = field
                .attrs
                .key
                .ok_or_else(|| syn::Error::new(field.span, "all fields need `key = N` in an integer-keyed map"))?;
            keys.push(WireKey::Int(key));
        } else {
            let name = field
                .attrs
                .rename
                .clone()
                .or_else(|| field.ident.as_ref().map(unraw))
                .ok_or_else(|| syn::Error::new(field.span, "a tuple field needs `key = N` or `rename = \"...\"`"))?;
            let name = if field.attrs.rename.is_some() {
                name
            } else {
                container.rename_all.map(|r| r.apply(&name)).unwrap_or(name)
            };
            keys.push(WireKey::Text(name));
        }
    }
    check_duplicate_keys(&keys, &fields.iter().map(|f| f.span).collect::<Vec<_>>())?;
    Ok(keys)
}

fn key_order(keys: &[WireKey], declaration_order: bool) -> Vec<usize> {
    let mut order: Vec<usize> = (0..keys.len()).collect();
    if !declaration_order {
        let encoded: Vec<Vec<u8>> = keys.iter().map(encoded_key).collect();
        order.sort_by(|&a, &b| encoded[a].cmp(&encoded[b]));
    }
    order
}

fn variant_keys(container: &ContainerAttrs, variants: &[VariantInfo], has_key: bool) -> syn::Result<Vec<WireKey>> {
    let mut keys = Vec::new();
    for variant in variants {
        if has_key {
            let key = variant
                .attrs
                .key
                .ok_or_else(|| syn::Error::new(variant.span, "all variants need `key = N` in an integer-keyed enum"))?;
            keys.push(WireKey::Int(key));
        } else {
            let name = variant.attrs.rename.clone().unwrap_or_else(|| unraw(&variant.ident));
            let name = if variant.attrs.rename.is_some() {
                name
            } else {
                container.rename_all.map(|r| r.apply(&name)).unwrap_or(name)
            };
            keys.push(WireKey::Text(name));
        }
    }
    check_duplicate_keys(&keys, &variants.iter().map(|v| v.span).collect::<Vec<_>>())?;
    Ok(keys)
}

fn check_duplicate_keys(keys: &[WireKey], spans: &[Span]) -> syn::Result<()> {
    let encoded: Vec<Option<Vec<u8>>> = keys
        .iter()
        .map(|key| match key {
            WireKey::Skip(_) => None,
            _ => Some(encoded_key(key)),
        })
        .collect();
    for i in 0..encoded.len() {
        let Some(a) = &encoded[i] else { continue };
        for j in (i + 1)..encoded.len() {
            let Some(b) = &encoded[j] else { continue };
            if a == b {
                return Err(syn::Error::new(spans[j], "duplicate CBOR map key"));
            }
        }
    }
    Ok(())
}

fn encoded_key(key: &WireKey) -> Vec<u8> {
    match key {
        WireKey::Text(name) => {
            let mut out = encode_head(0x60, name.len() as u64);
            out.extend_from_slice(name.as_bytes());
            out
        }
        WireKey::Int(value) => {
            if *value >= 0 {
                encode_head(0x00, *value as u64)
            } else {
                encode_head(0x20, (-1 - *value) as u64)
            }
        }
        WireKey::Skip(index) => vec![0xff, *index as u8, 0xff],
    }
}

fn encode_head(major: u8, value: u64) -> Vec<u8> {
    if value < 24 {
        vec![major | value as u8]
    } else if value <= u8::MAX as u64 {
        vec![major | 24, value as u8]
    } else if value <= u16::MAX as u64 {
        let mut out = vec![major | 25];
        out.extend_from_slice(&(value as u16).to_be_bytes());
        out
    } else if value <= u32::MAX as u64 {
        let mut out = vec![major | 26];
        out.extend_from_slice(&(value as u32).to_be_bytes());
        out
    } else {
        let mut out = vec![major | 27];
        out.extend_from_slice(&value.to_be_bytes());
        out
    }
}

fn keys_write(krate: &Path, key: &WireKey) -> syn::Result<TokenStream2> {
    let _ = krate;
    Ok(match key {
        WireKey::Text(name) => quote!(e.text(#name)?;),
        WireKey::Int(value) if *value >= 0 => {
            let value = *value as u64;
            quote!(e.u64(#value)?;)
        }
        WireKey::Int(value) => quote!(e.i64(#value)?;),
        WireKey::Skip(_) => TokenStream2::new(),
    })
}

fn key_match(krate: &Path, key: &WireKey) -> syn::Result<TokenStream2> {
    let _ = krate;
    Ok(match key {
        WireKey::Text(name) => quote!(#name),
        WireKey::Int(value) if *value >= 0 => {
            let value = *value as u64;
            quote!(#value)
        }
        WireKey::Int(value) => quote!(#value),
        WireKey::Skip(_) => quote!(_),
    })
}

fn array_indices(fields: &[FieldInfo]) -> syn::Result<Vec<usize>> {
    let mut indices = Vec::new();
    for field in fields {
        let index = field.attrs.index.unwrap_or(field.position);
        if indices.contains(&index) {
            return Err(syn::Error::new(field.span, "duplicate array index"));
        }
        indices.push(index);
    }
    Ok(indices)
}

struct VariantInfo {
    ident: Ident,
    index: u64,
    attrs: FieldAttrs,
    fields: Fields,
    span: Span,
}

fn enum_variants(data: &DataEnum) -> syn::Result<Vec<VariantInfo>> {
    let mut out = Vec::new();
    for (position, variant) in data.variants.iter().enumerate() {
        let attrs = parse_field(&variant.attrs)?;
        let index = attrs.index.map(|i| i as u64).unwrap_or(position as u64);
        if out.iter().any(|v: &VariantInfo| v.index == index) {
            return Err(syn::Error::new(variant.span(), "duplicate variant index"));
        }
        out.push(VariantInfo {
            ident: variant.ident.clone(),
            index,
            attrs,
            fields: variant.fields.clone(),
            span: variant.span(),
        });
    }
    Ok(out)
}

fn variant_pattern(ident: &Ident, fields: &Fields, bindings: &[Ident]) -> TokenStream2 {
    match fields {
        Fields::Unit => quote!(Self::#ident),
        Fields::Unnamed(_) => quote!(Self::#ident(#(#bindings),*)),
        Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            let bindings = bindings.iter();
            quote!(Self::#ident { #(#names: #bindings),* })
        }
    }
}

fn variant_construct(ident: &Ident, fields: &Fields, krate: &Path) -> syn::Result<TokenStream2> {
    let mut values = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let binding = format_ident!("__f{i}");
        let attrs = parse_field(&field.attrs)?;
        let value = if attrs.skip {
            quote!(::core::default::Default::default())
        } else if let Some(default) = &attrs.default {
            match default {
                Some(path) => quote!(#binding.unwrap_or_else(|| #path())),
                None => quote!(#binding.unwrap_or_default()),
            }
        } else if is_option(&field.ty) {
            quote!(#binding.unwrap_or(::core::option::Option::None))
        } else {
            quote!(match #binding {
                ::core::option::Option::Some(value) => value,
                ::core::option::Option::None => return Err(#krate::Error::missing_field("variant field").at(__start)),
            })
        };
        values.push(value);
    }
    Ok(match fields {
        Fields::Unit => quote!(Ok(Self::#ident)),
        Fields::Unnamed(_) => quote!(Ok(Self::#ident(#(#values),*))),
        Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            let values = values.iter();
            quote!(Ok(Self::#ident { #(#names: #values),* }))
        }
    })
}

fn krate(container: &ContainerAttrs) -> Path {
    container.krate.clone().unwrap_or_else(|| syn::parse_quote!(::cbor))
}

fn decode_lifetime(input: &DeriveInput) -> syn::Lifetime {
    let mut name = String::from("__cbor");
    loop {
        let candidate = syn::Lifetime::new(&format!("'{name}"), Span::call_site());
        if !input.generics.lifetimes().any(|l| l.lifetime.ident == name) {
            return candidate;
        }
        name.push('_');
    }
}

fn encode_generics(
    input: &DeriveInput,
    container: &ContainerAttrs,
    krate: &Path,
) -> (TokenStream2, TokenStream2, TokenStream2) {
    let mut generics = input.generics.clone();
    if let Some(bound) = &container.bound {
        let predicates = parse_bounds(bound);
        generics.make_where_clause().predicates.extend(predicates);
    } else {
        let idents: Vec<Ident> = generics.type_params().map(|p| p.ident.clone()).collect();
        for ident in idents {
            generics
                .make_where_clause()
                .predicates
                .push(syn::parse_quote!(#ident: #krate::Encode));
        }
    }
    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    (
        quote!(#impl_generics),
        quote!(#ty_generics),
        where_clause.map(|w| quote!(#w)).unwrap_or_default(),
    )
}

fn decode_generics(
    input: &DeriveInput,
    container: &ContainerAttrs,
    krate: &Path,
) -> syn::Result<(TokenStream2, TokenStream2, TokenStream2)> {
    let lifetime = decode_lifetime(input);
    let mut generics = input.generics.clone();
    generics
        .params
        .insert(0, GenericParam::Lifetime(syn::LifetimeParam::new(lifetime.clone())));
    let borrows = fields_borrow(input);
    if let Some(bound) = &container.bound {
        let predicates = parse_bounds(bound);
        generics.make_where_clause().predicates.extend(predicates);
    } else {
        for param in input.generics.type_params() {
            let ident = &param.ident;
            generics
                .make_where_clause()
                .predicates
                .push(syn::parse_quote!(#ident: #krate::Decode<#lifetime>));
        }
        if borrows {
            for param in input.generics.lifetimes() {
                let lt = &param.lifetime;
                generics
                    .make_where_clause()
                    .predicates
                    .push(syn::parse_quote!(#lifetime: #lt));
            }
        }
    }
    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    Ok((
        quote!(#impl_generics),
        quote!(#ty_generics),
        where_clause.map(|w| quote!(#w)).unwrap_or_default(),
    ))
}

fn parse_bounds(bound: &str) -> Punctuated<WherePredicate, Token![,]> {
    if bound.trim().is_empty() {
        return Punctuated::new();
    }
    syn::parse_str::<WherePredicateList>(bound)
        .map(|list| list.0)
        .unwrap_or_else(|_| Punctuated::new())
}

struct WherePredicateList(Punctuated<WherePredicate, Token![,]>);

impl Parse for WherePredicateList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(WherePredicateList(Punctuated::parse_terminated(input)?))
    }
}

fn fields_borrow(input: &DeriveInput) -> bool {
    struct Visitor {
        found: bool,
    }
    impl<'ast> Visit<'ast> for Visitor {
        fn visit_type_reference(&mut self, _: &'ast syn::TypeReference) {
            self.found = true;
        }
    }
    let mut visitor = Visitor {
        found: false,
    };
    match &input.data {
        Data::Struct(data) => {
            for field in &data.fields {
                if parse_field(&field.attrs).map(|a| a.borrow).unwrap_or(false) {
                    return true;
                }
                visitor.visit_type(&field.ty);
            }
        }
        Data::Enum(data) => {
            for variant in &data.variants {
                for field in &variant.fields {
                    if parse_field(&field.attrs).map(|a| a.borrow).unwrap_or(false) {
                        return true;
                    }
                    visitor.visit_type(&field.ty);
                }
            }
        }
        Data::Union(_) => {}
    }
    visitor.found
}

fn tag_check(container: &ContainerAttrs, _krate: &Path) -> TokenStream2 {
    match container.tag {
        Some(tag) if container.tag_optional => quote! {
            if d.peek_byte()? & 0xe0 == 0xc0 {
                d.tag_expected(#tag)?;
            }
        },
        Some(tag) => quote!(d.tag_expected(#tag)?;),
        None => TokenStream2::new(),
    }
}

fn unraw(ident: &Ident) -> String {
    ident.to_string().trim_start_matches("r#").to_string()
}
