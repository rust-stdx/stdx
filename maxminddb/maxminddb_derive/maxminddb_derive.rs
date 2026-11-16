//! Derive macros for the `maxminddb` crate.
//!
//! `#[derive(Decode, Encode)]` generates the `maxminddb::Decode` and
//! `maxminddb::Encode` implementations.
//!
//! Named structs are encoded as maps. Fields whose type is `Option<T>` are
//! omitted when they are `None` and default to `None` when the key is absent.
//! Unknown map keys are skipped during decoding.
//!
//! Supported container attributes:
//!
//! - `#[maxminddb(bound = "...")]`
//! - `#[maxminddb(crate = "...")]`
//!
//! Supported field attributes:
//!
//! - `#[maxminddb(rename = "...")]`
//! - `#[maxminddb(skip)]`

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DeriveInput, Fields, GenericArgument, GenericParam, Lifetime, LitStr, Path, PathArguments, Token,
    Type, TypeParamBound, WherePredicate, parse::Parser, parse_macro_input, punctuated::Punctuated, spanned::Spanned,
};

/// Derive `maxminddb::Decode`.
#[proc_macro_derive(Decode, attributes(maxminddb))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_decode(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

/// Derive `maxminddb::Encode`.
#[proc_macro_derive(Encode, attributes(maxminddb))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_encode(input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

#[derive(Default)]
struct ContainerAttrs {
    krate: Option<Path>,
    bound: Option<String>,
}

#[derive(Default, Clone)]
struct FieldAttrs {
    rename: Option<String>,
    skip: bool,
}

fn parse_container(attrs: &[Attribute]) -> syn::Result<ContainerAttrs> {
    let mut out = ContainerAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("maxminddb") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                let value: LitStr = meta.value()?.parse()?;
                out.krate = Some(
                    syn::parse_str(&value.value())
                        .map_err(|e| syn::Error::new(value.span(), format!("invalid crate path: {e}")))?,
                );
            } else if meta.path.is_ident("bound") {
                let value: LitStr = meta.value()?.parse()?;
                out.bound = Some(value.value());
            } else {
                return Err(meta.error("unknown maxminddb container attribute"));
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn parse_field(attrs: &[Attribute]) -> syn::Result<FieldAttrs> {
    let mut out = FieldAttrs::default();
    for attr in attrs {
        if !attr.path().is_ident("maxminddb") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                out.skip = true;
            } else if meta.path.is_ident("rename") {
                let value: LitStr = meta.value()?.parse()?;
                out.rename = Some(value.value());
            } else {
                return Err(meta.error("unknown maxminddb field attribute"));
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
                "maxminddb derive currently supports only named structs",
            )),
            Fields::Unit => Err(syn::Error::new(
                data.struct_token.span(),
                "maxminddb derive currently supports only named structs",
            )),
        },
        Data::Enum(data) => Err(syn::Error::new(
            data.enum_token.span(),
            "maxminddb derive currently supports only named structs",
        )),
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span(),
            "maxminddb derive currently supports only named structs",
        )),
    }
}

fn add_type_bounds(
    mut generics: syn::Generics,
    bound: Option<&str>,
    krate: &Path,
    trait_name: TokenStream2,
) -> syn::Result<syn::Generics> {
    if let Some(bound) = bound {
        let parser = Punctuated::<WherePredicate, Token![,]>::parse_terminated;
        let predicates = parser
            .parse_str(bound)
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("invalid bound: {e}")))?;
        generics.make_where_clause().predicates.extend(predicates);
    } else {
        for param in &mut generics.params {
            if let GenericParam::Type(param) = param {
                let bound: TypeParamBound = syn::parse_quote!(#krate::#trait_name);
                param.bounds.push(bound);
            }
        }
    }
    Ok(generics)
}

