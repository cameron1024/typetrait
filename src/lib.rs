use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::{codegen::generate, parse::Single};

mod codegen;
mod parse;

/// Generate boilerplate for a trait and unit structs to represent a set of possible states
///
/// ```
/// # fn main() {}
/// # use typetrait::union;
/// union! {
///   pub Status = Validated | Unvalidated
/// }
/// ```
/// This generates a trait `Status` and two unit structs `Validated` and `Unvalidated` that
/// implement `Status`. These types can then be used to provide a more type-safe API.
///
/// For example, consider a struct `Data` that represents some user input from a form:
/// ```
/// # fn main() {}
/// struct Data {
///     name: String,
///     age: u8,
/// }
/// ```
/// However, you might want to perform some validation on this struct before using it. To enforce
/// this, you can make it generic over `T: Status` to represent its validation status:
/// ```
/// # fn main() {}
/// # use typetrait::union;
/// union! {
///   pub Status = Validated | Unvalidated
/// }
///
/// struct Data<T: Status> {
///     _marker: std::marker::PhantomData<T>,
///     name: String,
///     age: u8,
/// }
/// ```
/// You might then have an API that looks like:
/// ```
/// # fn main() {}
/// # use typetrait::union;
/// # union! {
/// #   pub Status = Validated | Unvalidated
/// # }
/// # struct Data<T: Status> {
/// #     _marker: std::marker::PhantomData<T>,
/// #     name: String,
/// #     age: u8,
/// # }
/// fn get_data() -> Data<Unvalidated> {
///   todo!()
/// }
///
/// fn validate(data: Data<Unvalidated>) -> Data<Validated> {
///   todo!()
/// }
///
/// fn use_valid_data(data: Data<Validated>) {
///   todo!()
/// }
/// ```
/// With this API, using unvalidated data can now be prevented at compile time.
#[proc_macro]
pub fn union(tokens: TokenStream) -> TokenStream {
    let single = parse_macro_input!(tokens as Single);
    let tokens = generate(single);
    tokens.into()
}

#[cfg(test)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/pass/*.rs");
    t.compile_fail("tests/ui/fail/*.rs");
}
