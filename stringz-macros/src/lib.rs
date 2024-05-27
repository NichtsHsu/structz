use proc_macro::TokenStream;
use quote::quote;

struct ReExportStringz<T: syn::parse::Parse> {
    pub path: syn::Path,
    pub other: T,
}

impl<T: syn::parse::Parse> syn::parse::Parse for ReExportStringz<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let _: syn::Token![;] = input.parse()?;
        let other = input.parse()?;
        Ok(Self { path, other })
    }
}

#[proc_macro]
pub fn ident(input: TokenStream) -> TokenStream {
    let ReExportStringz { path, other: name } =
        syn::parse_macro_input!(input as ReExportStringz<syn::Ident>);
    let name = name.to_string();
    let name: Vec<_> = name
        .chars()
        .map(|ch| quote!( #path::Character<#ch> ))
        .collect();
    quote!( #path::__tuplez::tuple_t![ #( #name ),* ] ).into()
}

#[proc_macro]
pub fn string(input: TokenStream) -> TokenStream {
    let ReExportStringz { path, other: name } =
        syn::parse_macro_input!(input as ReExportStringz<syn::LitStr>);
    let name = name.value();
    let name: Vec<_> = name
        .chars()
        .map(|ch| quote!( #path::Character<#ch> ))
        .collect();
    quote!( #path::__tuplez::tuple_t![ #( #name ),* ] ).into()
}
