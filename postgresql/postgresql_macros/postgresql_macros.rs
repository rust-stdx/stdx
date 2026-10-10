//! Compile-time checked query macros for the `postgresql` crate.
//!
//! `query!` and `query_as!` build a query and, when `DATABASE_URL` is set at
//! build time (and `STDX_POSTGRESQL_CHECK` is not `false`), connect to the
//! database, describe the statement and check that the parameters match the
//! server-inferred types. When the SQL is not a string literal, or checking is
//! disabled, the same code compiles and is decoded at runtime.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod describe;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Lit, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct QueryInput {
    sql: Expr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for QueryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let sql: Expr = input.parse()?;
        let args = parse_trailing_args(input)?;
        Ok(QueryInput {
            sql,
            args,
        })
    }
}

struct QueryAsInput {
    ty: Type,
    sql: Expr,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for QueryAsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let sql: Expr = input.parse()?;
        let args = parse_trailing_args(input)?;
        Ok(QueryAsInput {
            ty,
            sql,
            args,
        })
    }
}

fn parse_trailing_args(input: ParseStream) -> syn::Result<Punctuated<Expr, Token![,]>> {
    if input.is_empty() {
        return Ok(Punctuated::new());
    }
    input.parse::<Token![,]>()?;
    if input.is_empty() {
        return Ok(Punctuated::new());
    }
    Punctuated::parse_terminated(input)
}

/// Builds an untyped query. See the crate documentation.
#[proc_macro]
pub fn query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as QueryInput);
    match expand(input.sql, input.args, None) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Builds a query decoded into a named type. See the crate documentation.
#[proc_macro]
pub fn query_as(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as QueryAsInput);
    match expand(input.sql, input.args, Some(input.ty)) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn checking_enabled() -> Option<String> {
    if let Ok(flag) = std::env::var("STDX_POSTGRESQL_CHECK") {
        if flag.eq_ignore_ascii_case("false") {
            return None;
        }
    }
    std::env::var("DATABASE_URL").ok().filter(|url| !url.is_empty())
}

fn sql_literal(sql: &Expr) -> Option<String> {
    if let Expr::Lit(lit) = sql {
        if let Lit::Str(s) = &lit.lit {
            return Some(s.value());
        }
    }
    None
}

fn expand(sql: Expr, args: Punctuated<Expr, Token![,]>, target: Option<Type>) -> syn::Result<proc_macro2::TokenStream> {
    let described = match (checking_enabled(), sql_literal(&sql)) {
        (Some(url), Some(text)) => Some(
            describe::describe(&url, &text).map_err(|e| syn::Error::new_spanned(&sql, format!("postgresql: {e}")))?,
        ),
        _ => None,
    };

    if let Some(described) = &described {
        if described.param_oids.len() != args.len() {
            return Err(syn::Error::new_spanned(
                &sql,
                format!(
                    "postgresql: query expects {} parameter(s) but {} were provided",
                    described.param_oids.len(),
                    args.len()
                ),
            ));
        }
    }

    let params: Vec<proc_macro2::TokenStream> = args
        .iter()
        .enumerate()
        .map(|(index, arg)| match &described {
            Some(described) => {
                let oid = described.param_oids[index];
                quote! {
                    ::postgresql::__private::param::<#oid, _>(&(#arg)) as &dyn ::postgresql::ToSql
                }
            }
            None => quote! {
                &(#arg) as &dyn ::postgresql::ToSql
            },
        })
        .collect();

    let params_slice = quote! { &[ #(#params),* ] };

    let tokens = match target {
        Some(ty) => {
            let base = quote! {
                ::postgresql::QueryAs::<#ty>::new(#sql, #params_slice)
            };
            match &described {
                Some(described) => {
                    let specs = described.columns.iter().map(|column| {
                        let name = column.name.as_str();
                        let oid = column.type_oid;
                        let nullable = column.nullable;
                        let known = column.nullable_known;
                        quote! {
                            ::postgresql::ColumnSpec {
                                name: #name,
                                type_oid: #oid,
                                nullable: #nullable,
                                known: #known,
                            },
                        }
                    });
                    quote! {
                        (
                            const {
                                ::postgresql::__private::verify_shape(
                                    <#ty as ::postgresql::RowShape>::SHAPE,
                                    &[ #(#specs)* ],
                                )
                            },
                            #base,
                        ).1
                    }
                }
                None => base,
            }
        }
        None => quote! {
            ::postgresql::Query::new(#sql, #params_slice)
        },
    };
    Ok(tokens)
}

struct CopyInput {
    conn: Expr,
    ty: Type,
    sql: Expr,
}

impl Parse for CopyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let conn: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![,]>()?;
        let sql: Expr = input.parse()?;
        Ok(CopyInput {
            conn,
            ty,
            sql,
        })
    }
}

/// Builds a compile-time checked binary `COPY ... FROM STDIN`.
///
/// Usage: `copy_in!(conn, RowType, "COPY t (a, b) FROM STDIN BINARY").await?`.
#[proc_macro]
pub fn copy_in(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CopyInput);
    match expand_copy(input, false) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Builds a compile-time checked binary `COPY ... TO STDOUT`.
///
/// Usage: `copy_out!(conn, RowType, "COPY t TO STDOUT BINARY").await?`.
#[proc_macro]
pub fn copy_out(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CopyInput);
    match expand_copy(input, true) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand_copy(input: CopyInput, out: bool) -> syn::Result<proc_macro2::TokenStream> {
    let columns = match (checking_enabled(), sql_literal(&input.sql)) {
        (Some(url), Some(text)) => Some(
            describe::copy_columns(&url, &text)
                .map_err(|e| syn::Error::new_spanned(&input.sql, format!("postgresql: {e}")))?,
        ),
        _ => None,
    };

    let ty = &input.ty;
    let conn = &input.conn;
    let sql = &input.sql;

    let check = columns.as_ref().map(|columns| {
        let specs = columns.iter().map(|column| {
            let name = column.name.as_str();
            let oid = column.type_oid;
            let nullable = column.nullable;
            let known = column.nullable_known;
            quote! {
                ::postgresql::ColumnSpec {
                    name: #name,
                    type_oid: #oid,
                    nullable: #nullable,
                    known: #known,
                },
            }
        });
        quote! {
            const {
                ::postgresql::__private::verify_shape(
                    <#ty as ::postgresql::RowShape>::SHAPE,
                    &[ #(#specs)* ],
                )
            }
        }
    });

    let call = if out {
        let oids: Vec<u32> = columns
            .as_ref()
            .map(|columns| columns.iter().map(|c| c.type_oid).collect())
            .unwrap_or_default();
        quote! {
            ::postgresql::__private::copy_out::<#ty>(&(#conn), #sql, &[ #(#oids),* ])
        }
    } else {
        quote! {
            ::postgresql::__private::copy_in::<#ty>(&(#conn), #sql)
        }
    };

    Ok(match check {
        Some(check) => quote! { ( #check, #call ).1 },
        None => call,
    })
}
