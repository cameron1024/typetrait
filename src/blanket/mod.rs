use proc_macro::TokenStream;
use syn::parse_macro_input;

use self::{parse::Blanket, codegen::generate};

mod parse;
mod codegen;

pub fn blanket_impl(tokens: TokenStream) -> TokenStream {
    let blanket = parse_macro_input!(tokens as Blanket);
    let tokens = generate(blanket);
    tokens.into()
}
