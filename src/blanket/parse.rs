use syn::{
    parse::Parse, punctuated::Punctuated, token::Eq, Ident, Token, TypeParamBound, Visibility,
};

// blanket!(pub Safe = Send + Sync + 'static)

pub struct Blanket {
    pub visibility: Visibility,
    pub trait_name: Ident,
    pub bounds: Vec<TypeParamBound>,
}

impl Parse for Blanket {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility = input.parse()?;
        let trait_name = input.parse()?;
        let _eq: Eq = input.parse()?;
        let bounds = Punctuated::<TypeParamBound, Token![+]>::parse_terminated(input)?;

        Ok(Self {
            visibility,
            trait_name,
            bounds: bounds.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_str;

    use super::*;

    #[test]
    fn can_parse() {
        let blanket: Blanket = parse_str("pub Safe = Send + Sync").unwrap();
        assert_eq!(&blanket.visibility.into_token_stream().to_string(), "pub");
        assert_eq!(&blanket.trait_name.into_token_stream().to_string(), "Safe");
        assert_eq!(
            &blanket.bounds[0].clone().to_token_stream().to_string(),
            "Send"
        );
        assert_eq!(
            &blanket.bounds[1].clone().to_token_stream().to_string(),
            "Sync"
        );
    }

    #[test]
    fn can_parse_lifetime_bounds() {
        let _: Blanket = parse_str("pub Safe = 'static").unwrap();
    }
}
