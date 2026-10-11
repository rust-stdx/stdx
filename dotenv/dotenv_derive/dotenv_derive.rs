use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    Data, DeriveInput, Expr, Fields, GenericArgument, LitStr, Path, PathArguments, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro_derive(FromEnv, attributes(env))]
pub fn derive_from_env(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(&input) {
        Ok(tokens) => tokens.into(),
        // Emit proper `compile_error!`s instead of panicking.
        Err(err) => err.to_compile_error().into(),
    }
}

fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "FromEnv only supports structs with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(input, "FromEnv only supports structs"));
        }
    };

    let mut field_assignments = Vec::new();
    for f in fields {
        let Some(field_name) = f.ident.as_ref() else {
            continue;
        };
        let field_name_str = field_name.to_string();
        let screaming = to_screaming_snake(&field_name_str);
        let ty = &f.ty;

        let mut attrs = Vec::new();
        for a in f.attrs.iter().filter(|a| a.path().is_ident("env")) {
            attrs.push(a.parse_args::<EnvAttr>()?);
        }
        if attrs.len() > 1 {
            return Err(syn::Error::new(
                field_name.span(),
                "multiple `#[env(...)]` attributes on one field; combine them into a single attribute",
            ));
        }
        let attr = attrs.first();

        let expr = field_expr(field_name, &screaming, ty, attr)?;
        field_assignments.push(quote! { #field_name: #expr });
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::dotenv::FromEnv for #name #ty_generics #where_clause {
            fn from_env_with_prefix(prefix: &str) -> ::std::result::Result<Self, ::dotenv::FromEnvError> {
                Ok(Self {
                    #(#field_assignments,)*
                })
            }
        }
    })
}

