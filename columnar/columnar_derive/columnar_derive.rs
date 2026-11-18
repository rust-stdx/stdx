//! Derive macros for the `columnar` crate.
//!
//! `#[derive(Columnar)]` implements `columnar::Columnar`,
//! `columnar::ColumnarDecode`, `columnar::Field` and `columnar::FieldDecode`.
//! The field traits let a derived type be nested inside another derived type.
//! Adding `#[columnar(iterable)]` to the container also implements
//! `columnar::ColumnarIter` and `columnar::FieldIter` for lazy row iteration.
//!
//! Struct fields accept:
//!
//! - `#[columnar(key = N)]`
//! - `#[columnar(encoding = "name")]`
//! - `#[columnar(skip)]`
//! - `#[columnar(default)]` / `#[columnar(default = "path")]`
//!
//! The container accepts `#[columnar(crate = "...")]` to override the crate
//! path used by generated code, `#[columnar(borrow = "'a")]` to select the
//! decode lifetime for a type with multiple lifetimes, and
//! `#[columnar(iterable)]` to generate lazy row iterators.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam, Generics, Ident, Index, Lifetime,
    LifetimeParam, LitInt, LitStr, Path, Token, Type, parse_quote, spanned::Spanned,
};

#[derive(Default)]
struct FieldAttrs {
    key: Option<u64>,
    encoding: Option<TokenStream2>,
    skip: bool,
    default: Option<Option<Path>>,
}

struct StructField {
    name: String,
    access: TokenStream2,
    ty: Type,
    attrs: FieldAttrs,
    key: Option<u64>,
    init: FieldInit,
}

enum FieldInit {
    Named(Ident),
    Tuple,
}

struct VariantField {
    binding: Ident,
    ty: Type,
    attrs: FieldAttrs,
    pattern_name: Option<Ident>,
}

struct Variant {
    ident: Ident,
    named: bool,
    fields: Vec<VariantField>,
}

struct ContainerAttrs {
    krate: Path,
    borrow: Option<Lifetime>,
    iterable: bool,
}

struct ImplGenerics {
    encode: Generics,
    decode: Generics,
    decode_lifetime: Lifetime,
}

#[proc_macro_derive(Columnar, attributes(columnar))]
pub fn derive_columnar(input: TokenStream) -> TokenStream {
    match expand(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand(input: TokenStream2) -> syn::Result<TokenStream2> {
    let input: DeriveInput = syn::parse2(input)?;
    let container = parse_container(&input.attrs)?;
    match &input.data {
        Data::Struct(data) => expand_struct(&input, data, &container),
        Data::Enum(data) => expand_enum(&input, data, &container),
        Data::Union(_) => Err(syn::Error::new(input.ident.span(), "Columnar cannot be derived for unions")),
    }
}

fn parse_container(attrs: &[Attribute]) -> syn::Result<ContainerAttrs> {
    let mut out = ContainerAttrs {
        krate: parse_quote!(::columnar),
        borrow: None,
        iterable: false,
    };
    for attr in attrs {
        if !attr.path().is_ident("columnar") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let value = meta.value()?;
                if let Ok(lit) = value.parse::<LitStr>() {
                    out.krate = lit.parse()?;
                } else {
                    out.krate = value.parse()?;
                }
                Ok(())
            } else if meta.path.is_ident("borrow") {
                let lit: LitStr = meta.value()?.parse()?;
                out.borrow = Some(lit.parse()?);
                Ok(())
            } else if meta.path.is_ident("iterable") {
                out.iterable = true;
                Ok(())
            } else {
                Err(meta.error("unknown columnar container attribute"))
            }
        })?;
    }
    Ok(out)
}

fn parse_field_attrs(attrs: &[Attribute], krate: &Path) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("columnar") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("key") {
                let lit: LitInt = meta.value()?.parse()?;
                out.key = Some(lit.base10_parse()?);
                Ok(())
            } else if meta.path.is_ident("encoding") {
                let lit: LitStr = meta.value()?.parse()?;
                out.encoding = Some(encoding_expr(krate, &lit)?);
                Ok(())
            } else if meta.path.is_ident("skip") {
                out.skip = true;
                Ok(())
            } else if meta.path.is_ident("default") {
                if meta.input.peek(Token![=]) {
                    let value = meta.value()?;
                    let lit: LitStr = value.parse()?;
                    let path: Path = lit.parse()?;
                    out.default = Some(Some(path));
                } else {
                    out.default = Some(None);
                }
                Ok(())
            } else {
                Err(meta.error("unknown columnar field attribute"))
            }
        })?;
    }
    Ok(out)
}

