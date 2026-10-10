//! `#[derive(FromRow)]` for the `postgresql` crate.
//!
//! Decodes a [`postgresql::Row`] into a struct by looking up each field's
//! column by name. Column names default to the field name, and can be changed
//! with `#[postgresql(rename = "...")]` or transformed with a container-level
//! `#[postgresql(rename_all = "...")]` (`snake_case`, `camelCase`,
//! `PascalCase`, `SCREAMING_SNAKE_CASE`, `lowercase`, `UPPERCASE`).
//!
//! `#[postgresql(default)]` makes a missing or `NULL` column fall back to the
//! field type's `Default` instead of erroring.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, WherePredicate, parse_macro_input};

/// Derives [`postgresql::FromRow`] for a struct with named fields.
#[proc_macro_derive(FromRow, attributes(postgresql, pg))]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn is_pg_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("postgresql") || attr.path().is_ident("pg")
}

/// Builds a `where` clause from the type's own predicates plus `extra`.
fn merge_where(generics: &syn::Generics, extra: impl IntoIterator<Item = WherePredicate>) -> proc_macro2::TokenStream {
    let mut predicates: syn::punctuated::Punctuated<WherePredicate, syn::Token![,]> = generics
        .where_clause
        .as_ref()
        .map(|w| w.predicates.clone())
        .unwrap_or_default();
    for predicate in extra {
        predicates.push(predicate);
    }
    if predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #predicates }
    }
}

fn get_rename_all(attrs: &[syn::Attribute]) -> syn::Result<Option<String>> {
    let mut rename_all = None;
    for attr in attrs.iter().filter(|a| is_pg_attr(a)) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename_all") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                rename_all = Some(lit.value());
                Ok(())
            } else {
                // Ignore unknown container attributes so field-only keys don't error.
                let _ = meta.input;
                Ok(())
            }
        })?;
    }
    Ok(rename_all)
}

fn expand(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let rename_all = get_rename_all(&input.attrs)?;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "FromRow can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(input, "FromRow can only be derived for structs"));
        }
    };

    let mut assignments = Vec::with_capacity(fields.len());
    let mut shape_entries = Vec::with_capacity(fields.len());
    let mut shape_bounds: Vec<syn::WherePredicate> = Vec::with_capacity(fields.len());

    for field in fields {
        let ident = field.ident.as_ref().expect("named field");
        let mut column: Option<String> = None;
        let mut default = false;

        for attr in field.attrs.iter().filter(|a| is_pg_attr(a)) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") || meta.path.is_ident("column") {
                    let value = meta.value()?;
                    let lit: syn::LitStr = value.parse()?;
                    column = Some(lit.value());
                    Ok(())
                } else if meta.path.is_ident("default") {
                    default = true;
                    Ok(())
                } else {
                    Ok(())
                }
            })?;
        }

        let column = column.unwrap_or_else(|| apply_rename(&ident.to_string(), rename_all.as_deref()));
        let ty = &field.ty;

        if default {
            assignments.push(quote! {
                #ident: ::postgresql::Row::try_get::<#ty>(row, #column)
                    .or_else(|_| ::core::result::Result::Ok(::core::default::Default::default()))?,
            });
        } else {
            assignments.push(quote! {
                #ident: ::postgresql::Row::try_get::<#ty>(row, #column)?,
            });
            shape_bounds.push(syn::parse_quote!(#ty: ::postgresql::StaticType));
            shape_entries.push(quote! {
                ::postgresql::ColumnSpec {
                    name: #column,
                    type_oid: <#ty as ::postgresql::StaticType>::OID,
                    nullable: <#ty as ::postgresql::StaticType>::NULLABLE,
                    known: true,
                },
            });
        }
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut row_shape_predicates: syn::punctuated::Punctuated<syn::WherePredicate, syn::Token![,]> = input
        .generics
        .where_clause
        .as_ref()
        .map(|w| w.predicates.clone())
        .unwrap_or_default();
    for bound in shape_bounds {
        row_shape_predicates.push(bound);
    }
    let row_shape_where = if row_shape_predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #row_shape_predicates }
    };

    let expanded = quote! {
        impl #impl_generics ::postgresql::FromRow for #name #ty_generics #where_clause {
            fn from_row(
                row: &::postgresql::Row,
            ) -> ::core::result::Result<Self, ::postgresql::Error> {
                ::core::result::Result::Ok(Self {
                    #(#assignments)*
                })
            }
        }

        impl #impl_generics ::postgresql::RowShape for #name #ty_generics #row_shape_where {
            const SHAPE: &'static [::postgresql::ColumnSpec] = &[ #(#shape_entries)* ];
        }
    };
    Ok(expanded)
}

