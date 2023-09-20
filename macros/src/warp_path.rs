use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitStr, Result, Token, Type};

enum Segment {
    Param(Type),
    Static(LitStr),
}

impl Parse for Segment {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            return Ok(Segment::Static(input.parse()?));
        }
        let Ok(ty) = input.parse() else {
            return Err(input.error("segment must be either type or str-literal"));
        };

        Ok(Segment::Param(ty))
    }
}

struct WarpPath {
    segments: Punctuated<Segment, Token![/]>,
}

impl Parse for WarpPath {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(WarpPath {
            segments: input.parse_terminated(Segment::parse, Token![/])?,
        })
    }
}

pub(crate) fn warp_path(input: TokenStream) -> TokenStream {
    let WarpPath { segments } = parse_macro_input!(input as _);
    let mut codes = Vec::with_capacity(segments.len());
    for segment in segments {
        match segment {
            Segment::Param(ty) => codes.push(quote!(::warp::path::param::<#ty>())),

            Segment::Static(lit) => codes.push(quote! {
                ::warp::path({
                    #[derive(Clone, Copy)]
                    struct __StaticPath;
                    impl ::core::convert::AsRef<str> for __StaticPath {
                        fn as_ref(&self) -> &str {
                            static S: &str = #lit;
                            S
                        }
                    }
                    __StaticPath
                })
            }),
        }
    }
    codes.push(quote!(::warp::path::end()));
    let mut codes = codes.into_iter();
    let mut result = {
        let code = codes.next().unwrap();
        quote! { #code }
    };
    for code in codes {
        result = quote! { #result.and(#code) }
    }

    result.into()
}
