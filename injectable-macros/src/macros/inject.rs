use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, FnArg, ItemFn, Pat, PatType,
};

pub fn inject(_args: InjectArgs, mut input: ItemFn) -> syn::Result<TokenStream> {
    let deps_name = quote::format_ident!("deps__injectable");
    let deps_type = quote::format_ident!("Deps__injectable");

    let args = std::mem::take(&mut input.sig.inputs);
    let mut provided_types = Vec::new();
    let mut stmts = Vec::new();

    for arg in args {
        match arg {
            FnArg::Receiver(_) => {
                input.sig.inputs.push(arg);
            }

            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                if let Pat::Wild(_) = &*pat {
                    // for optimization, we'll ignore wildcard patterns
                    // even though this may elimite side effects
                } else {
                    stmts.push(parse_quote! {
                        #( #attrs )*
                        let #pat : #ty = ::injectable::Provide::provide(&#deps_name);
                    });
                }
                provided_types.push(ty);
            }
        }
    }

    input.sig.generics.params.insert(
        0,
        parse_quote! ( #deps_type : Sized #(+ ::injectable::Provide::<#provided_types> )* ),
    );

    input
        .sig
        .inputs
        .push(parse_quote!( #deps_name : #deps_type ));

    stmts.push(parse_quote! {
        #[allow(unused_macros)]
        macro_rules! inject {
            () => { &#deps_name };
        }
    });
    stmts.extend(input.block.stmts);
    input.block.stmts = stmts;

    Ok(quote! {
        #input
    })
}

pub struct InjectArgs {}

impl Parse for InjectArgs {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}