fn apply_rename(field: &str, rename_all: Option<&str>) -> String {
    match rename_all {
        Some("camelCase") => to_camel_case(field),
        Some("PascalCase") => to_pascal_case(field),
        Some("SCREAMING_SNAKE_CASE") | Some("UPPERCASE") => field.to_ascii_uppercase(),
        Some("lowercase") => field.to_ascii_lowercase(),
        Some("snake_case") | None => to_snake_case(field),
        Some(_) => field.to_string(),
    }
}

fn to_camel_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut upper = false;
    for c in s.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.extend(c.to_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

fn to_pascal_case(s: &str) -> String {
    let camel = to_camel_case(s);
    let mut chars = camel.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(c.to_lowercase());
    }
    out
}

/// Derives [`postgresql::ToRow`] for a struct with named fields.
///
/// Fields are encoded in declaration order, matching a binary `COPY` column
/// list.
#[proc_macro_derive(ToRow, attributes(postgresql, pg))]
pub fn derive_to_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_to_row(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_to_row(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "ToRow can only be derived for structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(input, "ToRow can only be derived for structs"));
        }
    };

    let encoders = fields.iter().map(|field| {
        let ident = field.ident.as_ref().expect("named field");
        quote! { &self.#ident as &dyn ::postgresql::ToSql }
    });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics ::postgresql::ToRow for #name #ty_generics #where_clause {
            fn encode_row(
                &self,
                buf: &mut ::postgresql::__private::BytesMut,
            ) -> ::core::result::Result<(), ::postgresql::Error> {
                ::postgresql::encode_copy_row(&[ #(#encoders),* ], buf)
            }
        }
    })
}

/// Derives `json` / `jsonb` encoding and decoding for a type.
///
/// The type must also derive `serde::Serialize` (to be written as a parameter
/// or `COPY` field) and `serde::Deserialize` (to be read from a result
/// column). The generated impls map the whole value to a single `jsonb`
/// column: `FromSql`, `ToSql`, `StaticType`, `FromRow` and `RowShape`.
///
/// Because it defines `FromRow` and `RowShape`, a type cannot derive both
/// `Json` and `FromRow`; use `postgresql::Json<T>` for the JSON role in that
/// case.
#[proc_macro_derive(Json, attributes(postgresql, pg))]
pub fn derive_json(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_json(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_json(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
    let self_ty = quote! { #name #ty_generics };

    let deserialize: WherePredicate = syn::parse_quote! {
        #self_ty: ::postgresql::__private::serde::de::DeserializeOwned
    };
    let serialize: WherePredicate = syn::parse_quote! {
        #self_ty: ::postgresql::__private::serde::Serialize
    };
    let send_sync: WherePredicate = syn::parse_quote! {
        #self_ty: ::core::marker::Send + ::core::marker::Sync
    };

    let from_sql_where = merge_where(&input.generics, [deserialize.clone()]);
    let from_row_where = merge_where(&input.generics, [deserialize]);
    let to_sql_where = merge_where(&input.generics, [serialize, send_sync]);
    let plain_where = merge_where(&input.generics, []);

    Ok(quote! {
        impl #impl_generics ::postgresql::FromSql for #self_ty #from_sql_where {
            fn from_sql(
                oid: ::postgresql::types::Oid,
                buf: &[u8],
            ) -> ::core::result::Result<Self, ::postgresql::Error> {
                ::postgresql::__private::json_from_sql(oid, buf)
            }

            fn accepts(oid: ::postgresql::types::Oid) -> bool {
                ::postgresql::__private::json_accepts(oid)
            }
        }

        impl #impl_generics ::postgresql::FromRow for #self_ty #from_row_where {
            fn from_row(
                row: &::postgresql::Row,
            ) -> ::core::result::Result<Self, ::postgresql::Error> {
                ::postgresql::Row::try_get_by_index(row, 0usize)
            }
        }

        impl #impl_generics ::postgresql::ToSql for #self_ty #to_sql_where {
            fn encode(
                &self,
                buf: &mut ::postgresql::__private::BytesMut,
            ) -> ::core::result::Result<::postgresql::IsNull, ::postgresql::Error> {
                ::postgresql::__private::json_encode(self, buf)
            }

            fn oid(&self) -> ::postgresql::types::Oid {
                ::postgresql::__private::JSONBOID
            }
        }

        impl #impl_generics ::postgresql::StaticType for #self_ty #plain_where {
            const OID: ::postgresql::types::Oid = ::postgresql::__private::JSONBOID;
        }

        impl #impl_generics ::postgresql::RowShape for #self_ty #plain_where {
            const SHAPE: &'static [::postgresql::ColumnSpec] = &[::postgresql::ColumnSpec {
                name: "",
                type_oid: ::postgresql::__private::JSONBOID,
                nullable: false,
                known: true,
            }];
        }
    })
}
