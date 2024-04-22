use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn ident(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as syn::Ident);
    let name = name.to_string();
    let name: Vec<_> = name
        .chars()
        .map(|ch| quote!( stringz::Character<#ch> ))
        .collect();
    quote!(tuplez::tuple_t![ #( #name ),* ]).into()
}

#[proc_macro]
pub fn string(input: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(input as syn::LitStr);
    let name = name.value();
    let name: Vec<_> = name
        .chars()
        .map(|ch| quote!( stringz::Character<#ch> ))
        .collect();
    quote!(tuplez::tuple_t![ #( #name ),* ]).into()
}