fn encoding_expr(krate: &Path, lit: &LitStr) -> syn::Result<TokenStream2> {
    let variant = match lit.value().as_str() {
        "auto" => "Auto",
        "packed" => "Packed",
        "for" | "for_bitpack" => "ForBitpack",
        "bitpack" => "Bitpack",
        "varint" => "Varint",
        "rle" => "Rle",
        "delta_rle" => "DeltaRle",
        "delta_of_delta" => "DeltaOfDelta",
        "dictionary" | "dict" => "Dictionary",
        "lengths" | "string_lengths" => "StringLengths",
        "offsets" | "string_offsets" => "StringOffsets",
        "nullable" => "Nullable",
        "list_lengths" => "ListLengths",
        "list_offsets" => "ListOffsets",
        "struct" => "Struct",
        "enum" => "Enum",
        "all_null" => "AllNull",
        other => {
            return Err(syn::Error::new(lit.span(), format!("unknown columnar encoding `{other}`")));
        }
    };
    let ident = Ident::new(variant, lit.span());
    Ok(quote!(#krate::Encoding::#ident))
}

fn default_expr(attrs: &FieldAttrs) -> TokenStream2 {
    match &attrs.default {
        Some(Some(path)) => quote!(#path()),
        Some(None) | None => quote!(::core::default::Default::default()),
    }
}

fn assign_keys(fields: &mut [StructField]) -> syn::Result<()> {
    let mut used = Vec::new();
    for field in fields.iter() {
        if let Some(key) = field.attrs.key {
            if key == 0 {
                return Err(syn::Error::new(
                    field.ty.span(),
                    "columnar key 0 is reserved for the envelope header",
                ));
            }
            if used.contains(&key) {
                return Err(syn::Error::new(field.ty.span(), format!("duplicate columnar key {key}")));
            }
            used.push(key);
        }
    }
    let mut next = 1u64;
    for field in fields.iter_mut() {
        if field.attrs.skip {
            if field.attrs.key.is_some() {
                return Err(syn::Error::new(field.ty.span(), "a skipped field cannot have a columnar key"));
            }
            continue;
        }
        if let Some(key) = field.attrs.key {
            field.key = Some(key);
        } else {
            while used.contains(&next) {
                next += 1;
            }
            field.key = Some(next);
            used.push(next);
            next += 1;
        }
    }
    Ok(())
}

fn build_impl_generics(
    input: &DeriveInput,
    krate: &Path,
    types: impl Iterator<Item = Type>,
    container: &ContainerAttrs,
) -> syn::Result<ImplGenerics> {
    let types: Vec<Type> = types.collect();
    let mut encode = input.generics.clone();
    {
        let where_clause = encode.make_where_clause();
        for ty in &types {
            where_clause.predicates.push(parse_quote!(#ty: #krate::Field));
        }
    }

    let decode_lifetime = if let Some(lifetime) = &container.borrow {
        if !input.generics.lifetimes().any(|param| param.lifetime == *lifetime) {
            return Err(syn::Error::new(
                lifetime.span(),
                "borrow lifetime is not a parameter of the type",
            ));
        }
        lifetime.clone()
    } else {
        let lifetimes: Vec<Lifetime> = input.generics.lifetimes().map(|param| param.lifetime.clone()).collect();
        match lifetimes.as_slice() {
            [] => Lifetime::new("'__columnar", proc_macro2::Span::call_site()),
            [lifetime] => lifetime.clone(),
            _ => {
                return Err(syn::Error::new(
                    input.ident.span(),
                    "multiple lifetimes require #[columnar(borrow = \"'a\")]",
                ));
            }
        }
    };

    let mut decode = input.generics.clone();
    if !decode.lifetimes().any(|param| param.lifetime == decode_lifetime) {
        decode
            .params
            .insert(0, GenericParam::Lifetime(LifetimeParam::new(decode_lifetime.clone())));
    }
    {
        let where_clause = decode.make_where_clause();
        for ty in &types {
            where_clause
                .predicates
                .push(parse_quote!(#ty: #krate::FieldDecode<#decode_lifetime>));
        }
    }

    Ok(ImplGenerics {
        encode,
        decode,
        decode_lifetime,
    })
}

fn expand_struct(input: &DeriveInput, data: &DataStruct, container: &ContainerAttrs) -> syn::Result<TokenStream2> {
    let krate = &container.krate;
    let name = &input.ident;
    let mut fields = Vec::new();
    match &data.fields {
        Fields::Named(named) => {
            for field in &named.named {
                let ident = field.ident.clone().expect("named field");
                fields.push(StructField {
                    name: ident.to_string(),
                    access: quote!(#ident),
                    ty: field.ty.clone(),
                    attrs: parse_field_attrs(&field.attrs, krate)?,
                    key: None,
                    init: FieldInit::Named(ident),
                });
            }
        }
        Fields::Unnamed(unnamed) => {
            for (index, field) in unnamed.unnamed.iter().enumerate() {
                let index = Index::from(index);
                fields.push(StructField {
                    name: index.index.to_string(),
                    access: quote!(#index),
                    ty: field.ty.clone(),
                    attrs: parse_field_attrs(&field.attrs, krate)?,
                    key: None,
                    init: FieldInit::Tuple,
                });
            }
        }
        Fields::Unit => {}
    }
    assign_keys(&mut fields)?;

    let generics = build_impl_generics(input, krate, fields.iter().map(|field| field.ty.clone()), container)?;
    let (encode_impl_generics, _, encode_where) = generics.encode.split_for_impl();
    let (decode_impl_generics, _, decode_where) = generics.decode.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let decode_lifetime = &generics.decode_lifetime;

    let field_impl = expand_struct_field_impl(
        name,
        &fields,
        krate,
        decode_lifetime,
        encode_impl_generics.clone(),
        encode_where,
        decode_impl_generics.clone(),
        decode_where,
        ty_generics.clone(),
    );
    let row_impl = expand_struct_row_impl(
        name,
        &fields,
        krate,
        decode_lifetime,
        encode_impl_generics.clone(),
        encode_where,
        decode_impl_generics.clone(),
        decode_where,
        ty_generics.clone(),
    );
    let iter_impl = if container.iterable {
        expand_struct_iter_impl(input, name, &fields, krate, decode_lifetime, ty_generics.clone())
    } else {
        quote!()
    };
    Ok(quote! {
        #field_impl
        #row_impl
        #iter_impl
    })
}

fn expand_struct_field_impl(
    name: &Ident,
    fields: &[StructField],
    krate: &Path,
    decode_lifetime: &Lifetime,
    encode_impl_generics: syn::ImplGenerics<'_>,
    encode_where: Option<&syn::WhereClause>,
    decode_impl_generics: syn::ImplGenerics<'_>,
    decode_where: Option<&syn::WhereClause>,
    ty_generics: syn::TypeGenerics<'_>,
) -> TokenStream2 {
    let encoded: Vec<(usize, &StructField)> = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| !field.attrs.skip)
        .collect();
    let count = encoded.len();
    let encode_fields = encoded.iter().map(|(_, field)| {
        let ty = &field.ty;
        let access = &field.access;
        let encoding = field
            .attrs
            .encoding
            .clone()
            .unwrap_or_else(|| quote!(#krate::Encoding::Auto));
        quote! {
            <#ty as #krate::Field>::encode_column(
                values.clone().map(|__row| &__row.#access),
                out,
                #encoding,
            )?;
        }
    });
    let decode_fields = encoded.iter().map(|(index, field)| {
        let ty = &field.ty;
        let local = format_ident!("__col_{}", index);
        quote! {
            let mut #local = <#ty as #krate::FieldDecode<#decode_lifetime>>::decode_column(
                d, rows, options
            )?.into_iter();
        }
    });
    let inits = fields.iter().enumerate().map(|(index, field)| {
        let value = if field.attrs.skip {
            default_expr(&field.attrs)
        } else {
            let local = format_ident!("__col_{}", index);
            quote!(#krate::__private::column_value(&mut #local, rows)?)
        };
        match &field.init {
            FieldInit::Named(ident) => quote!(#ident: #value),
            FieldInit::Tuple => value,
        }
    });
    let constructor = struct_constructor(name, fields, inits);

    quote! {
        impl #encode_impl_generics #krate::Field for #name #ty_generics #encode_where {
            fn encode_column<'__a, __I>(
                values: __I,
                out: &mut #krate::__private::Encoder<#krate::__private::Vec<u8>>,
                hint: #krate::Encoding,
            ) -> #krate::Result<()>
            where
                __I: Iterator<Item = &'__a Self> + Clone,
                Self: '__a,
            {
                if !matches!(hint, #krate::Encoding::Auto | #krate::Encoding::Struct) {
                    return Err(#krate::Error::UnsupportedEncoding(hint));
                }
                #krate::__private::write_column(out, #krate::__private::ENC_STRUCT)?;
                out.array(#count)?;
                #(#encode_fields)*
                Ok(())
            }
        }

        impl #decode_impl_generics #krate::FieldDecode<#decode_lifetime> for #name #ty_generics #decode_where {
            fn decode_column(
                d: &mut #krate::__private::Decoder<#decode_lifetime>,
                rows: usize,
                options: &#krate::DecodeOptions,
            ) -> #krate::Result<#krate::__private::Vec<Self>> {
                let encoding = #krate::__private::read_column(d)?;
                if encoding != #krate::__private::ENC_STRUCT {
                    return Err(#krate::Error::InvalidEncoding(encoding));
                }
                #krate::__private::read_array_len(d, #count)?;
                #(#decode_fields)*
                #krate::__private::check_output_bytes::<Self>(options, rows)?;
                let mut out = #krate::__private::Vec::with_capacity(
                    #krate::__private::output_capacity::<Self>(options, rows)?
                );
                for _ in 0..rows {
                    out.push(#constructor);
                }
                Ok(out)
            }
        }
    }
}

fn expand_struct_row_impl(
    name: &Ident,
    fields: &[StructField],
    krate: &Path,
    decode_lifetime: &Lifetime,
    encode_impl_generics: syn::ImplGenerics<'_>,
    encode_where: Option<&syn::WhereClause>,
    decode_impl_generics: syn::ImplGenerics<'_>,
    decode_where: Option<&syn::WhereClause>,
    ty_generics: syn::TypeGenerics<'_>,
) -> TokenStream2 {
    let encoded: Vec<(usize, &StructField)> = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| !field.attrs.skip)
        .collect();
    let count = encoded.len();

    let encode_fields = encoded.iter().map(|(_, field)| {
        let key = field.key.expect("assigned key");
        let ty = &field.ty;
        let access = &field.access;
        let encoding = field
            .attrs
            .encoding
            .clone()
            .unwrap_or_else(|| quote!(#krate::Encoding::Auto));
        quote! {
            out.u64(#key)?;
            <#ty as #krate::Field>::encode_column(
                rows.iter().map(|__row| &__row.#access),
                out,
                #encoding,
            )?;
        }
    });

    let decode_decls = encoded.iter().map(|(index, field)| {
        let ty = &field.ty;
        let local = format_ident!("__field_{}", index);
        quote! {
            let mut #local: ::core::option::Option<#krate::__private::Vec<#ty>> =
                ::core::option::Option::None;
        }
    });
    let decode_arms = encoded.iter().map(|(index, field)| {
        let key = field.key.expect("assigned key");
        let local = format_ident!("__field_{}", index);
        let ty = &field.ty;
        quote! {
            #key => {
                #local = ::core::option::Option::Some(
                    <#ty as #krate::FieldDecode<#decode_lifetime>>::decode_column(
                        &mut d, row_count, &options
                    )?
                );
            }
        }
    });
    let decode_iters = fields.iter().enumerate().map(|(index, field)| {
        let iter = format_ident!("__iter_{}", index);
        let ty = &field.ty;
        if field.attrs.skip {
            let default = default_expr(&field.attrs);
            quote! {
                #krate::__private::check_output_bytes::<#ty>(&options, rows)?;
                let mut #iter = ::core::iter::repeat_with(|| #default)
                    .take(rows)
                    .collect::<#krate::__private::Vec<#ty>>()
                    .into_iter();
            }
        } else if field.attrs.default.is_some() {
            let local = format_ident!("__field_{}", index);
            let default = default_expr(&field.attrs);
            quote! {
                let mut #iter = match #local {
                    ::core::option::Option::Some(values) => values.into_iter(),
                    ::core::option::Option::None => {
                        #krate::__private::check_output_bytes::<#ty>(&options, rows)?;
                        ::core::iter::repeat_with(|| #default)
                            .take(rows)
                            .collect::<#krate::__private::Vec<#ty>>()
                            .into_iter()
                    }
                };
            }
        } else {
            let local = format_ident!("__field_{}", index);
            let name = &field.name;
            quote! {
                let mut #iter = match #local {
                    ::core::option::Option::Some(values) => values.into_iter(),
                    ::core::option::Option::None => {
                        return Err(#krate::Error::MissingField(#name));
                    }
                };
            }
        }
    });
    let inits = fields.iter().enumerate().map(|(index, field)| {
        let iter = format_ident!("__iter_{}", index);
        let value = quote!(#krate::__private::column_value(&mut #iter, rows)?);
        match &field.init {
            FieldInit::Named(ident) => quote!(#ident: #value),
            FieldInit::Tuple => value,
        }
    });
    let constructor = struct_constructor(name, fields, inits);

    quote! {
        impl #encode_impl_generics #krate::Columnar for #name #ty_generics #encode_where {
            fn encode_rows(rows: &[Self]) -> #krate::Result<#krate::__private::Vec<u8>> {
                let mut out = #krate::__private::Vec::with_capacity(
                    rows.len()
                        .saturating_mul(::core::mem::size_of::<Self>().min(8))
                        .saturating_add(256)
                );
                Self::encode_rows_into(rows, &mut out)?;
                Ok(out)
            }

            fn encode_rows_into(
                rows: &[Self],
                out: &mut #krate::__private::Vec<u8>,
            ) -> #krate::Result<()> {
                out.clear();
                let mut enc = #krate::__private::Encoder::new(::core::mem::take(out));
                let result = (|| -> #krate::Result<()> {
                    let out = &mut enc;
                    out.map(1 + #count)?;
                    out.u64(0)?;
                    #krate::__private::write_header(out, rows.len())?;
                    #(#encode_fields)*
                    Ok(())
                })();
                *out = enc.into_inner();
                result
            }
        }

        impl #decode_impl_generics #krate::ColumnarDecode<#decode_lifetime> for #name #ty_generics #decode_where {
            fn decode_rows(
                bytes: &#decode_lifetime [u8],
                options: #krate::DecodeOptions,
            ) -> #krate::Result<#krate::__private::Vec<Self>> {
                if bytes.len() > options.max_total_bytes {
                    return Err(#krate::Error::LimitExceeded("input size"));
                }
                let mut d = #krate::__private::decoder(bytes, options);
                let map_len = #krate::__private::read_map_len(&mut d)?;
                let mut rows: ::core::option::Option<usize> = ::core::option::Option::None;
                let mut __seen = #krate::__private::SeenKeys::new();
                #(#decode_decls)*
                let mut __first = true;
                for _ in 0..map_len {
                    let key = d.u64()?;
                    if __first {
                        if key != 0 {
                            return Err(#krate::Error::MissingHeader);
                        }
                        __first = false;
                    }
                    if !__seen.insert(key) {
                        return Err(#krate::Error::DuplicateKey(key));
                    }
                    if key == 0 {
                        rows = ::core::option::Option::Some(
                            #krate::__private::read_header(&mut d, options)?
                        );
                        continue;
                    }
                    let row_count = rows.ok_or(#krate::Error::MissingHeader)?;
                    match key {
                        #(#decode_arms)*
                        _ => {
                            d.skip()?;
                        }
                    }
                }
                if d.remaining() != 0 {
                    return Err(#krate::Error::TrailingData);
                }
                let rows = rows.ok_or(#krate::Error::MissingHeader)?;
                #(#decode_iters)*
                #krate::__private::check_output_bytes::<Self>(&options, rows)?;
                let mut out = #krate::__private::Vec::with_capacity(
                    #krate::__private::output_capacity::<Self>(&options, rows)?
                );
                for _ in 0..rows {
                    out.push(#constructor);
                }
                Ok(out)
            }
        }
    }
}

fn expand_struct_iter_impl(
    input: &DeriveInput,
    name: &Ident,
    fields: &[StructField],
    krate: &Path,
    decode_lifetime: &Lifetime,
    ty_generics: syn::TypeGenerics<'_>,
) -> TokenStream2 {
    let encoded: Vec<(usize, &StructField)> = fields
        .iter()
        .enumerate()
        .filter(|(_, field)| !field.attrs.skip)
        .collect();
    let count = encoded.len();
    let rows_name = format_ident!("{}Rows", name);
    let visibility = &input.vis;

    let mut iter_generics = input.generics.clone();
    if !iter_generics
        .lifetimes()
        .any(|param| param.lifetime == *decode_lifetime)
    {
        iter_generics
            .params
            .insert(0, GenericParam::Lifetime(LifetimeParam::new(decode_lifetime.clone())));
    }
    {
        let where_clause = iter_generics.make_where_clause();
        for (_, field) in &encoded {
            let ty = &field.ty;
            where_clause
                .predicates
                .push(parse_quote!(#ty: #krate::FieldIter<#decode_lifetime> + #decode_lifetime));
            if matches!(field.attrs.default, Some(None)) {
                where_clause
                    .predicates
                    .push(parse_quote!(#ty: ::core::default::Default));
            }
        }
        for field in fields.iter().filter(|field| field.attrs.skip) {
            if field.attrs.default.is_none() {
                let ty = &field.ty;
                where_clause
                    .predicates
                    .push(parse_quote!(#ty: ::core::default::Default));
            }
        }
        for param in input.generics.lifetimes() {
            let lifetime = &param.lifetime;
            if lifetime != decode_lifetime {
                where_clause.predicates.push(parse_quote!(#lifetime: #decode_lifetime));
            }
        }
    }
    let (iter_impl_generics, iter_ty_generics, iter_where) = iter_generics.split_for_impl();

    let row_field_names: Vec<Ident> = encoded.iter().map(|(index, _)| format_ident!("__f{}", index)).collect();
    let row_field_types: Vec<TokenStream2> = encoded
        .iter()
        .map(|(_, field)| {
            let ty = &field.ty;
            if field.attrs.default.is_some() {
                quote!(
                    #krate::__private::MaybeDefault<
                        <#ty as #krate::FieldIter<#decode_lifetime>>::Iter,
                        #ty,
                    >
                )
            } else {
                quote!(<#ty as #krate::FieldIter<#decode_lifetime>>::Iter)
            }
        })
        .collect();

    let next_values = encoded.iter().map(|(index, _)| {
        let row_field = format_ident!("__f{}", index);
        let local = format_ident!("__v{}", index);
        quote! {
            let #local = match self.#row_field.next() {
                ::core::option::Option::Some(::core::result::Result::Ok(value)) => value,
                ::core::option::Option::Some(::core::result::Result::Err(error)) => {
                    return ::core::option::Option::Some(::core::result::Result::Err(error));
                }
                ::core::option::Option::None => {
                    return ::core::option::Option::Some(::core::result::Result::Err(
                        #krate::Error::LengthMismatch { expected: 1, actual: 0 },
                    ));
                }
            };
        }
    });
    let inits = fields.iter().enumerate().map(|(index, field)| {
        let value = if field.attrs.skip {
            default_expr(&field.attrs)
        } else {
            let local = format_ident!("__v{}", index);
            quote!(#local)
        };
        match &field.init {
            FieldInit::Named(ident) => quote!(#ident: #value),
            FieldInit::Tuple => value,
        }
    });
    let constructor = struct_constructor(name, fields, inits);

    let decode_fields = encoded.iter().map(|(index, field)| {
        let row_field = format_ident!("__f{}", index);
        let ty = &field.ty;
        let decoded = quote!(
            <#ty as #krate::FieldIter<#decode_lifetime>>::decode_column_iter(
                d, rows, options
            )?
        );
        if field.attrs.default.is_some() {
            quote!(let #row_field = #krate::__private::MaybeDefault::Values(#decoded);)
        } else {
            quote!(let #row_field = #decoded;)
        }
    });

    let iter_locals = encoded.iter().map(|(index, field)| {
        let local = format_ident!("__iter_{}", index);
        let ty = &field.ty;
        let field_type = if field.attrs.default.is_some() {
            quote!(
                #krate::__private::MaybeDefault<
                    <#ty as #krate::FieldIter<#decode_lifetime>>::Iter,
                    #ty,
                >
            )
        } else {
            quote!(<#ty as #krate::FieldIter<#decode_lifetime>>::Iter)
        };
        quote! {
            let mut #local: ::core::option::Option<#field_type> =
                ::core::option::Option::None;
        }
    });
    let decode_arms = encoded.iter().map(|(index, field)| {
        let key = field.key.expect("assigned key");
        let local = format_ident!("__iter_{}", index);
        let ty = &field.ty;
        let decoded = quote!(
            <#ty as #krate::FieldIter<#decode_lifetime>>::decode_column_iter(
                &mut d, row_count, &options
            )?
        );
        if field.attrs.default.is_some() {
            quote! {
                #key => {
                    #local = ::core::option::Option::Some(
                        #krate::__private::MaybeDefault::Values(#decoded)
                    );
                }
            }
        } else {
            quote! {
                #key => {
                    #local = ::core::option::Option::Some(#decoded);
                }
            }
        }
    });
    let finish_fields = encoded.iter().map(|(index, field)| {
        let row_field = format_ident!("__f{}", index);
        let local = format_ident!("__iter_{}", index);
        if field.attrs.default.is_some() {
            let ty = &field.ty;
            let default_fn = match &field.attrs.default {
                Some(Some(path)) => quote!(#path),
                Some(None) => quote!(<#ty as ::core::default::Default>::default),
                None => unreachable!(),
            };
            quote! {
                let #row_field = match #local {
                    ::core::option::Option::Some(iter) => iter,
                    ::core::option::Option::None => {
                        #krate::__private::MaybeDefault::Default {
                            remaining: rows,
                            create: #default_fn,
                        }
                    }
                };
            }
        } else {
            let name = &field.name;
            quote! {
                let #row_field = match #local {
                    ::core::option::Option::Some(iter) => iter,
                    ::core::option::Option::None => {
                        return ::core::result::Result::Err(
                            #krate::Error::MissingField(#name)
                        );
                    }
                };
            }
        }
    });

    quote! {
        #[doc(hidden)]
        #visibility struct #rows_name #iter_ty_generics #iter_where {
            #(#row_field_names: #row_field_types,)*
            __marker: ::core::marker::PhantomData<&#decode_lifetime ()>,
            remaining: usize,
        }

        impl #iter_impl_generics ::core::iter::Iterator for #rows_name #iter_ty_generics #iter_where {
            type Item = #krate::Result<#name #ty_generics>;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                if self.remaining == 0 {
                    return ::core::option::Option::None;
                }
                self.remaining -= 1;
                #(#next_values)*
                ::core::option::Option::Some(::core::result::Result::Ok(#constructor))
            }

            fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
                (self.remaining, ::core::option::Option::Some(self.remaining))
            }
        }

        impl #iter_impl_generics ::core::iter::ExactSizeIterator for #rows_name #iter_ty_generics #iter_where {}
        impl #iter_impl_generics ::core::iter::FusedIterator for #rows_name #iter_ty_generics #iter_where {}

        impl #iter_impl_generics #krate::FieldIter<#decode_lifetime> for #name #ty_generics #iter_where {
            type Iter = #rows_name #iter_ty_generics;

            fn decode_column_iter(
                d: &mut #krate::__private::Decoder<#decode_lifetime>,
                rows: usize,
                options: &#krate::DecodeOptions,
            ) -> #krate::Result<Self::Iter> {
                let encoding = #krate::__private::read_column(d)?;
                if encoding != #krate::__private::ENC_STRUCT {
                    return ::core::result::Result::Err(#krate::Error::InvalidEncoding(encoding));
                }
                #krate::__private::read_array_len(d, #count)?;
                #(#decode_fields)*
                ::core::result::Result::Ok(#rows_name {
                    #(#row_field_names,)*
                    __marker: ::core::marker::PhantomData,
                    remaining: rows,
                })
            }
        }

        impl #iter_impl_generics #krate::ColumnarIter<#decode_lifetime> for #name #ty_generics #iter_where {
            type Rows = #rows_name #iter_ty_generics;

            fn iter_rows(
                bytes: &#decode_lifetime [u8],
                options: #krate::DecodeOptions,
            ) -> #krate::Result<Self::Rows> {
                if bytes.len() > options.max_total_bytes {
                    return ::core::result::Result::Err(
                        #krate::Error::LimitExceeded("input size")
                    );
                }
                let mut d = #krate::__private::decoder(bytes, options);
                let map_len = #krate::__private::read_map_len(&mut d)?;
                let mut rows: ::core::option::Option<usize> = ::core::option::Option::None;
                let mut __seen = #krate::__private::SeenKeys::new();
                #(#iter_locals)*
                let mut __first = true;
                for _ in 0..map_len {
                    let key = d.u64()?;
                    if __first {
                        if key != 0 {
                            return ::core::result::Result::Err(#krate::Error::MissingHeader);
                        }
                        __first = false;
                    }
                    if !__seen.insert(key) {
                        return ::core::result::Result::Err(#krate::Error::DuplicateKey(key));
                    }
                    if key == 0 {
                        rows = ::core::option::Option::Some(
                            #krate::__private::read_header(&mut d, options)?
                        );
                        continue;
                    }
                    let row_count = rows.ok_or(#krate::Error::MissingHeader)?;
                    match key {
                        #(#decode_arms)*
                        _ => {
                            d.skip()?;
                        }
                    }
                }
                if d.remaining() != 0 {
                    return ::core::result::Result::Err(#krate::Error::TrailingData);
                }
                let rows = rows.ok_or(#krate::Error::MissingHeader)?;
                #(#finish_fields)*
                ::core::result::Result::Ok(#rows_name {
                    #(#row_field_names,)*
                    __marker: ::core::marker::PhantomData,
                    remaining: rows,
                })
            }
        }
    }
}

fn struct_constructor(name: &Ident, fields: &[StructField], inits: impl Iterator<Item = TokenStream2>) -> TokenStream2 {
    if fields.is_empty() {
        return quote!(#name);
    }
    if fields.iter().all(|field| matches!(field.init, FieldInit::Named(_))) {
        quote!(#name { #(#inits),* })
    } else {
        quote!(#name(#(#inits),*))
    }
}

fn expand_enum(input: &DeriveInput, data: &DataEnum, container: &ContainerAttrs) -> syn::Result<TokenStream2> {
    let krate = &container.krate;
    let name = &input.ident;
    let mut variants = Vec::new();
    for (variant_index, variant) in data.variants.iter().enumerate() {
        let named = matches!(variant.fields, Fields::Named(_));
        let mut fields = Vec::new();
        match &variant.fields {
            Fields::Named(named_fields) => {
                for (field_index, field) in named_fields.named.iter().enumerate() {
                    let ident = field.ident.clone().expect("named field");
                    let binding = format_ident!("__v{}_{}", variant_index, field_index);
                    fields.push(VariantField {
                        binding,
                        ty: field.ty.clone(),
                        attrs: parse_field_attrs(&field.attrs, krate)?,
                        pattern_name: Some(ident),
                    });
                }
            }
            Fields::Unnamed(unnamed) => {
                for (field_index, field) in unnamed.unnamed.iter().enumerate() {
                    let binding = format_ident!("__v{}_{}", variant_index, field_index);
                    fields.push(VariantField {
                        binding,
                        ty: field.ty.clone(),
                        attrs: parse_field_attrs(&field.attrs, krate)?,
                        pattern_name: None,
                    });
                }
            }
            Fields::Unit => {}
        }
        for field in &fields {
            if field.attrs.key.is_some() {
                return Err(syn::Error::new(
                    field.ty.span(),
                    "columnar keys are only supported on struct fields",
                ));
            }
        }
        variants.push(Variant {
            ident: variant.ident.clone(),
            named,
            fields,
        });
    }

    let variant_count = variants.len();
    let generics = build_impl_generics(
        input,
        krate,
        variants
            .iter()
            .flat_map(|variant| variant.fields.iter().map(|field| field.ty.clone())),
        container,
    )?;
    let (encode_impl_generics, _, encode_where) = generics.encode.split_for_impl();
    let (decode_impl_generics, _, decode_where) = generics.decode.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();
    let decode_lifetime = &generics.decode_lifetime;

    let field_impl = expand_enum_field_impl(
        name,
        &variants,
        variant_count,
        krate,
        decode_lifetime,
        encode_impl_generics.clone(),
        encode_where,
        decode_impl_generics.clone(),
        decode_where,
        ty_generics.clone(),
    );
    let row_impl = quote! {
        impl #encode_impl_generics #krate::Columnar for #name #ty_generics #encode_where {
            fn encode_rows(rows: &[Self]) -> #krate::Result<#krate::__private::Vec<u8>> {
                let mut out = #krate::__private::Vec::with_capacity(
                    rows.len()
                        .saturating_mul(::core::mem::size_of::<Self>().min(8))
                        .saturating_add(256)
                );
                Self::encode_rows_into(rows, &mut out)?;
                Ok(out)
            }

            fn encode_rows_into(
                rows: &[Self],
                out: &mut #krate::__private::Vec<u8>,
            ) -> #krate::Result<()> {
                out.clear();
                let mut enc = #krate::__private::Encoder::new(::core::mem::take(out));
                let result = (|| -> #krate::Result<()> {
                    let out = &mut enc;
                    out.map(2)?;
                    out.u64(0)?;
                    #krate::__private::write_header(out, rows.len())?;
                    out.u64(1)?;
                    <Self as #krate::Field>::encode_column(rows.iter(), out, #krate::Encoding::Auto)?;
                    Ok(())
                })();
                *out = enc.into_inner();
                result
            }
        }

        impl #decode_impl_generics #krate::ColumnarDecode<#decode_lifetime> for #name #ty_generics #decode_where {
            fn decode_rows(
                bytes: &#decode_lifetime [u8],
                options: #krate::DecodeOptions,
            ) -> #krate::Result<#krate::__private::Vec<Self>> {
                if bytes.len() > options.max_total_bytes {
                    return Err(#krate::Error::LimitExceeded("input size"));
                }
                let mut d = #krate::__private::decoder(bytes, options);
                let map_len = #krate::__private::read_map_len(&mut d)?;
                let mut rows: ::core::option::Option<usize> = ::core::option::Option::None;
                let mut column: ::core::option::Option<#krate::__private::Vec<Self>> =
                    ::core::option::Option::None;
                let mut __seen = #krate::__private::SeenKeys::new();
                let mut __first = true;
                for _ in 0..map_len {
                    let key = d.u64()?;
                    if __first {
                        if key != 0 {
                            return Err(#krate::Error::MissingHeader);
                        }
                        __first = false;
                    }
                    if !__seen.insert(key) {
                        return Err(#krate::Error::DuplicateKey(key));
                    }
                    if key == 0 {
                        rows = ::core::option::Option::Some(
                            #krate::__private::read_header(&mut d, options)?
                        );
                        continue;
                    }
                    let row_count = rows.ok_or(#krate::Error::MissingHeader)?;
                    match key {
                        1 => {
                            column = ::core::option::Option::Some(
                                <Self as #krate::FieldDecode<#decode_lifetime>>::decode_column(
                                    &mut d, row_count, &options
                                )?
                            );
                        }
                        _ => {
                            d.skip()?;
                        }
                    }
                }
                if d.remaining() != 0 {
                    return Err(#krate::Error::TrailingData);
                }
                let rows = rows.ok_or(#krate::Error::MissingHeader)?;
                let column = column.ok_or(#krate::Error::MissingField("enum column"))?;
                if column.len() != rows {
                    return Err(#krate::Error::LengthMismatch {
                        expected: rows,
                        actual: column.len(),
                    });
                }
                Ok(column)
            }
        }
    };
    let iter_impl = if container.iterable {
        expand_enum_iter_impl(
            input,
            name,
            &variants,
            variant_count,
            krate,
            decode_lifetime,
            ty_generics.clone(),
        )
    } else {
        quote!()
    };
    Ok(quote! {
        #field_impl
        #row_impl
        #iter_impl
    })
}

fn expand_enum_field_impl(
    name: &Ident,
    variants: &[Variant],
    variant_count: usize,
    krate: &Path,
    decode_lifetime: &Lifetime,
    encode_impl_generics: syn::ImplGenerics<'_>,
    encode_where: Option<&syn::WhereClause>,
    decode_impl_generics: syn::ImplGenerics<'_>,
    decode_where: Option<&syn::WhereClause>,
    ty_generics: syn::TypeGenerics<'_>,
) -> TokenStream2 {
    let tag_arms = variants.iter().enumerate().map(|(index, variant)| {
        let pattern = variant_pattern(variant, false);
        let index = index as u32;
        quote!(#pattern => #index,)
    });
    let total_fields: usize = variants
        .iter()
        .map(|variant| variant.fields.iter().filter(|field| !field.attrs.skip).count())
        .sum();

    let encode_children = variants.iter().flat_map(|variant| {
        variant
            .fields
            .iter()
            .filter(|field| !field.attrs.skip)
            .map(move |field| {
                let ty = &field.ty;
                let encoding = field
                    .attrs
                    .encoding
                    .clone()
                    .unwrap_or_else(|| quote!(#krate::Encoding::Auto));
                let pattern = variant_pattern_with_binding(variant, field);
                quote! {
                    <#ty as #krate::Field>::encode_column(
                        values.clone().filter_map(|__value| match __value {
                            #pattern => ::core::option::Option::Some(__bound),
                            _ => ::core::option::Option::None,
                        }),
                        out,
                        #encoding,
                    )?;
                }
            })
    });

    let decode_decls = variants.iter().enumerate().flat_map(|(variant_index, variant)| {
        variant
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !field.attrs.skip)
            .map(move |(field_index, field)| {
                let ty = &field.ty;
                let local = format_ident!("__ec_{}_{}", variant_index, field_index);
                quote! {
                    let mut #local: ::core::option::Option<#krate::__private::Vec<#ty>> =
                        ::core::option::Option::None;
                }
            })
    });
    let decode_children = variants.iter().enumerate().flat_map(|(variant_index, variant)| {
        variant
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !field.attrs.skip)
            .map(move |(field_index, field)| {
                let ty = &field.ty;
                let local = format_ident!("__ec_{}_{}", variant_index, field_index);
                quote! {
                    #local = ::core::option::Option::Some(
                        <#ty as #krate::FieldDecode<#decode_lifetime>>::decode_column(
                            d,
                            __counts[#variant_index],
                            options,
                        )?
                    );
                }
            })
    });
    let decode_iters = variants.iter().enumerate().flat_map(|(variant_index, variant)| {
        variant
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !field.attrs.skip)
            .map(move |(field_index, _field)| {
                let local = format_ident!("__ec_{}_{}", variant_index, field_index);
                let iter = format_ident!("__ei_{}_{}", variant_index, field_index);
                quote! {
                    let mut #iter = #local
                        .ok_or(#krate::Error::MissingField("enum variant column"))?
                        .into_iter();
                }
            })
    });

    let build_arms = variants.iter().enumerate().map(|(variant_index, variant)| {
        let body = if variant.fields.is_empty() {
            let ident = &variant.ident;
            quote!(#name::#ident)
        } else if variant.named {
            let inits = variant.fields.iter().enumerate().map(|(field_index, field)| {
                let field_ident = field.pattern_name.as_ref().unwrap();
                let value = if field.attrs.skip {
                    default_expr(&field.attrs)
                } else {
                    let iter = format_ident!("__ei_{}_{}", variant_index, field_index);
                    quote!(#krate::__private::column_value(&mut #iter, __counts[#variant_index])?)
                };
                quote!(#field_ident: #value)
            });
            let ident = &variant.ident;
            quote!(#name::#ident { #(#inits),* })
        } else {
            let values = variant.fields.iter().enumerate().map(|(field_index, field)| {
                if field.attrs.skip {
                    default_expr(&field.attrs)
                } else {
                    let iter = format_ident!("__ei_{}_{}", variant_index, field_index);
                    quote!(#krate::__private::column_value(&mut #iter, __counts[#variant_index])?)
                }
            });
            let ident = &variant.ident;
            quote!(#name::#ident(#(#values),*))
        };
        let index = variant_index as u32;
        quote! {
            #index => {
                #body
            }
        }
    });

    quote! {
        impl #encode_impl_generics #krate::Field for #name #ty_generics #encode_where {
            fn encode_column<'__a, __I>(
                values: __I,
                out: &mut #krate::__private::Encoder<#krate::__private::Vec<u8>>,
                hint: #krate::Encoding,
            ) -> #krate::Result<()>
            where
                __I: Iterator<Item = &'__a Self> + Clone,
                Self: '__a,
            {
                if !matches!(hint, #krate::Encoding::Auto | #krate::Encoding::Enum) {
                    return Err(#krate::Error::UnsupportedEncoding(hint));
                }
                let width = #krate::__private::bits_needed(
                    u128::from((#variant_count as u64).saturating_sub(1))
                );
                let tags = values.clone().map(|value| {
                    let tag: u32 = match value {
                        #(#tag_arms)*
                    };
                    tag
                });
                let packed = #krate::__private::pack_u32(tags, width);
                #krate::__private::write_column(out, #krate::__private::ENC_ENUM)?;
                out.array(1 + #total_fields)?;
                out.bytes(&packed)?;
                #(#encode_children)*
                Ok(())
            }
        }

        impl #decode_impl_generics #krate::FieldDecode<#decode_lifetime> for #name #ty_generics #decode_where {
            fn decode_column(
                d: &mut #krate::__private::Decoder<#decode_lifetime>,
                rows: usize,
                options: &#krate::DecodeOptions,
            ) -> #krate::Result<#krate::__private::Vec<Self>> {
                let encoding = #krate::__private::read_column(d)?;
                if encoding != #krate::__private::ENC_ENUM {
                    return Err(#krate::Error::InvalidEncoding(encoding));
                }
                #krate::__private::read_array_len(d, 1 + #total_fields)?;
                let tags_data = d.bytes()?;
                let width = #krate::__private::bits_needed(
                    u128::from((#variant_count as u64).saturating_sub(1))
                );
                let mut __counts = [0usize; #variant_count];
                #krate::__private::count_enum_tags(tags_data, rows, width, &mut __counts)?;
                let mut __tags = #krate::__private::TagIter::new(
                    tags_data, rows, width, #variant_count
                )?;
                #(#decode_decls)*
                #(#decode_children)*
                #(#decode_iters)*
                #krate::__private::check_output_bytes::<Self>(&options, rows)?;
                let mut out = #krate::__private::Vec::with_capacity(
                    #krate::__private::output_capacity::<Self>(&options, rows)?
                );
                for tag in &mut __tags {
                    let tag = tag?;
                    out.push(match tag {
                        #(#build_arms)*
                        _ => return Err(#krate::Error::InvalidBitpackedValue),
                    });
                }
                Ok(out)
            }
        }
    }
}

fn expand_enum_iter_impl(
    input: &DeriveInput,
    name: &Ident,
    variants: &[Variant],
    variant_count: usize,
    krate: &Path,
    decode_lifetime: &Lifetime,
    ty_generics: syn::TypeGenerics<'_>,
) -> TokenStream2 {
    let total_fields: usize = variants
        .iter()
        .map(|variant| variant.fields.iter().filter(|field| !field.attrs.skip).count())
        .sum();
    let rows_name = format_ident!("{}Rows", name);
    let visibility = &input.vis;

    let mut iter_generics = input.generics.clone();
    if !iter_generics
        .lifetimes()
        .any(|param| param.lifetime == *decode_lifetime)
    {
        iter_generics
            .params
            .insert(0, GenericParam::Lifetime(LifetimeParam::new(decode_lifetime.clone())));
    }
    {
        let where_clause = iter_generics.make_where_clause();
        for variant in variants {
            for field in variant.fields.iter().filter(|field| !field.attrs.skip) {
                let ty = &field.ty;
                where_clause
                    .predicates
                    .push(parse_quote!(#ty: #krate::FieldIter<#decode_lifetime> + #decode_lifetime));
                if matches!(field.attrs.default, Some(None)) {
                    where_clause
                        .predicates
                        .push(parse_quote!(#ty: ::core::default::Default));
                }
            }
            for field in variant.fields.iter().filter(|field| field.attrs.skip) {
                if field.attrs.default.is_none() {
                    let ty = &field.ty;
                    where_clause
                        .predicates
                        .push(parse_quote!(#ty: ::core::default::Default));
                }
            }
        }
        for param in input.generics.lifetimes() {
            let lifetime = &param.lifetime;
            if lifetime != decode_lifetime {
                where_clause.predicates.push(parse_quote!(#lifetime: #decode_lifetime));
            }
        }
    }
    let (iter_impl_generics, iter_ty_generics, iter_where) = iter_generics.split_for_impl();

    let iterator_names: Vec<Ident> = variants
        .iter()
        .enumerate()
        .flat_map(|(variant_index, variant)| {
            variant
                .fields
                .iter()
                .enumerate()
                .filter(|(_, field)| !field.attrs.skip)
                .map(move |(field_index, _)| format_ident!("__v{}_{}", variant_index, field_index))
        })
        .collect();
    let iterator_types: Vec<TokenStream2> = variants
        .iter()
        .flat_map(|variant| {
            variant.fields.iter().filter(|field| !field.attrs.skip).map(|field| {
                let ty = &field.ty;
                quote!(<#ty as #krate::FieldIter<#decode_lifetime>>::Iter)
            })
        })
        .collect();

    let build_arms = variants.iter().enumerate().map(|(variant_index, variant)| {
        let body = if variant.fields.is_empty() {
            let ident = &variant.ident;
            quote!(#name::#ident)
        } else if variant.named {
            let inits = variant.fields.iter().enumerate().map(|(field_index, field)| {
                let field_ident = field.pattern_name.as_ref().unwrap();
                let value = if field.attrs.skip {
                    default_expr(&field.attrs)
                } else {
                    let iterator = format_ident!("__v{}_{}", variant_index, field_index);
                    quote! {
                        match self.#iterator.next() {
                            ::core::option::Option::Some(::core::result::Result::Ok(value)) => value,
                            ::core::option::Option::Some(::core::result::Result::Err(error)) => {
                                return ::core::option::Option::Some(::core::result::Result::Err(error));
                            }
                            ::core::option::Option::None => {
                                return ::core::option::Option::Some(::core::result::Result::Err(
                                    #krate::Error::LengthMismatch { expected: 1, actual: 0 },
                                ));
                            }
                        }
                    }
                };
                quote!(#field_ident: #value)
            });
            let ident = &variant.ident;
            quote!(#name::#ident { #(#inits),* })
        } else {
            let values = variant.fields.iter().enumerate().map(|(field_index, field)| {
                if field.attrs.skip {
                    default_expr(&field.attrs)
                } else {
                    let iterator = format_ident!("__v{}_{}", variant_index, field_index);
                    quote! {
                        match self.#iterator.next() {
                            ::core::option::Option::Some(::core::result::Result::Ok(value)) => value,
                            ::core::option::Option::Some(::core::result::Result::Err(error)) => {
                                return ::core::option::Option::Some(::core::result::Result::Err(error));
                            }
                            ::core::option::Option::None => {
                                return ::core::option::Option::Some(::core::result::Result::Err(
                                    #krate::Error::LengthMismatch { expected: 1, actual: 0 },
                                ));
                            }
                        }
                    }
                }
            });
            let ident = &variant.ident;
            quote!(#name::#ident(#(#values),*))
        };
        let index = variant_index as u32;
        quote!(#index => #body,)
    });

    let decode_fields = variants.iter().enumerate().flat_map(|(variant_index, variant)| {
        variant
            .fields
            .iter()
            .enumerate()
            .filter(|(_, field)| !field.attrs.skip)
            .map(move |(field_index, field)| {
                let iterator = format_ident!("__v{}_{}", variant_index, field_index);
                let ty = &field.ty;
                quote! {
                    let #iterator = <#ty as #krate::FieldIter<#decode_lifetime>>::decode_column_iter(
                        d, counts[#variant_index], options
                    )?;
                }
            })
    });

    quote! {
        #[doc(hidden)]
        #visibility struct #rows_name #iter_ty_generics #iter_where {
            __tags: #krate::__private::TagIter<#decode_lifetime>,
            #(#iterator_names: #iterator_types,)*
            remaining: usize,
        }

        impl #iter_impl_generics ::core::iter::Iterator for #rows_name #iter_ty_generics #iter_where {
            type Item = #krate::Result<#name #ty_generics>;

            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                if self.remaining == 0 {
                    return ::core::option::Option::None;
                }
                self.remaining -= 1;
                let tag = match self.__tags.next() {
                    ::core::option::Option::Some(::core::result::Result::Ok(tag)) => tag,
                    ::core::option::Option::Some(::core::result::Result::Err(error)) => {
                        return ::core::option::Option::Some(::core::result::Result::Err(error));
                    }
                    ::core::option::Option::None => {
                        return ::core::option::Option::Some(::core::result::Result::Err(
                            #krate::Error::LengthMismatch { expected: 1, actual: 0 },
                        ));
                    }
                };
                let row = match tag {
                    #(#build_arms)*
                    _ => {
                        return ::core::option::Option::Some(::core::result::Result::Err(
                            #krate::Error::InvalidBitpackedValue,
                        ));
                    }
                };
                ::core::option::Option::Some(::core::result::Result::Ok(row))
            }

            fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
                (self.remaining, ::core::option::Option::Some(self.remaining))
            }
        }

        impl #iter_impl_generics ::core::iter::ExactSizeIterator for #rows_name #iter_ty_generics #iter_where {}
        impl #iter_impl_generics ::core::iter::FusedIterator for #rows_name #iter_ty_generics #iter_where {}

        impl #iter_impl_generics #krate::FieldIter<#decode_lifetime> for #name #ty_generics #iter_where {
            type Iter = #rows_name #iter_ty_generics;

            fn decode_column_iter(
                d: &mut #krate::__private::Decoder<#decode_lifetime>,
                rows: usize,
                options: &#krate::DecodeOptions,
            ) -> #krate::Result<Self::Iter> {
                let encoding = #krate::__private::read_column(d)?;
                if encoding != #krate::__private::ENC_ENUM {
                    return ::core::result::Result::Err(#krate::Error::InvalidEncoding(encoding));
                }
                #krate::__private::read_array_len(d, 1 + #total_fields)?;
                let tags_data = d.bytes()?;
                let width = #krate::__private::bits_needed(
                    u128::from((#variant_count as u64).saturating_sub(1))
                );
                let mut counts = [0usize; #variant_count];
                #krate::__private::count_enum_tags(tags_data, rows, width, &mut counts)?;
                let tags = #krate::__private::TagIter::new(
                    tags_data, rows, width, #variant_count
                )?;
                #(#decode_fields)*
                ::core::result::Result::Ok(#rows_name {
                    __tags: tags,
                    #(#iterator_names,)*
                    remaining: rows,
                })
            }
        }

        impl #iter_impl_generics #krate::ColumnarIter<#decode_lifetime> for #name #ty_generics #iter_where {
            type Rows = #rows_name #iter_ty_generics;

            fn iter_rows(
                bytes: &#decode_lifetime [u8],
                options: #krate::DecodeOptions,
            ) -> #krate::Result<Self::Rows> {
                if bytes.len() > options.max_total_bytes {
                    return ::core::result::Result::Err(
                        #krate::Error::LimitExceeded("input size")
                    );
                }
                let mut d = #krate::__private::decoder(bytes, options);
                let map_len = #krate::__private::read_map_len(&mut d)?;
                let mut rows: ::core::option::Option<usize> = ::core::option::Option::None;
                let mut column: ::core::option::Option<Self::Rows> = ::core::option::Option::None;
                let mut __seen = #krate::__private::SeenKeys::new();
                let mut __first = true;
                for _ in 0..map_len {
                    let key = d.u64()?;
                    if __first {
                        if key != 0 {
                            return ::core::result::Result::Err(#krate::Error::MissingHeader);
                        }
                        __first = false;
                    }
                    if !__seen.insert(key) {
                        return ::core::result::Result::Err(#krate::Error::DuplicateKey(key));
                    }
                    if key == 0 {
                        rows = ::core::option::Option::Some(
                            #krate::__private::read_header(&mut d, options)?
                        );
                        continue;
                    }
                    let row_count = rows.ok_or(#krate::Error::MissingHeader)?;
                    match key {
                        1 => {
                            column = ::core::option::Option::Some(
                                <Self as #krate::FieldIter<#decode_lifetime>>::decode_column_iter(
                                    &mut d, row_count, &options
                                )?
                            );
                        }
                        _ => {
                            d.skip()?;
                        }
                    }
                }
                if d.remaining() != 0 {
                    return ::core::result::Result::Err(#krate::Error::TrailingData);
                }
                let _rows = rows.ok_or(#krate::Error::MissingHeader)?;
                column.ok_or(#krate::Error::MissingField("enum column"))
            }
        }
    }
}

fn variant_pattern(variant: &Variant, bindings: bool) -> TokenStream2 {
    let ident = &variant.ident;
    if variant.named {
        if bindings {
            let fields = variant.fields.iter().map(|field| {
                let name = field.pattern_name.as_ref().unwrap();
                let binding = &field.binding;
                quote!(#name: #binding)
            });
            quote!(Self::#ident { #(#fields),* })
        } else {
            quote!(Self::#ident { .. })
        }
    } else if variant.fields.is_empty() {
        quote!(Self::#ident)
    } else if bindings {
        let bindings = variant.fields.iter().map(|field| &field.binding);
        quote!(Self::#ident(#(#bindings),*))
    } else {
        quote!(Self::#ident(..))
    }
}

fn variant_pattern_with_binding(variant: &Variant, field: &VariantField) -> TokenStream2 {
    let ident = &variant.ident;
    if variant.named {
        let name = field.pattern_name.as_ref().unwrap();
        quote!(Self::#ident { #name: __bound, .. })
    } else {
        let mut patterns = Vec::new();
        for other in &variant.fields {
            if std::ptr::eq(other, field) {
                patterns.push(quote!(__bound));
            } else {
                patterns.push(quote!(_));
            }
        }
        quote!(Self::#ident(#(#patterns),*))
    }
}
