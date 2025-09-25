use syn::{
    Expr, Path, Token,
    parse::{Parse, ParseStream},
};

pub enum MiddlewareKind {
    TowerLayer(Expr),
    Sword(SwordMiddlewareArgs),
}

pub struct SwordMiddlewareArgs {
    pub path: Path,
    pub config: Option<Expr>,
}

impl Parse for MiddlewareKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content = input.to_string();

        if content.contains("::") || (content.contains("(") && content.contains(")"))
        {
            let expr: Expr = input.parse()?;
            return Ok(MiddlewareKind::TowerLayer(expr));
        }

        if let Ok(sword_args) = input.parse::<SwordMiddlewareArgs>() {
            return Ok(MiddlewareKind::Sword(sword_args));
        }

        let expr: Expr = input.parse()?;

        Ok(MiddlewareKind::TowerLayer(expr))
    }
}

impl Parse for SwordMiddlewareArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: Path = input.parse()?;

        let mut config = None;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;

            if input.peek(syn::Ident) && input.peek2(Token![=]) {
                let ident: syn::Ident = input.parse()?;

                if ident != "config" {
                    return Err(syn::Error::new(ident.span(), "expected 'config'"));
                }

                input.parse::<Token![=]>()?;
                config = Some(input.parse()?);
            }
        }

        Ok(SwordMiddlewareArgs { path, config })
    }
}
