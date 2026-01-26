use proc_macro::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use syn::{Fields, Ident, ItemEnum, LitInt, Token, Type, parse::Parse, parse_macro_input};

fn compute_discriminator_bytes(name: &str, take: usize) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(b"event:");
    hasher.update(name.as_bytes());
    hasher.finalize().into_iter().take(take).collect()
}

struct Args {
    discriminator: u8,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        if ident != "discriminator" {
            return Err(input.error("expected `discriminator = <u8>`"));
        }

        input.parse::<Token![=]>()?;

        let value: LitInt = input.parse()?;

        let discriminator = value.base10_parse::<u8>()?;

        Ok(Self { discriminator })
    }
}

#[proc_macro_attribute]
pub fn anchor_events(attr: TokenStream, item: TokenStream) -> TokenStream {
    let Args { discriminator } = parse_macro_input!(attr as Args);

    let input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;

    let mut statics = Vec::new();
    let mut match_arms = Vec::new();

    for variant in &input.variants {
        let variant_ident = &variant.ident;

        let inner_ty = match &variant.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => &f.unnamed[0].ty,
            _ => {
                return syn::Error::new_spanned(
                    variant,
                    "Each variant must have exactly one unnamed field",
                )
                .to_compile_error()
                .into();
            }
        };

        let inner_ident = match inner_ty {
            Type::Path(p) => &p.path.segments.last().unwrap().ident,
            _ => {
                return syn::Error::new_spanned(
                    inner_ty,
                    "Inner event type must be a named struct",
                )
                .to_compile_error()
                .into();
            }
        };

        let disc_ident = syn::Ident::new(
            &format!("{}_DISC", variant_ident.to_string().to_uppercase()),
            variant_ident.span(),
        );

        let bytes = compute_discriminator_bytes(&inner_ident.to_string(), discriminator as usize);

        let byte_tokens = bytes.iter().map(|b| quote! { #b });

        statics.push(quote! {
            static #disc_ident: [u8; #discriminator as usize] = [
                #(#byte_tokens),*
            ];
        });

        match_arms.push(quote! {
            d if d == #disc_ident => {
                Some(#enum_name::#variant_ident(
                    #inner_ty::try_from_slice(body).ok()?
                ))
            }
        });
    }

    let expanded = quote! {
        #input

        #(#statics)*

        impl #enum_name {
            pub fn from_logs<T, I>(logs: T) -> Vec<Self>
            where
                T: IntoIterator<Item = I>,
                I: AsRef<str>, {
                use borsh::BorshDeserialize;
                use base64::{Engine, prelude::BASE64_STANDARD};

                logs.into_iter()
                    .filter_map(|log| {
                        let data = log.as_ref().strip_prefix("Program data: ")?;
                        let bytes = BASE64_STANDARD.decode(data).ok()?;
                        let (disc, body) = bytes.split_at(#discriminator as usize);

                        match disc {
                            #(#match_arms)*
                            _ => None,
                        }
                    })
                    .collect()
            }
        }
    };

    expanded.into()
}