/// Build the value expression for a single field.
fn field_expr(
    field_name: &syn::Ident,
    screaming: &str,
    ty: &Type,
    attr: Option<&EnvAttr>,
) -> syn::Result<TokenStream2> {
    let inner = option_inner(ty);

    let (rename, default, with) = match attr {
        Some(EnvAttr {
            rename,
            default,
            with,
        }) => {
            if with.is_some() && default.is_some() {
                return Err(syn::Error::new(field_name.span(), "`with` cannot be combined with `default`"));
            }
            (rename.as_ref(), default.as_ref(), with.as_ref())
        }
        None => (None, None, None),
    };

    // `#[env(rename = "VAR", with = "func")]`
    if let (Some(var), None, Some(path)) = (rename, default, with) {
        return Ok(quote! {{
            let val = ::std::env::var(#var)
                .map_err(|_| ::dotenv::FromEnvError::missing(#var))?;
            #path(#var, &val)?
        }});
    }

    // `#[env(with = "func")]`
    if let (None, None, Some(path)) = (rename, default, with) {
        return Ok(quote! {{
            let var_name = ::std::format!("{}{}", prefix, #screaming);
            let val = ::std::env::var(&var_name)
                .map_err(|_| ::dotenv::FromEnvError::missing(var_name.clone()))?;
            #path(&var_name, &val)?
        }});
    }

    if let Some(inner) = inner {
        // `Option<T>` fields are optional.
        return Ok(match (rename, default) {
            (Some(var), None) => quote! {
                match ::std::env::var(#var) {
                    ::std::result::Result::Ok(val) if !val.is_empty() => ::std::option::Option::Some(
                        <#inner as ::dotenv::FromEnvValue>::from_env_value(&val)
                            .map_err(|e| ::dotenv::FromEnvError::invalid(#var, val, e))?
                    ),
                    _ => ::std::option::Option::None,
                }
            },
            (Some(var), Some(kind)) => {
                let fallback = fallback_tokens(kind);
                quote! {
                    match ::std::env::var(#var) {
                        ::std::result::Result::Ok(val) if !val.is_empty() => ::std::option::Option::Some(
                            <#inner as ::dotenv::FromEnvValue>::from_env_value(&val)
                                .map_err(|e| ::dotenv::FromEnvError::invalid(#var, val, e))?
                        ),
                        _ => #fallback,
                    }
                }
            }
            (None, Some(kind)) => {
                let fallback = fallback_tokens(kind);
                quote! {{
                    let var_name = ::std::format!("{}{}", prefix, #screaming);
                    match ::std::env::var(&var_name) {
                        ::std::result::Result::Ok(val) if !val.is_empty() => ::std::option::Option::Some(
                            <#inner as ::dotenv::FromEnvValue>::from_env_value(&val)
                                .map_err(|e| ::dotenv::FromEnvError::invalid(var_name.as_str(), val, e))?
                        ),
                        _ => #fallback,
                    }
                }}
            }
            (None, None) => quote! {{
                let var_name = ::std::format!("{}{}", prefix, #screaming);
                match ::std::env::var(&var_name) {
                    ::std::result::Result::Ok(val) if !val.is_empty() => ::std::option::Option::Some(
                        <#inner as ::dotenv::FromEnvValue>::from_env_value(&val)
                            .map_err(|e| ::dotenv::FromEnvError::invalid(var_name.as_str(), val, e))?
                    ),
                    _ => ::std::option::Option::None,
                }
            }},
        });
    }

    Ok(match (rename, default) {
        // `#[env(rename = "VAR")]`
        (Some(var), None) => quote! {{
            let val = ::std::env::var(#var)
                .map_err(|_| ::dotenv::FromEnvError::missing(#var))?;
            <#ty as ::dotenv::FromEnvValue>::from_env_value(&val)
                .map_err(|e| ::dotenv::FromEnvError::invalid(#var, val, e))?
        }},
        // `#[env(rename = "VAR", default)]` / `#[env(rename = "VAR", default = expr)]`
        (Some(var), Some(kind)) => {
            let fallback = fallback_tokens(kind);
            quote! {
                match ::std::env::var(#var) {
                    ::std::result::Result::Ok(val) => <#ty as ::dotenv::FromEnvValue>::from_env_value(&val)
                        .map_err(|e| ::dotenv::FromEnvError::invalid(#var, val, e))?,
                    ::std::result::Result::Err(_) => #fallback,
                }
            }
        }
        // `#[env(default)]` / `#[env(default = expr)]` (no rename):
        // read the derived variable name first, then fall back.
        (None, Some(kind)) => {
            let fallback = fallback_tokens(kind);
            quote! {
                match <#ty as ::dotenv::FromEnvAuto>::from_env_auto(
                    &::std::format!("{}{}_", prefix, #screaming),
                    &::std::format!("{}{}", prefix, #screaming),
                ) {
                    ::std::result::Result::Ok(v) => v,
                    ::std::result::Result::Err(::dotenv::FromEnvError::Missing(_)) => #fallback,
                    ::std::result::Result::Err(e) => return ::std::result::Result::Err(e),
                }
            }
        }
        // No attribute (or `#[env()]`): auto-dispatch.
        (None, None) => quote! {
            <#ty as ::dotenv::FromEnvAuto>::from_env_auto(
                &::std::format!("{}{}_", prefix, #screaming),
                &::std::format!("{}{}", prefix, #screaming),
            )?
        },
    })
}

fn fallback_tokens(kind: &DefaultKind) -> TokenStream2 {
    match kind {
        DefaultKind::Standard => quote!(::std::default::Default::default()),
        DefaultKind::Expr(expr) => quote!(#expr),
    }
}

/// If `ty` is `Option<T>`, return `T`.
fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(tp) = ty else { return None };
    if tp.qself.is_some() {
        return None;
    }
    let seg = tp.path.segments.last()?;
    if seg.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    if args.args.len() != 1 {
        return None;
    }
    match &args.args[0] {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

fn to_screaming_snake(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if i > 0 && chars[i].is_ascii_uppercase() {
            let prev_lower = chars[i - 1].is_ascii_lowercase();
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_ascii_lowercase();
            if prev_lower || (next_lower && i > 0 && chars[i - 1].is_ascii_uppercase()) {
                result.push('_');
            }
        }
        result.push(chars[i].to_ascii_uppercase());
        i += 1;
    }
    result
}

struct EnvAttr {
    rename: Option<LitStr>,
    default: Option<DefaultKind>,
    with: Option<Path>,
}

enum DefaultKind {
    Standard,
    Expr(Expr),
}

impl Parse for EnvAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rename = None;
        let mut default = None;
        let mut with = None;

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            match ident.to_string().as_str() {
                "rename" => {
                    input.parse::<Token![=]>()?;
                    rename = Some(input.parse()?);
                }
                "default" => {
                    if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                        default = Some(DefaultKind::Expr(input.parse()?));
                    } else {
                        default = Some(DefaultKind::Standard);
                    }
                }
                "with" => {
                    input.parse::<Token![=]>()?;
                    let lit: LitStr = input.parse()?;
                    let path = syn::parse_str::<Path>(&lit.value()).map_err(|e| syn::Error::new_spanned(&lit, e))?;
                    with = Some(path);
                }
                other => {
                    return Err(syn::Error::new(ident.span(), format!("unknown env attribute: `{other}`")));
                }
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(EnvAttr {
            rename,
            default,
            with,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_screaming_snake() {
        assert_eq!(to_screaming_snake("url"), "URL");
        assert_eq!(to_screaming_snake("my_field"), "MY_FIELD");
        assert_eq!(to_screaming_snake("myField"), "MY_FIELD");
        assert_eq!(to_screaming_snake("XMLParser"), "XML_PARSER");
        assert_eq!(to_screaming_snake("database_url"), "DATABASE_URL");
        assert_eq!(to_screaming_snake("db"), "DB");
        assert_eq!(to_screaming_snake("a"), "A");
        assert_eq!(to_screaming_snake(""), "");
    }
}
