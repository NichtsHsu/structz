use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, punctuated::Punctuated};

mod parse;

use parse::*;

#[proc_macro]
pub fn stru(input: TokenStream) -> TokenStream {
    let AnonymousStruct(input) = parse_macro_input!(input as AnonymousStruct);
    let fields: Vec<_> = input
        .into_iter()
        .map(|(ident, expr)| match expr {
            Some(expr) => quote! { (<::stringz::ident!(#ident)>::default(), #expr) },
            None => quote! { (<::stringz::ident!(#ident)>::default(), #ident) },
        })
        .collect();
    quote! {
        ::tuplez::tuple!(#(#fields),*)
    }
    .into()
}

#[proc_macro]
pub fn stru_t(input: TokenStream) -> TokenStream {
    let AnonymousStructType(input) = parse_macro_input!(input as AnonymousStructType);
    let fields: Vec<_> = input
        .into_iter()
        .map(|(ident, ty)| quote! { (::stringz::ident!(#ident), #ty) })
        .collect();
    quote! {
        ::tuplez::tuple_t!(#(#fields),*)
    }
    .into()
}

#[proc_macro_attribute]
pub fn named_args(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemFn);
    let mut args = vec![];
    let mut has_self = None;
    for arg in input.sig.inputs {
        match arg {
            syn::FnArg::Receiver(arg) => has_self = Some(arg),
            syn::FnArg::Typed(arg) => {
                if let syn::Pat::Ident(pat) = *arg.pat {
                    args.push((pat, arg.ty))
                } else {
                    return quote! {
                       compile_error!("arguments must be an identifier or receiver");
                    }
                    .into();
                }
            }
        }
    }
    args.sort_by(|x, y| x.0.ident.cmp(&y.0.ident));
    let (idents, tys): (Vec<syn::PatIdent>, Vec<Box<syn::Type>>) = args.into_iter().unzip();
    let struct_type: syn::Type = parse_quote! {
        structz::stru_t! { #(#idents: #tys),* }
    };
    let mut inputs = Punctuated::new();
    if let Some(arg) = has_self {
        inputs.push(syn::FnArg::Receiver(arg));
    }
    inputs.push(syn::FnArg::Typed(syn::PatType {
        attrs: vec![],
        pat: parse_quote! { structz_s },
        colon_token: Default::default(),
        ty: Box::new(struct_type),
    }));
    input.sig.inputs = inputs;
    let mut unpack: Vec<(syn::Pat, syn::Type)> = vec![];
    for ident in idents {
        let ident = ident.ident;
        unpack.push((
            parse_quote! { #ident },
            parse_quote! { ::stringz::ident!(#ident) },
        ))
    }
    let (pat, ident_tys): (Vec<syn::Pat>, Vec<syn::Type>) = unpack.into_iter().unzip();
    let pat: syn::Pat = parse_quote! { ::tuplez::tuple_pat!( #( (_, #pat) ),* )};
    let ident_tys: syn::Type = parse_quote! { ::tuplez::tuple_t!( #( (#ident_tys, #tys) ),* )};
    let unpack: syn::Stmt = parse_quote! { let #pat: #ident_tys = structz_s; };
    input.block.stmts.insert(0, unpack);
    quote!(#input).into()
}
