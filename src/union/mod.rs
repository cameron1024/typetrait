use proc_macro::TokenStream;
use syn::parse_macro_input;
use parse::Single;
use codegen::generate;

mod codegen;
mod parse;

pub fn union_impl(tokens: TokenStream) -> TokenStream {
    
    let single = parse_macro_input!(tokens as Single);
    let tokens = generate(single);
    tokens.into()
}
