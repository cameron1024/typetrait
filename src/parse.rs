use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Attribute, Ident, Result, Token, Visibility,
};


pub struct Single {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub trait_name: Ident,
    pub eq: Token![=],
    pub struct_names: Vec<Ident>,
}

impl Parse for Single {
    fn parse(input: ParseStream) -> Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let visibility = input.parse()?;
        let trait_name = input.parse()?;
        let eq = input.parse()?;
        let struct_names = Punctuated::<Ident, Token![|]>::parse_terminated(input)?;

        Ok(Self {
            attributes,
            visibility,
            trait_name,
            eq,
            struct_names: struct_names.into_iter().collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;
    use syn::parse_str;

    use super::*;

    #[test]
    fn can_parse_single() {
        let single: Single = parse_str("#[foo] pub Foo = Bar | Baz").unwrap();
        assert_eq!(single.attributes.len(), 1);
        assert_eq!(single.visibility.to_token_stream().to_string(), "pub");
        assert_eq!(single.trait_name.to_string(), "Foo");
        assert_eq!(single.struct_names[0].to_string(), "Bar");
        assert_eq!(single.struct_names[1].to_string(), "Baz");
    }

    #[test]
    fn allows_trailing_pipe() {
        let _: Single = parse_str("Foo = Bar | ").unwrap();
    }
}
