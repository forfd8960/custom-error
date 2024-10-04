use error_info::process_error_info;
use proc_macro::TokenStream;

mod error_info;

/// for enum, we'd like to generate From impls for each variant
#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error_info(_input: TokenStream) -> TokenStream {
    let derive_input = syn::parse_macro_input!(_input as syn::DeriveInput);
    println!("{:#?}", derive_input);

    process_error_info(derive_input).into()
}
