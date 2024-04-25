use syn::{parse::Parse, Expr, Ident, Token, Type};

pub struct AnonymousStruct(pub Vec<(Ident, Option<Expr>)>);

impl Parse for AnonymousStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }
            let ident: Ident = input.parse()?;
            if fields.iter().any(|(i, _)| i == &ident) {
                return Err(syn::Error::new(ident.span(), "field already defined"));
            }
            if input.is_empty() {
                fields.push((ident, None));
                break;
            }
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
                fields.push((ident, None));
                continue;
            }
            if lookahead.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                let expr = input.parse()?;
                fields.push((ident, Some(expr)));
                if input.is_empty() {
                    break;
                }
                let _: Token![,] = input.parse()?;
            }
        }

        fields.sort_by(|x, y| x.0.cmp(&y.0));
        Ok(Self(fields))
    }
}

pub struct AnonymousStructType(pub Vec<(Ident, Type)>);

impl Parse for AnonymousStructType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }
            let ident: Ident = input.parse()?;
            if fields.iter().any(|(i, _)| i == &ident) {
                return Err(syn::Error::new(ident.span(), "field already defined"));
            }
            let _: Token![:] = input.parse()?;
            let ty = input.parse()?;
            fields.push((ident, ty));
            if input.is_empty() {
                break;
            }
            let _: Token![,] = input.parse()?;
        }

        fields.sort_by(|x, y| x.0.cmp(&y.0));
        Ok(Self(fields))
    }
}
