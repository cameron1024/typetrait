use proc_macro2::TokenStream;
use quote::quote;

use super::parse::Blanket;

pub fn generate(Blanket { visibility, trait_name, bounds }: Blanket) -> TokenStream {
    let bounds: Vec<_> = bounds.iter().map(|bound| quote! { #bound + }).collect();
    quote! {
        #visibility trait #trait_name: #(#bounds)* {}
        impl<BlanketTypeParameter> #trait_name for BlanketTypeParameter where BlanketTypeParameter: #(#bounds)* {}
        
    }
}
