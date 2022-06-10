use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token, Type,
};

pub fn provide(input: ProvideInput) -> syn::Result<TokenStream> {
    let type_name = format_ident!("InlineProvider");

    let mut types = Vec::new();
    let mut exprs = Vec::new();
    let mut provide_impls = Vec::new();

    for (index, type_spec) in input.type_specs.iter().enumerate() {
        let index = syn::Index::from(index);
        let ty_ = &type_spec.ty_;
        let expr = &type_spec.expr;

        types.push(ty_);
        exprs.push(expr);

        provide_impls.push(quote! {
            impl ::injectable::Provide::<#ty_> for #type_name {
                fn provide(&self) -> #ty_ {
                    ::core::clone::Clone::clone(&self.#index)
                }
            }
        })
    }

    Ok(quote! {
        {
            struct #type_name(
                #( #types, )*
            );

            #( #provide_impls )*

            #type_name(
                #( #exprs, )*
            )
        }
    })
}

#[derive(Default)]
pub struct ProvideInput {
    type_specs: Punctuated<TypeSpec, Token![,]>,
}

impl Parse for ProvideInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(ProvideInput::default());
        }

        Ok(Self {
            type_specs: Punctuated::parse_terminated(input)?,
        })
    }
}

pub struct TypeSpec {
    ty_: Type,
    arrow: Token![=>],
    expr: Expr,
}

impl Parse for TypeSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            ty_: input.parse()?,
            arrow: input.parse()?,
            expr: input.parse()?,
        })
    }
}
