#![allow(dead_code)]

mod macros;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn inject(args: TokenStream, input: TokenStream) -> TokenStream {
    macros::inject::inject(parse_macro_input!(args), {
        let input = input.clone();
        parse_macro_input!(input)
    })
    .map(|output| output.into_token_stream())
    .unwrap_or_else(|err| {
        let input = proc_macro2::TokenStream::from(input);
        let err = err.to_compile_error();
        quote! {
            #input
            #err
        }
    })
    .into()
}

#[proc_macro]
pub fn provide(input: TokenStream) -> TokenStream {
    macros::provide::provide(parse_macro_input!(input))
        .map(|output| output.into_token_stream())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