fn expand_decode(input: DeriveInput) -> syn::Result<TokenStream2> {
    let fields = named_fields(&input.data)?;
    let container = parse_container(&input.attrs)?;
    let krate: Path = container
        .krate
        .clone()
        .unwrap_or_else(|| syn::parse_quote!(::maxminddb));
    let name = &input.ident;

    let de_lifetime: Lifetime = syn::parse_quote!('__maxminddb_de);
    let original_lifetimes = input
        .generics
        .lifetimes()
        .map(|param| param.lifetime.clone())
        .collect::<Vec<_>>();

    let mut generics = input.generics.clone();
    generics
        .params
        .insert(0, GenericParam::Lifetime(syn::LifetimeParam::new(de_lifetime.clone())));
    {
        let where_clause = generics.make_where_clause();
        for lifetime in &original_lifetimes {
            where_clause.predicates.push(syn::parse_quote!(#de_lifetime: #lifetime));
        }
    }
    let generics = add_type_bounds(generics, container.bound.as_deref(), &krate, quote!(Decode<#de_lifetime>))?;
    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let (_, ty_generics, _) = input.generics.split_for_impl();

    let mut declarations = Vec::new();
    let mut decode_arms = Vec::new();
    let mut initializers = Vec::new();

    for (index, field) in fields.named.iter().enumerate() {
        let attrs = parse_field(&field.attrs)?;
        let ident = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new(field.span(), "expected a named field"))?;
        let key = attrs.rename.clone().unwrap_or_else(|| ident.to_string());
        let storage = format_ident!("__field_{index}");
        let field_name = ident.to_string();

        if attrs.skip {
            initializers.push(quote! {
                #ident: ::core::default::Default::default(),
            });
            continue;
        }

        let ty = &field.ty;
        let storage_ty = option_inner(ty).unwrap_or(ty);
        declarations.push(quote! {
            let mut #storage: Option<#storage_ty> = None;
        });
        decode_arms.push(quote! {
            #key => {
                if #storage.is_some() {
                    return ::core::result::Result::Err(
                        #krate::MaxMindDBError::DecodingError(
                            ::std::format!("duplicate field `{}`", #field_name)
                        )
                    );
                }
                #storage = ::core::option::Option::Some(
                    __map.next_value::<#storage_ty>()?
                );
            }
        });

        if option_inner(ty).is_some() {
            initializers.push(quote! {
                #ident: #storage,
            });
        } else {
            initializers.push(quote! {
                #ident: match #storage {
                    ::core::option::Option::Some(value) => value,
                    ::core::option::Option::None => {
                        return ::core::result::Result::Err(
                            #krate::MaxMindDBError::DecodingError(
                                ::std::format!("missing field `{}`", #field_name)
                            )
                        );
                    }
                },
            });
        }
    }

    Ok(quote! {
        impl #impl_generics #krate::Decode<#de_lifetime> for #name #ty_generics #where_clause {
            fn decode(
                decoder: &mut #krate::decoder::Decoder<#de_lifetime>,
            ) -> #krate::decoder::DecodeResult<Self> {
                #(#declarations)*
                decoder.with_map(|__map| {
                    while let ::core::option::Option::Some(__key) = __map.next_key()? {
                        match __key {
                            #(#decode_arms)*
                            _ => __map.skip_value()?,
                        }
                    }
                    ::core::result::Result::Ok(())
                })?;
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
    let krate: Path = container
        .krate
        .clone()
        .unwrap_or_else(|| syn::parse_quote!(::maxminddb));
    let name = &input.ident;
    let generics = add_type_bounds(input.generics.clone(), container.bound.as_deref(), &krate, quote!(Encode))?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut count = Vec::new();
    let mut entries = Vec::new();

    for field in &fields.named {
        let attrs = parse_field(&field.attrs)?;
        if attrs.skip {
            continue;
        }
        let ident = field
            .ident
            .as_ref()
            .ok_or_else(|| syn::Error::new(field.span(), "expected a named field"))?;
        let key = attrs.rename.clone().unwrap_or_else(|| ident.to_string());

        if option_inner(&field.ty).is_some() {
            count.push(quote! {
                if self.#ident.is_some() {
                    __map_len += 1;
                }
            });
            entries.push(quote! {
                if let ::core::option::Option::Some(__value) = &self.#ident {
                    encoder.str(#key)?;
                    #krate::Encode::encode(__value, encoder)?;
                }
            });
        } else {
            count.push(quote! {
                __map_len += 1;
            });
            entries.push(quote! {
                encoder.str(#key)?;
                #krate::Encode::encode(&self.#ident, encoder)?;
            });
        }
    }

    Ok(quote! {
        impl #impl_generics #krate::Encode for #name #ty_generics #where_clause {
            fn encode(&self, encoder: &mut #krate::encoder::Encoder) -> ::core::result::Result<(), #krate::encoder::SerError> {
                let mut __map_len = 0usize;
                #(#count)*
                encoder.map(__map_len)?;
                #(#entries)*
                ::core::result::Result::Ok(())
            }
        }
    })
}

fn option_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    args.args.iter().find_map(|arg| match arg {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    })
}
