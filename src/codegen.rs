use convert_case::Casing;
use proc_macro2::TokenStream;
use quote::quote;

use crate::parse::Single;

pub fn generate(
    Single {
        attributes,
        visibility,
        trait_name,
        struct_names,
        ..
    }: Single,
) -> TokenStream {
    let private = format!(
        "__{}_private",
        trait_name.to_string().to_case(convert_case::Case::Snake)
    );
    let private = syn::Ident::new(&private, trait_name.span());
    let private_trait = syn::Ident::new(
        &format!("{}Sealed", trait_name.to_string()),
        trait_name.span(),
    );

    let types = struct_names.iter().map(|ident| {
        let ty = quote! { #visibility enum #ident {} };
        quote! {
            #(#attributes)*
            #ty

            impl #trait_name for #ident {}
        }
    });
    let private_impls = struct_names.iter().map(|ident| {
        quote! {
            impl #private_trait for super::#ident {}
        }
    });

    quote! {
        #visibility trait #trait_name: #private::#private_trait {}

        #(#types)*

        mod #private {
            pub trait #private_trait {}
            #(#private_impls)*
        }
    }
}
